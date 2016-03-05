use iron::prelude::*;
use iron::status;
use middlewares::db_pool::PostgresPooledConnection;
use postgres::rows::Row;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    id: Option<i32>,
    first_name: String,
    last_name: String,
}

struct PersonAccess {
    conn: PostgresPooledConnection
}
const CREATE_QUERY: &'static str =
"   insert into http_sandbox.persons
    (first_name, last_name) values ($1, $2)
    returning id";

const UPDATE_QUERY: &'static str =
"   update http_sandbox.persons
    set (first_name, last_name) = ($2, $3)
    where id = $1;";

const GET_QUERY: &'static str =
    "select * from http_sandbox.persons where id = $1";

const INDEX_QUERY: &'static str =
    "select * from http_sandbox.persons";

const DELETE_QUERY: &'static str =
    "delete from http_sandbox.persons where id = $1";

impl PersonAccess {
    fn new(conn: PostgresPooledConnection) -> PersonAccess {
        PersonAccess { conn: conn }
    }

    fn from_row(row: Row) -> Person {
        Person {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name")
        }
    }

    fn create(&self, person: &Person) -> Result<Person, String> {
        match self.conn.query(CREATE_QUERY, &[&person.first_name, &person.last_name]) {
            Ok(rows) => {
                Ok(Person {
                    id: Some(rows.get(0).get("id")),
                    first_name: person.first_name.clone(),
                    last_name: person.last_name.clone()
                })
            },
            Err(err) => Err(format!("Could not query db: {:?}", err)),
        }
    }

    fn update(&self, person: &Person) -> Result<(), String> {
        match self.conn.execute(UPDATE_QUERY,
                                &[
                                    &person.id,
                                    &person.first_name,
                                    &person.last_name
                                ])
        {
            Ok(_)   => Ok(()),
            Err(err) => Err(format!("Could not query db: {:?}", err)),
        }
    }

    fn delete(&self, person_id: i32) -> Result<(), String> {
        match self.conn.execute(DELETE_QUERY, &[&person_id]) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Could not query db: {:?}", err)),
        }
    }

    fn index(&self) -> Result<Vec<Person>, String> {
        match self.conn.query(INDEX_QUERY, &[]) {
            Ok(rows) => Ok(rows.iter().map(PersonAccess::from_row).collect()),
            Err(err) => Err(format!("Could not query db: {:?}", err))
        }
    }

    fn get(&self, person_id: i32) -> Result<Option<Person>, String> {
        match self.conn.query(GET_QUERY, &[&person_id]) {
            Ok(rows) => {
                if rows.len() == 1 {
                    Ok(Some(PersonAccess::from_row(rows.get(0))))
                } else {
                    Ok(None)
                }
            },
            Err(err) => Err(format!("Could not query db: {:?}", err))
        }
    }
}

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
