use actix_web::{HttpResponse, post, Scope};
use actix_web::web::{Data, Json, scope};
use crate::entities::auth::{LoginDTO, RegisterDTO};
use crate::helpers::factory::Factory;
use crate::repositories::user::UserRepo;
use crate::services::auth::AuthService;

type Service = AuthService<UserRepo>;

pub fn auth_scope(factory: &Factory) -> Scope {
    scope("/auth")
        .app_data(Data::new(factory.auth_service()))
        .service(register)
        .service(login)
}

#[post("/register")]
async fn register(
    service: Data<Service>,
    body: Json<RegisterDTO>,
) -> HttpResponse {
    match service.register(body.0) {
        Ok(auth) => HttpResponse::Created().json(auth),
        Err(error) => HttpResponse::from(error)
    }
}

#[post("/login")]
async fn login(
    service: Data<Service>,
    body: Json<LoginDTO>,
) -> HttpResponse {
    match service.login(body.0) {
        Ok(auth) => HttpResponse::Ok().json(auth),
        Err(error) => HttpResponse::from(error)
    }
}
