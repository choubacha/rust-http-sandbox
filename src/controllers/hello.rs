use iron::prelude::*;
use iron::status;

pub fn hello(req: &mut Request) -> IronResult<Response> {
    let name = super::extract_param(&req, "name");
    Ok(Response::with((status::Ok, format!("Hello: {}", name))))
}
