extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;

use controllers;
pub fn load_router() -> Router {
    router!(get "hello/:name" => controllers::hello::hello,
            get "json/"       => controllers::json::parse)
}
