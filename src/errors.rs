use actix_http::ResponseBuilder;
use actix_web::{error::ResponseError, http::header, http::StatusCode, HttpResponse};
use diesel::result::Error as DBError;
use failure::Fail;
use serde::{Deserialize, Serialize};

use std::convert::From;

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "some characters are not permitted")] //405j
    CharError,
    #[fail(display = "username exists")] //405
    UsernameExists,
    #[fail(display = "invalid credentials")]
    AuthorizationRequired,
    #[fail(display = "internal error")] // 500
    InternalServerError,
    #[fail(display = "timeout")] //408
    Timeout,
    #[fail(display = "bad request")] //400
    BadRequest,
    #[fail(display = "Unable to connect to DB")]
    UnableToConnectToDb,
}

#[derive(Serialize, Deserialize)]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::CharError => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::UsernameExists => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::AuthorizationRequired => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ServiceError::UnableToConnectToDb => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(_kind, _info) => ServiceError::BadRequest,
            _ => ServiceError::InternalServerError,
        }
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
