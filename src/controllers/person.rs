use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use models::person::Person;
use access::person::PersonAccess;

pub fn create(req: &mut Request) -> IronResult<Response> {
    match super::parse_body::<Person>(req) {
        Ok(person) => {
            let result = super::with_conn(req, |conn| -> Result<Person, String> {
                PersonAccess::new(conn).create(&person)
            });
            match result {
                Ok(person) => render(person),
                Err(err) => fail(err)
            }
        },
        Err(err) => fail(err)
    }
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    let person_id = super::extract_or_param(&req, "id", 0i32);
    match super::parse_body::<Person>(req) {
        Ok(decoded) => {
            let person = Person { id: Some(person_id), .. decoded };
            let result = super::with_conn(req, |conn| -> Result<(), String> {
                PersonAccess::new(conn).update(&person)
            });
            match result {
                Ok(_) => render(person),
                Err(err) => fail(err)
            }
        },
        Err(err) => fail(err)
    }
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let person_id = super::extract_or_param(&req, "id", 0i32);

    let result = super::with_conn(req, |conn| -> Result<Option<Person>, String> {
        PersonAccess::new(conn).get(person_id)
    });

    match result {
        Ok(None)         => Ok(Response::with((status::NotFound, "Person not found"))),
        Ok(Some(person)) => render(person),
        Err(err)         => fail(err)
    }
}

pub fn index(req: &mut Request) -> IronResult<Response> {
    let result = super::with_conn(req, |conn| -> Result<Vec<Person>, String> {
        PersonAccess::new(conn).index()
    });
    match result {
        Ok(persons) => {
            let body = json::encode(&persons).unwrap_or(String::new());
            Ok(Response::with((status::Ok, body)))
        },
        Err(err) => fail(err)
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let person_id = super::extract_or_param(&req, "id", 0i32);

    let result = super::with_conn(req, |conn| -> Result<(), String> {
        PersonAccess::new(conn).delete(person_id)
    });

    match result {
        Ok(_)   => Ok(Response::with((status::Ok, ""))),
        Err(err)=> fail(err)
    }
}

fn render(person: Person) -> IronResult<Response> {
    let body = json::encode(&person).unwrap_or(String::new());
    Ok(Response::with((status::Ok, body)))
}

fn fail(body: String) -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, body)))
}
