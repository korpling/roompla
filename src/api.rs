use crate::errors::ServiceError;
use crate::{models::User, DbPool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use ldap3::{LdapConnAsync, Scope, SearchEntry};
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
            env::var("JWT_EXPIRATION")?.parse::<i64>()?,
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
    dotenv()?;

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
            let (_, mut ldap) = LdapConnAsync::new("ldaps://ldapmaster.cms.hu-berlin.de").await?;

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
