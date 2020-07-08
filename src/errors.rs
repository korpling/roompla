use actix_web::{error::ResponseError, HttpResponse};
use hmac::crypto_mac::InvalidKeyLength;
use std::fmt::Display;

#[derive(Debug)]
pub enum ServiceError {
    BadRequest(String),
    InvalidJWTToken(String),
    DatabaseError(String),
    InternalServerError(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::BadRequest(msg) => write!(f, "Bad Request: {}", msg)?,
            ServiceError::InvalidJWTToken(msg) => write!(f, "Invalid JWT Token: {}", msg)?,
            ServiceError::DatabaseError(e) => write!(f, "Error accessing database: {}", e)?,
            ServiceError::InternalServerError(msg) => {
                write!(f, "Internal Server Error: {:?}", msg)?
            }
        }
        Ok(())
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::InvalidJWTToken(ref message) => {
                HttpResponse::Unauthorized().json(message)
            }

            ServiceError::DatabaseError(_) => {
                HttpResponse::BadGateway().json("Error accessing database")
            }
            ServiceError::InternalServerError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(orig: anyhow::Error) -> Self {
        ServiceError::InternalServerError(orig.to_string())
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(e: std::io::Error) -> Self {
        ServiceError::InternalServerError(e.to_string())
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(orig: diesel::result::Error) -> Self {
        ServiceError::DatabaseError(orig.to_string())
    }
}

impl From<r2d2::Error> for ServiceError {
    fn from(orig: r2d2::Error) -> Self {
        ServiceError::DatabaseError(orig.to_string())
    }
}

impl From<bcrypt::BcryptError> for ServiceError {
    fn from(e: bcrypt::BcryptError) -> Self {
        ServiceError::InternalServerError(format!("{}", e))
    }
}

impl From<std::env::VarError> for ServiceError {
    fn from(e: std::env::VarError) -> Self {
        ServiceError::InternalServerError(format!("{}", e))
    }
}

impl From<std::num::ParseIntError> for ServiceError {
    fn from(e: std::num::ParseIntError) -> Self {
        ServiceError::InternalServerError(format!("{}", e))
    }
}

impl From<InvalidKeyLength> for ServiceError {
    fn from(_: InvalidKeyLength) -> Self {
        ServiceError::BadRequest("Invalid JWT key length".to_owned())
    }
}

impl From<dotenv::Error> for ServiceError {
    fn from(e: dotenv::Error) -> Self {
        ServiceError::InternalServerError(format!("{}", e))
    }
}

impl From<ldap3::result::LdapError> for ServiceError {
    fn from(e: ldap3::result::LdapError) -> Self {
        match e {
            ldap3::result::LdapError::OpSend { ref source } => {
                ServiceError::InternalServerError(format!("{} (source: {})", e, source))
            }
            _ => ServiceError::InternalServerError(format!("{}", e)),
        }
    }
}

impl From<jwt::Error> for ServiceError {
    fn from(orig: jwt::Error) -> Self {
        ServiceError::InvalidJWTToken(format!("{}", orig))
    }
}
