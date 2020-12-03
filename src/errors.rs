/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix_http::ResponseBuilder;
use actix_web::{error::ResponseError, http::header, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
use tokio_postgres::error::SqlState as PGBError;
use validator::ValidationErrors;

use std::convert::From;

#[derive(Debug, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[error("some characters are not permitted")] //405j
    UsernameError,
    #[error("username exists")] //405
    UsernameExists,
    #[error("invalid credentials")]
    AuthorizationRequired,
    #[error("internal error")] // 500
    InternalServerError,
    #[error("bad request")] //400
    BadRequest,
    #[error("PoW required, request not processed")]
    PoWRequired,
    #[error("PoW submitted is incorrect")]
    PoWInvalid,
    #[error("The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[error("Account Doesn't exist")]
    AccountDoesntExist,
}

#[derive(Serialize, Deserialize)]
#[cfg(not(tarpaulin_include))]
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
            ServiceError::UsernameError => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::UsernameExists => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::AuthorizationRequired => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::PoWRequired => StatusCode::PAYMENT_REQUIRED,
            ServiceError::PoWInvalid => StatusCode::BAD_REQUEST,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
            ServiceError::AccountDoesntExist => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<PGMError> for ServiceError {
    fn from(_: PGMError) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<PGBError> for ServiceError {
    fn from(error: PGBError) -> ServiceError {
        if error == PGBError::UNIQUE_VIOLATION {
            ServiceError::UsernameExists
        } else {
            ServiceError::InternalServerError
        }
    }
}

impl From<PGError> for ServiceError {
    fn from(error: PGError) -> ServiceError {
        if let Some(e) = error.code() {
            if e == &PGBError::UNIQUE_VIOLATION {
                ServiceError::UsernameExists
            } else {
                ServiceError::InternalServerError
            }
        } else {
            ServiceError::InternalServerError
        }
    }
}

impl From<actix_http::Error> for ServiceError {
    fn from(_: actix_http::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<deadpool_postgres::PoolError> for ServiceError {
    fn from(_: deadpool_postgres::PoolError) -> ServiceError {
        ServiceError::InternalServerError
    }
}

//impl From<deadpool_postgres::PoolError<PGError>> for ServiceError {
//    fn from(error: deadpool_postgres::PoolError<PGError>) -> ServiceError {
//        ServiceError::InternalServerError
//    }
//}

impl From<argon2::Error> for ServiceError {
    fn from(_: argon2::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<ValidationErrors> for ServiceError {
    fn from(_: ValidationErrors) -> ServiceError {
        ServiceError::NotAnEmail
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
