use iron::prelude::*;
use router::Router;

use controllers;

/// Build the routes for this server.
pub fn load_router() -> Router {
    router!(get   "hello/:name" => controllers::hello::hello,
            post  "json"        => controllers::json::parse,
            post  "persons"     => controllers::person::create,
            get   "persons/:id" => controllers::person::show)
}
