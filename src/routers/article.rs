use actix_web::{get, post, put, delete, HttpResponse, Scope};
use actix_web::web::{Data, Json, Path, scope};
use crate::entities::article::CreateArticle;
use crate::entities::auth::AuthToken;
use crate::helpers::factory::Factory;
use crate::repositories::article::{ArticleRepo};
use crate::services::article::ArticleService;

type Service = ArticleService<ArticleRepo>;

pub fn article_scope(factory: &Factory) -> Scope {
    scope("/article")
        .app_data(Data::new(factory.article_service()))
        .service(get_articles)
        .service(create_article)
        .service(get_article_by_id)
        .service(update_article)
        .service(delete_article)
}

#[get("")]
async fn get_articles(
    service: Data<Service>
) -> HttpResponse {
    match service.get_articles() {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(error) => HttpResponse::from(error)
    }
}

#[post("")]
async fn create_article(
    service: Data<Service>,
    body: Json<CreateArticle>,
    token: AuthToken
) -> HttpResponse {
    println!("token: {}", token.id);

    match service.create_article(body.0) {
        Ok(article) => HttpResponse::Created().json(article),
        Err(error) => HttpResponse::from(error)
    }
}

#[get("/{id}")]
async fn get_article_by_id(
    service: Data<Service>,
    path: Path<i32>
) -> HttpResponse {
    match service.get_article_by_id(path.into_inner()) {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(error) => HttpResponse::from(error)
    }
}

#[put("/{id}")]
async fn update_article(
    service: Data<Service>,
    path: Path<i32>,
    body: Json<CreateArticle>
) -> HttpResponse {
    match service.update_article(path.into_inner(), body.0) {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(error) => HttpResponse::from(error)
    }
}

#[delete("/{id}")]
async fn delete_article(
    service: Data<Service>,
    path: Path<i32>
) -> HttpResponse {
    match service.delete_article(path.into_inner()) {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(error) => HttpResponse::from(error)
    }
}
