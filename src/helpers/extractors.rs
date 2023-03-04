use std::future::{ready, Ready};
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use actix_web::{http, dev::Payload, FromRequest, HttpRequest, error::Error as ActixError};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use crate::entities::auth::{AuthToken, Claims};
use crate::helpers::config::Secrets;
use crate::helpers::error::Error as AppError;

impl FromRequest for AuthToken {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        match from_req_result(req, payload) {
            Ok(auth_token) => ready(Ok(auth_token)),
            Err(error) => ready(Err(error))
        }
    }
}

fn from_req_result(req: &HttpRequest, _: &mut Payload) -> Result<AuthToken, ActixError> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(ErrorUnauthorized(AppError::no_token()))?;

    let auth_token = auth_header.to_str()
        .map_err(|_| ErrorUnauthorized(AppError::no_token()))?;

    let secret = &req.app_data::<Data<Secrets>>()
        .ok_or(ErrorInternalServerError(AppError::no_token()))?
        .jwt_access_secret;

    let claims = decode::<Claims>(
        &auth_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256)
    ).map_err(|_| ErrorUnauthorized(AppError::invalid_token()))?
        .claims;

    Ok(AuthToken { id: claims.id })
}
