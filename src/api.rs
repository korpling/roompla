use crate::errors::ServiceError;
use crate::{
    config::Settings,
    extractors::ClaimsFromAuth,
    models::{NewOccupancy, Occupancy, Room, User},
    DbPool,
};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Duration, DurationRound, TimeZone};
use diesel::prelude::*;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use ldap3::{LdapConnAsync, Scope, SearchEntry};
use sha2::Sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub contact_info: String,
    /// Expiration date as unix timestamp in seconds since epoch and UTC
    pub exp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub user_id: String,
    pub password: String,
}

fn create_signed_token(
    sub: &str,
    name: &str,
    contact_info: &str,
    settings: &Settings,
) -> Result<String, ServiceError> {
    // Create the JWT token
    let key: Hmac<Sha256> = Hmac::new_varkey(
        settings
            .jwt
            .secret
            .as_deref()
            .unwrap_or_default()
            .as_bytes(),
    )?;
    // Determine an expiration date based on the configuration
    let now: chrono::DateTime<_> = chrono::Utc::now();
    let exp = if let Some(exp_config) = settings.jwt.expiration {
        let exp = now
            .checked_add_signed(chrono::Duration::minutes(exp_config))
            .ok_or_else(|| {
                ServiceError::InternalServerError(
                    "Could not add expiration time to current time".to_string(),
                )
            })?
            .timestamp();
        Some(exp)
    } else {
        None
    };
    let claims = Claims {
        sub: sub.to_string(),
        exp: exp,
        name: name.to_string(),
        contact_info: contact_info.to_string(),
    };
    // Create the actual token
    let token_str = claims.sign_with_key(&key)?;
    Ok(token_str)
}

pub async fn login(
    login_data: web::Json<LoginData>,
    db_pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
) -> Result<HttpResponse, ServiceError> {
    // Get the corresponding user entry from the database
    use crate::schema::users::dsl::*;
    let conn = db_pool.get()?;
    let u: Vec<User> = users
        .filter(id.eq(&login_data.user_id))
        .load::<User>(&conn)?;
    if let Some(u) = u.first() {
        // The user is explicitly configured, use the stored password hash for authentification
        if let Some(actual_hash) = &u.password_hash {
            // Compare provided password with actual hash
            let verified = bcrypt::verify(&login_data.password, actual_hash)?;
            if verified {
                let token_str = create_signed_token(
                    &u.id,
                    &u.display_name,
                    &u.contact_info,
                    settings.as_ref(),
                )?;
                return Ok(HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(token_str));
            }
        }
    } else {
        // Query LDAP if the credentials are correct
        let ldap_settings = ldap3::LdapConnSettings::new();
        let (conn, mut ldap) =
            LdapConnAsync::with_settings(ldap_settings, &settings.ldap.url).await?;

        ldap3::drive!(conn);

        let user_query = format!("uid={},{}", &login_data.user_id, settings.ldap.organization);
        let result = ldap.simple_bind(&user_query, &login_data.password).await?;
        if result.rc == 0 {
            // Gather additional information from LDAP, only accept users that are also matching the filter from the configuration
            let (user_attributes, _) = ldap
                .search(
                    &user_query,
                    Scope::Subtree,
                    &settings.ldap.filter,
                    vec!["cn", "publicEMailAddress"],
                )
                .await?
                .success()?;
            if let Some(user_attributes) = user_attributes.into_iter().next() {
                let user_attributes = SearchEntry::construct(user_attributes);

                if let (Some(cn), Some(email)) = (
                    user_attributes.attrs.get("cn"),
                    user_attributes.attrs.get("publicEMailAddress"),
                ) {
                    if !cn.is_empty() && !email.is_empty() {
                        let token_str = create_signed_token(
                            &login_data.user_id,
                            &cn[0],
                            &email[0],
                            &settings.as_ref(),
                        )?;
                        return Ok(HttpResponse::Ok()
                            .content_type("text/plain")
                            .body(token_str));
                    }
                }
            }
        }
    }
    Ok(HttpResponse::Unauthorized().finish())
}

#[derive(Deserialize)]
pub struct TimeRange {
    pub start: String,
    pub end: String,
}

