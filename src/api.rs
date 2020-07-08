use crate::errors::ServiceError;
use crate::{
    extractors::ClaimsFromAuth,
    models::{NewOccupancy, Occupancy, Room, User},
    DbPool,
};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Duration, DurationRound};
use diesel::prelude::*;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use ldap3::LdapConnAsync;
use sha2::Sha256;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    /// Expiration date as unix timestamp in seconds since epoch and UTC
    pub exp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub user_id: String,
    pub password: String,
}

fn create_signed_token(sub: &str) -> Result<String, ServiceError> {
    // Create the JWT token
    let key: Hmac<Sha256> = Hmac::new_varkey(env::var("JWT_SECRET")?.as_bytes())?;
    // Determine an expiration date based on the configuration
    let now: chrono::DateTime<_> = chrono::Utc::now();
    let exp: i64 = now
        .checked_add_signed(chrono::Duration::minutes(
            env::var("JWT_EXPIRATION")
                .ok()
                .as_deref()
                .unwrap_or("120")
                .parse::<i64>()?,
        ))
        .ok_or_else(|| {
            ServiceError::InternalServerError(
                "Could not add expiration time to current time".to_string(),
            )
        })?
        .timestamp();
    let claims = Claims {
        sub: sub.to_string(),
        exp: Some(exp),
    };
    // Create the actual token
    let token_str = claims.sign_with_key(&key)?;
    Ok(token_str)
}

pub async fn login(
    login_data: web::Json<LoginData>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    // Get the corresponding user entry from the database
    use crate::schema::users::dsl::*;
    let conn = db_pool.get()?;
    let u: Vec<User> = users
        .filter(id.eq(&login_data.user_id))
        .load::<User>(&conn)?;
    if let Some(u) = u.first() {
        if let Some(actual_hash) = &u.password_hash {
            // Compare provided password with actual hash
            let verified = bcrypt::verify(&login_data.password, actual_hash)?;
            if verified {
                let token_str = create_signed_token(&u.id)?;
                return Ok(HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(token_str));
            }
        } else {
            // Query LDAP if the credentials are correct
            let ldap_settings = ldap3::LdapConnSettings::new();
            let (conn, mut ldap) =
                LdapConnAsync::with_settings(ldap_settings, "ldaps://ldapmaster.cms.hu-berlin.de")
                    .await?;

            ldap3::drive!(conn);

            let result = ldap
                .simple_bind(
                    &format!(
                        "uid={},ou=users,ou=Benutzerverwaltung,ou=Computer- und Medienservice,o=Humboldt-Universitaet zu Berlin,c=DE",
                        &u.id
                    ),
                    &login_data.password,
                )
                .await?;
            if result.rc == 0 {
                let token_str = create_signed_token(&u.id)?;
                return Ok(HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(token_str));
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
    let rooms: Vec<Room> = rooms::dsl::rooms.load(&conn)?;

    Ok(HttpResponse::Ok().json(rooms))
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

    // Get the general room capacity
    use crate::schema::rooms;
    let room: Option<Room> = rooms::dsl::rooms
        .filter(rooms::dsl::id.eq(room.as_ref()))
        .load(&conn)?
        .into_iter()
        .next();

    if let Some(room) = room {
        use crate::schema::occupancies::dsl;
        // Check the number of persons per partially overlapped full hour
        let mut t = start.clone();
        while t <= end {
            let t_next = t + Duration::hours(1);
            let overlapping_existing: Vec<_> = dsl::occupancies
                .filter(dsl::room.eq(&room.id))
                .filter(dsl::start.lt(t_next.naive_utc()))
                .filter(dsl::end.gt(t.naive_utc()))
                .load::<Occupancy>(&conn)?;

            if overlapping_existing.len() as i32 >= room.max_occupancy {
                debug!(
                    "Too many overlapping existing events for room {} and new time {}-{}: {:?}",
                    &room.id, t, t_next, overlapping_existing
                );
                return Ok(HttpResponse::Forbidden().json("Room already full"));
            }

            t = t_next;
        }

        // Check was successful, add the new event
        let new_item = NewOccupancy {
            room: room.id,
            user: claims.0.sub.to_string(),
            start: start.naive_utc(),
            end: end.naive_utc(),
        };
        diesel::insert_into(crate::schema::occupancies::table)
            .values(new_item.clone())
            .execute(&conn)?;

        Ok(HttpResponse::Ok().json(new_item))
    } else {
        Ok(HttpResponse::NotFound().json("Room not found"))
    }
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
    _claims: ClaimsFromAuth,
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
        let result = if let (Some(start), Some(end)) = (start, end) {
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

        Ok(HttpResponse::Ok().json(result))
    } else {
        Ok(HttpResponse::NotFound().json("Room not found"))
    }
}
