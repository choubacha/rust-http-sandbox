extern crate core;
extern crate iron;
extern crate persistent;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rustc_serialize;

#[macro_use(router)]
extern crate router;

use iron::prelude::*;
use router::Router;

mod controllers;
mod middlewares;
mod models;
mod access;
pub mod routes;

/// Command to start the server.
pub fn start_server(router: Router) {
    let mut chain = Chain::new(router);
    chain.link_before(middlewares::db_pool::build());
    Iron::new(chain)
        .http("0.0.0.0:3000")
        .unwrap();
}