pub async fn all_rooms(
    db_pool: web::Data<DbPool>,
    _claims: ClaimsFromAuth,
) -> Result<HttpResponse, ServiceError> {
    let conn = db_pool.get()?;

    // Get the general room capacity
    use crate::schema::rooms;
    let rooms: Vec<Room> = rooms::dsl::rooms.order(rooms::dsl::id).load(&conn)?;

    Ok(HttpResponse::Ok().json(rooms))
}

fn check_if_room_available<Conn: Connection<Backend = diesel::sqlite::Sqlite>, Tz: TimeZone>(
    conn: &Conn,
    room: &Room,
    start: DateTime<Tz>,
    end: DateTime<Tz>,
    ignore_id: Option<i32>,
) -> Result<bool, ServiceError> {
    use crate::schema::occupancies::dsl;
    // Check the number of persons per partially overlapped full hour
    let mut t = start.clone();
    while t <= end {
        let t_next = t.clone() + Duration::hours(1);
        let basic_overlap_query = dsl::occupancies
            .filter(dsl::room.eq(&room.id))
            .filter(dsl::start.lt(t_next.naive_utc()))
            .filter(dsl::end.gt(t.naive_utc()));

        let overlapping_existing: Vec<_> = if let Some(ignore_id) = ignore_id {
            basic_overlap_query
                .filter(dsl::id.ne(ignore_id))
                .load::<Occupancy>(conn)?
        } else {
            basic_overlap_query.load::<Occupancy>(conn)?
        };

        if overlapping_existing.len() as i32 >= room.max_occupancy {
            return Ok(false);
        }
        t = t_next;
    }
    Ok(true)
}

pub async fn add_occupancy(
    event: web::Json<TimeRange>,
    room: web::Path<String>,
    db_pool: web::Data<DbPool>,
    claims: ClaimsFromAuth,
) -> Result<HttpResponse, ServiceError> {
    // Parse the dates and round them to the full hour
    let start = DateTime::parse_from_rfc3339(&event.start)?.duration_round(Duration::hours(1))?;
    let end = DateTime::parse_from_rfc3339(&event.end)?.duration_round(Duration::hours(1))?;

    if end <= start {
        return Ok(HttpResponse::Forbidden().json(format!(
            "Begin of time range ({}) ist after end of range ({}).",
            start, end
        )));
    }

    let conn = db_pool.get()?;
    let result = conn.transaction::<_, ServiceError, _>(|| {
        // Get the general room capacity
        use crate::schema::rooms;
        let room: Option<Room> = rooms::dsl::rooms
            .filter(rooms::dsl::id.eq(room.as_ref()))
            .order(rooms::dsl::id)
            .load(&conn)?
            .into_iter()
            .next();
        if let Some(room) = room {
            if check_if_room_available(&conn, &room, start.clone(), end.clone(), None)? {
                // Check was successful, add the new event
                let new_item = NewOccupancy {
                    room: room.id,
                    user_id: claims.0.sub.to_string(),
                    user_name: claims.0.name.to_string(),
                    user_contact: claims.0.contact_info.to_string(),
                    start: start.naive_utc(),
                    end: end.naive_utc(),
                };
                diesel::insert_into(crate::schema::occupancies::table)
                    .values(new_item.clone())
                    .execute(&conn)?;

                Ok(HttpResponse::Ok().json(new_item))
            } else {
                return Ok(HttpResponse::Conflict().json("Room already full"));
            }
        } else {
            Ok(HttpResponse::NotFound().json("Room not found"))
        }
    })?;
    Ok(result)
}

pub async fn update_occupancy(
    path: web::Path<(String, i32)>,
    event: web::Json<TimeRange>,
    db_pool: web::Data<DbPool>,
    claims: ClaimsFromAuth,
) -> Result<HttpResponse, ServiceError> {
    let conn = db_pool.get()?;
    let result = conn.transaction::<_, ServiceError, _>(|| {
        use crate::schema::occupancies;
        use crate::schema::rooms;

        // Parse the dates and round them to the full hour
        let start =
            DateTime::parse_from_rfc3339(&event.start)?.duration_round(Duration::hours(1))?;
        let end = DateTime::parse_from_rfc3339(&event.end)?.duration_round(Duration::hours(1))?;

        let room: Option<Room> = rooms::dsl::rooms
            .filter(rooms::dsl::id.eq(path.0.as_str()))
            .load(&conn)?
            .into_iter()
            .next();
        if let Some(room) = room {
            // Check if this event would lead to an invalid state
            if check_if_room_available(&conn, &room, start, end, Some(path.1))? {
                // Only update if this event is from the same user
                diesel::update(occupancies::dsl::occupancies)
                    .set((
                        occupancies::dsl::start.eq(start.naive_utc()),
                        occupancies::dsl::end.eq(end.naive_utc()),
                    ))
                    .filter(occupancies::dsl::id.eq(path.1))
                    .filter(occupancies::dsl::room.eq(path.0.as_str()))
                    .filter(occupancies::dsl::user_id.eq(claims.0.sub))
                    .execute(&conn)?;
                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::Conflict().json("Room already full"))
            }
        } else {
            Ok(HttpResponse::NotFound().json("Room not found"))
        }
    })?;
    Ok(result)
}

