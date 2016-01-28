extern crate iron;

#[macro_use(router)]
extern crate router;

use iron::prelude::*;
use router::Router;

mod controllers;
mod routes;

fn main() {
    start_server(routes::load_router());
}

fn start_server(router: Router) {
    Iron::new(router)
        .http("0.0.0.0:3000")
        .unwrap();
}
