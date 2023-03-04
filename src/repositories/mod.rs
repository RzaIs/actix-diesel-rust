pub mod article;
pub mod user;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Conn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool(db_url: String, pool_size: u32) -> Pool {
    Pool::builder()
        .max_size(pool_size)
        .build(ConnectionManager::<PgConnection>::new(db_url))
        .expect("Connection pool to postgres cannot be created")
}
