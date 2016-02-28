extern crate iron;

#[macro_use(router)]
extern crate router;

extern crate rustc_serialize;

use iron::prelude::*;
use router::Router;

mod controllers;
pub mod routes;

/// Command to start the server.
pub fn start_server(router: Router) {
    Iron::new(router)
        .http("0.0.0.0:3000")
        .unwrap();
}
