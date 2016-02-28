use rustc_serialize::json;
use iron::prelude::*;
use iron::status;
use std::io::Read;
use router::Router;

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    id: Option<i32>,
    first_name: String,
    last_name: String,
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let mut buf: String = String::new();
    let pool = req.get::<::persistent::Read<::middlewares::db_pool::DbPool>>().unwrap();
    match req.body.read_to_string(&mut buf) {
        Ok(_) => {
            match json::decode::<Person>(&buf) {
                Ok(decoded) => {
                    match pool.get() {
                        Ok(conn) => {
                            match conn.query("insert into http_sandbox.persons (first_name, last_name) values ($1, $2) returning id",
                                             &[&decoded.first_name, &decoded.last_name]) {
                                Ok(rows) => {
                                    let row = rows.get(0);
                                    render(Person { id: row.get("id"), .. decoded })
                                },
                                Err(err) => fail(format!("Could not query db: {:?}", err)),
                            }
                        },
                        Err(err) => fail(format!("Could not get a db conn: {:?}", err)),
                    }
                },
                Err(err) => fail(format!("Could not parse json: {:?}", err)),
            }
        },
        Err(_) => fail(buf),
    }
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "update")))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let person_param = super::extract_param(&req, "id");
    let person_id = person_param.parse::<i32>().unwrap_or(0i32);
    let pool = req.get::<::persistent::Read<::middlewares::db_pool::DbPool>>().unwrap();
    match pool.get() {
        Ok(conn) => {
            match conn.query("select * from http_sandbox.persons where id = $1", &[&person_id]) {
                Ok(rows) => {
                    if rows.len() == 0 {
                        Ok(Response::with((status::NotFound, "")))
                    } else {
                        let row = rows.get(0);
                        let person = Person {
                            id: row.get("id"),
                            first_name: row.get("first_name"),
                            last_name: row.get("last_name")
                        };
                        render(person)
                    }
                },
                Err(err) => fail(format!("Could not query db: {:?}", err)),
            }
        },
        Err(err) => fail(format!("Could not get a db conn: {:?}", err)),
    }
}

pub fn index(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "index")))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "delete")))
}

fn render(person: Person) -> IronResult<Response> {
    let body = json::encode(&person).unwrap_or(String::new());
    Ok(Response::with((status::Ok, body)))
}

fn fail(body: String) -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, body)))
}
