use actix_web::{get, HttpResponse, Scope};
use actix_web::web::{scope, Data, Path};
use crate::helpers::factory::Factory;
use crate::repositories::user::{UserRepo};
use crate::services::user::UserService;

type Service = UserService<UserRepo>;

pub fn get_user_scope(factory: &Factory) -> Scope {
    scope("/user")
        .app_data(Data::new(factory.user_service()))
        .service(get_user_by_id)
}

#[get("/{id}")]
async fn get_user_by_id(
    service: Data<Service>,
    path: Path<i32>
) -> HttpResponse {
    match service.get_user_by_id(path.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::from(error)
    }
}
