extern crate iron;
extern crate router;
extern crate rustc_serialize;

use rustc_serialize::json;
use iron::prelude::*;
use std::io::Read;
use iron::status;

#[derive(RustcDecodable, RustcEncodable)]
struct HelloWorld {
    hello: String,
    world: f64
}

pub fn parse(req: &mut Request) -> IronResult<Response> {
    let mut buf: String = String::new();
    match req.body.read_to_string(&mut buf) {
        Ok(_) => {
            match json::decode::<HelloWorld>(&buf) {
                Ok(decoded) => success(decoded),
                Err(err) => fail(format!("Could not parse json: {:?}", err)),
            }
        },
        Err(_) => fail(buf),
    }
}

fn success(body: HelloWorld) -> IronResult<Response> {
    Ok(Response::with((status::Ok, format!("You said: {}, to world number: {}", body.hello, body.world))))
}

fn fail(body: String) -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, body)))
}
