extern crate iron;

#[macro_use(router)]
extern crate router;

use iron::prelude::*;

mod controllers;
mod routes;

fn main() {
    let router = routes::load_router();

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
