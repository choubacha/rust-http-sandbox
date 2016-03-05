use iron::prelude::*;
use router::Router;

use controllers;

/// Build the routes for this server.
pub fn load_router() -> Router {
    router!(get     "hello/:name" => controllers::hello::hello,
            post    "json"        => controllers::json::parse,
            post    "persons"     => controllers::person::create,
            get     "persons"     => controllers::person::index,
            delete  "persons/:id" => controllers::person::delete,
            put     "persons/:id" => controllers::person::update,
            post    "persons/:id" => controllers::person::update,
            get     "persons/:id" => controllers::person::show)
}
