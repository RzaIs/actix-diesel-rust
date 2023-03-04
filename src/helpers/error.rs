use std::fmt::{Debug, Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use r2d2::Error as R2D2E;
use uuid::{Uuid};
use diesel::result::Error as DError;
use serde::Serialize;
use argon2::password_hash;

#[derive(Serialize)]
pub struct Error {
    pub uuid: String,
    pub code: u16,
    pub message: String,
}

impl Error {
    pub fn login() -> Self {
        Error {
            uuid: Uuid::new_v4().to_string(),
            code: 401,
            message: "username and password does not match"
                .to_string()
        }
    }

    pub fn hash(step: i8) -> Self {
        Error {
            uuid: Uuid::new_v4().to_string(),
            code: 500,
            message: format!("password encryption error (step-{})", step)
                .to_string()
        }
    }

    pub fn invalid_token() -> Self {
        Error {
            uuid: Uuid::new_v4().to_string(),
            code: 401,
            message: "invalid auth token"
                .to_string()
        }
    }

    pub fn tokens() -> Self {
        Error {
            uuid: Uuid::new_v4().to_string(),
            code: 500,
            message: "jwt generation error"
                .to_string()
        }
    }

    pub fn no_token() -> Self {
        Error {
            uuid: Uuid::new_v4().to_string(),
            code: 401,
            message: "auth token missing"
                .to_string()
        }
    }

    pub fn no_secret() -> Self {
        Error {
            uuid: Uuid::new_v4().to_string(),
            code: 500,
            message: "jwt secret missing in server"
                .to_string()
        }
    }
}

impl Debug for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        println!(
            "Error(code: {}, message: {}, uuid: {})",
            self.code, self.message, self.uuid
        );
        Ok(())
    }
}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        println!(
            "Error(code: {}, message: {}, uuid: {})",
            self.code, self.message, self.uuid
        );
        Ok(())
    }
}

impl ResponseError for Error { }

impl From<Error> for HttpResponse {
    fn from(error: Error) -> Self {
        HttpResponse::build(
            StatusCode::from_u16(error.code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        ).json(error)
    }
}

impl From<R2D2E> for Error {
    fn from(error: R2D2E) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let code = 504;
        let message = error.to_string();
        Self { uuid, code, message }
    }
}

impl From<password_hash::Error> for Error {
    fn from(error: password_hash::Error) -> Self {
        Error {
            uuid: Uuid::new_v4().to_string(),
            code: 500,
            message: error.to_string()
        }
    }
}

impl From<DError> for Error {
    fn from(error: DError) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let code: u16;
        let message: String;
        match error {
            DError::InvalidCString(e) => {
                code = 500;
                message = String::from(
                    format!("InvalidCString: {}", e.to_string())
                );
            }
            DError::DatabaseError(_, info) => {
                code = 500;
                message = String::from(info.message());
            }
            DError::NotFound => {
                code = 404;
                message = String::from("NotFound");
            }
            DError::QueryBuilderError(e) => {
                code = 400;
                message = String::from(e.to_string());
            }
            DError::DeserializationError(e) => {
                code = 400;
                message = String::from(e.to_string());
            }
            DError::SerializationError(e) => {
                code = 400;
                message = String::from(e.to_string());
            }
            DError::RollbackErrorOnCommit { .. } => {
                code = 500;
                message = String::from("RollbackErrorOnCommit")
            }
            DError::RollbackTransaction => {
                code = 500;
                message = String::from("RollbackTransaction")
            }
            DError::AlreadyInTransaction => {
                code = 500;
                message = String::from("AlreadyInTransaction")
            }
            DError::NotInTransaction => {
                code = 500;
                message = String::from("NotInTransaction")
            }
            DError::BrokenTransactionManager => {
                code = 500;
                message = String::from("BrokenTransactionManager")
            }
            _ => {
                code = 500;
                message = String::from("null")
            }
        };
        Self { uuid, code, message }
    }
}
