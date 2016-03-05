pub mod hello;
pub mod json;
pub mod person;

use core::str::FromStr;
use iron::prelude::*;
use middlewares::db_pool::{PostgresPooledConnection, DbPool};
use router::Router;
use rustc_serialize::Decodable;
use rustc_serialize::json as json_lib;
use std::io::Read;

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

pub fn extract_or_param<T>(req: &Request, param: &str, default: T) -> T
    where T: FromStr
{
    let param = extract_param(&req, &param);
    param.parse::<T>().unwrap_or(default)
}

pub fn read_body(req: &mut Request) -> Option<String> {
    let mut buf: String = String::new();
    match req.body.read_to_string(&mut buf) {
        Ok(_)   => Some(buf),
        Err(_)  => None
    }
}

pub fn parse_body<T>(req: &mut Request) -> Result<T, String>
    where T: Decodable
{
    match read_body(req) {
        Some(body) => {
            match json_lib::decode::<T>(&body) {
                Ok(decoded) => Ok(decoded),
                Err(err)    => Err(format!("Could not parse json: {:?}", err))
            }
        },
        None => Err(String::from("Body is missing"))
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
        Err(_) => Err(String::from("Plugin not available"))
    }
}