pub async fn delete_occupancy(
    path: web::Path<(String, i32)>,
    db_pool: web::Data<DbPool>,
    claims: ClaimsFromAuth,
) -> Result<HttpResponse, ServiceError> {
    let conn = db_pool.get()?;
    let result = conn.transaction::<_, ServiceError, _>(|| {
        use crate::schema::occupancies;

        // Only delete if this event is from the same user
        diesel::delete(
            occupancies::dsl::occupancies
                .filter(occupancies::dsl::id.eq(path.1))
                .filter(occupancies::dsl::room.eq(path.0.as_str()))
                .filter(occupancies::dsl::user_id.eq(claims.0.sub)),
        )
        .execute(&conn)?;

        Ok(HttpResponse::Ok().finish())
    })?;
    Ok(result)
}

#[derive(Deserialize, Clone)]
pub struct RoomOccupancyParams {
    pub start: Option<String>,
    pub end: Option<String>,
}

pub async fn get_occupancies(
    params: web::Query<RoomOccupancyParams>,
    room: web::Path<String>,
    db_pool: web::Data<DbPool>,
    claims: ClaimsFromAuth,
) -> Result<HttpResponse, ServiceError> {
    // Parse the dates and round them to the full hour
    let start = if let Some(start) = &params.start {
        Some(DateTime::parse_from_rfc3339(start)?.duration_round(Duration::hours(1))?)
    } else {
        None
    };
    let end = if let Some(end) = &params.end {
        Some(DateTime::parse_from_rfc3339(end)?.duration_round(Duration::hours(1))?)
    } else {
        None
    };

    if let (Some(start), Some(end)) = (start, end) {
        if end <= start {
            return Ok(HttpResponse::Forbidden().json(format!(
                "Begin of time range ({}) ist after end of range ({}).",
                start, end
            )));
        }
    }

    let conn = db_pool.get()?;

    // Get the general room capacity
    use crate::schema::rooms;
    let room: Option<Room> = rooms::dsl::rooms
        .filter(rooms::dsl::id.eq(room.as_ref()))
        .load(&conn)?
        .into_iter()
        .next();

    if let Some(room) = room {
        use crate::schema::occupancies::dsl;
        // Get all occupancy events for the given range
        let mut result = if let (Some(start), Some(end)) = (start, end) {
            dsl::occupancies
                .filter(dsl::room.eq(&room.id))
                .filter(dsl::start.ge(start.naive_utc()))
                .filter(dsl::end.le(end.naive_utc()))
                .load::<Occupancy>(&conn)?
        } else if let Some(start) = start {
            dsl::occupancies
                .filter(dsl::room.eq(&room.id))
                .filter(dsl::start.ge(start.naive_utc()))
                .load::<Occupancy>(&conn)?
        } else if let Some(end) = end {
            dsl::occupancies
                .filter(dsl::room.eq(&room.id))
                .filter(dsl::end.le(end.naive_utc()))
                .load::<Occupancy>(&conn)?
        } else {
            dsl::occupancies
                .filter(dsl::room.eq(&room.id))
                .load::<Occupancy>(&conn)?
        };

        // anonymize all occupancy entries for other users
        for mut o in result.iter_mut() {
            if o.user_id != claims.0.sub {
                o.user_id = "<anonym>".to_string();
                o.user_contact = "".to_string();
                o.user_name = "<anonym>".to_string();
            }
        }

        Ok(HttpResponse::Ok().json(result))
    } else {
        Ok(HttpResponse::NotFound().json("Room not found"))
    }
}
