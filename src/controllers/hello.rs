extern crate iron;
extern crate router;
use iron::prelude::*;
use iron::status;
use router::Router;

pub fn hello(req: &mut Request) -> IronResult<Response> {
    let name = extract_param(&req, "name");
    Ok(Response::with((status::Ok, format!("Hello: {}", name))))
}

fn extract_param<'a>(req: &'a Request, param: &str) -> &'a str {
    req.extensions
        .get::<Router>()
        .unwrap()
        .find(param)
        .unwrap_or("")
}
