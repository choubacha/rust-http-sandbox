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
    match req.body.read_to_string(&mut buf) {
        Ok(_) => {
            match json::decode::<Person>(&buf) {
                Ok(decoded) => {
                    match create_person(req, &decoded) {
                        Ok(person) => render(person),
                        Err(err) => fail(err)
                    }
                },
                Err(err) => fail(format!("Could not parse json: {:?}", err)),
            }
        },
        Err(_) => fail(buf),
    }
}

fn create_person(req: &mut Request, person: &Person) -> Result<Person, String> {
    super::with_conn(req, |conn| -> Result<Person, String> {
        match conn.query("insert into http_sandbox.persons (first_name, last_name) values ($1, $2) returning id",
                         &[&person.first_name, &person.last_name]) {
            Ok(rows) => {
                Ok(Person {
                    id: Some(rows.get(0).get("id")),
                    first_name: person.first_name.clone(),
                    last_name: person.last_name.clone()
                })
            },
            Err(err) => Err(format!("Could not query db: {:?}", err)),
        }
    })
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "update")))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let person_param = super::extract_param(&req, "id");
    let person_id = person_param.parse::<i32>().unwrap_or(0i32);
    match get_person(req, person_id) {
        Ok(person) => render(person),
        Err(err) => {
            if err == String::from("404") {
                Ok(Response::with((status::NotFound, "Person not found")))
            } else {
                fail(err)
            }
        }
    }
}

fn get_person(req: &mut Request, person_id: i32) -> Result<Person, String> {
    super::with_conn(req, |conn| -> Result<Person, String> {
        match conn.query("select * from http_sandbox.persons where id = $1", &[&person_id]) {
            Ok(rows) => {
                if rows.len() != 1 {
                    Err(String::from("404"))
                } else {
                    let row = rows.get(0);
                    Ok(Person {
                        id: row.get("id"),
                        first_name: row.get("first_name"),
                        last_name: row.get("last_name")
                    })
                }
            }
            Err(err) => Err(format!("Could not query db: {:?}", err)),
        }
    })
}

fn get_people(req: &mut Request) -> Result<Vec<Person>, String> {
    super::with_conn(req, |conn| -> Result<Vec<Person>, String> {
        match conn.query("select * from http_sandbox.persons", &[]) {
            Ok(rows) => {
                Ok(rows.iter().map(|row| {
                        Person {
                            id: row.get("id"),
                            first_name: row.get("first_name"),
                            last_name: row.get("last_name")
                        }
                    }).collect()
                )
            }
            Err(err) => Err(format!("Could not query db: {:?}", err)),
        }
    })
}

pub fn index(req: &mut Request) -> IronResult<Response> {
    match get_people(req) {
        Ok(persons) => {
            let body = json::encode(&persons).unwrap_or(String::new());
            Ok(Response::with((status::Ok, body)))
        },
        Err(err) => fail(err)
    }
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
