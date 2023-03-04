mod routers;
mod entities;
mod services;
mod repositories;
mod helpers;
mod schema;

use dotenv::dotenv;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use crate::helpers::config::{Secrets, ServerConfig};
use crate::helpers::factory::Factory;
use crate::repositories::{create_pool, Pool};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let server_config = ServerConfig::from_env();
    let addrs = format!("{}:{}", server_config.host, server_config.port);

    std::env::set_var("RUST_LOG", server_config.rust_log);
    env_logger::init();

    let secrets = Secrets::from_env();
    let pool: Box<Pool> = Box::new(create_pool(
        secrets.database_url.clone(),
        server_config.pool_size
    ));

    let factory: Factory = Factory::new(pool, secrets.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(secrets.clone()))
            .service(routers::article::article_scope(&factory))
            .service(routers::user::get_user_scope(&factory))
            .service(routers::auth::auth_scope(&factory))

    }).bind(addrs)?
        .run()
        .await
}
