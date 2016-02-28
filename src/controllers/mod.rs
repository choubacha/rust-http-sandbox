pub mod hello;
pub mod json;
pub mod person;

use iron::prelude::*;
use router::Router;
use middlewares::db_pool::{PostgresPooledConnection, DbPool};

pub fn extract_param(req: &Request, param: &str) -> String {
    match req.extensions.get::<Router>() {
        Some(router) => {
            match router.find(param) {
                Some(value) => value.to_string(),
                None => String::new()
            }
        },
        None => String::new()
    }
}

pub fn with_conn<F, T>(req: &mut Request, closure: F) -> Result<T, String>
    where F: Fn(PostgresPooledConnection) -> Result<T, String>
{
    match req.get::<::persistent::Read<DbPool>>() {
        Ok(pool) => {
            match pool.get() {
                Ok(conn) => closure(conn),
                Err(err) => Err(format!("Could not get a db conn: {:?}", err)),
            }
        },
        Err(err) => Err(String::from("Plugin not available"))
    }
}
