use iron::middleware::{Chain};
use iron::typemap::Key;
use persistent::{Read};
use r2d2::{Config, PooledConnection, Pool};
use r2d2_postgres::{SslMode, PostgresConnectionManager};

// Aliased types
pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

/// Acts as a key type for the persistent middleware.
/// This will allow us to extract the db pool for each
/// request.
pub struct DbPool;
impl Key for DbPool {
    type Value = PostgresPool;
}

pub fn build() -> Read<DbPool> {
    let config = Config::builder().pool_size(8).build();
    let manager = PostgresConnectionManager::new("postgres://postgres@db:5432",
                                                 SslMode::None).unwrap();
    let pool = Pool::new(config, manager).unwrap();
    Read::<DbPool>::one(pool)
}
