use crate::{api::Claims, errors::ServiceError};
use actix_web::FromRequest;
use futures::future::{err, ok, Ready};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::env;

#[derive(Debug, Clone)]
pub struct ClaimsFromAuth(pub Claims);

fn verify_token(token: &str) -> Result<Claims, ServiceError> {
    let key = Hmac::<Sha256>::new_varkey(env::var("JWT_SECRET")?.as_bytes())?;
    let claims = VerifyWithKey::<Claims>::verify_with_key(token, &key)?;
    let claim_still_valid = if let Some(exp) = claims.exp {
        // Check that the claim is still valid, thus not expired
        let expiration_date = chrono::NaiveDateTime::from_timestamp(exp, 0);
        let current_date = chrono::Utc::now();
        current_date.naive_utc() < expiration_date
    } else {
        true
    };

    if claim_still_valid {
        Ok(claims)
    } else {
        Err(ServiceError::InvalidJWTToken(
            "Token is expired".to_string(),
        ))
    }
}

impl FromRequest for ClaimsFromAuth {
    type Error = ServiceError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if let Some(authen_header) = req.headers().get("Authorization") {
            // Parse header
            if let Ok(authen_str) = authen_header.to_str() {
                if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                    // Parse and verify token
                    let token = authen_str[6..authen_str.len()].trim();
                    return match verify_token(token) {
                        // Use the verified claim
                        Ok(claim) => ok(ClaimsFromAuth(claim)),
                        // If a token was given but invalid, report an error
                        Err(e) => err(e),
                    };
                }
            }
        }
        err(ServiceError::InvalidJWTToken("".to_string()))
    }
}
