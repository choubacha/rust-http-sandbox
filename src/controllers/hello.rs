extern crate iron;
extern crate router;
use iron::prelude::*;
use iron::status;
use router::Router;

/// j
pub fn hello(req: &mut Request) -> IronResult<Response> {
    let name = req.extensions
        .get::<Router>()
        .unwrap()
        .find("name")
        .unwrap_or("/");
    Ok(Response::with((status::Ok, format!("Hello: {}", name))))
}
