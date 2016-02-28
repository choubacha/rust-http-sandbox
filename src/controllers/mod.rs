pub mod hello;
pub mod json;
pub mod person;

use iron::prelude::*;
use router::Router;

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
