use models::person::Person;
use middlewares::db_pool::PostgresPooledConnection;
use postgres::rows::Row;

/// Provides access to the persons table through a few simple
/// crud operations
pub struct PersonAccess {
    conn: PostgresPooledConnection
}

impl Person {
    /// Returns a Person as parsed from a postgres row.
    fn from_row(row: Row) -> Person {
        Person {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name")
        }
    }
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
    /// Returns a access object
    pub fn new(conn: PostgresPooledConnection) -> PersonAccess {
        PersonAccess { conn: conn }
    }

    /// Inserts a given person into the database
    pub fn create(&self, person: &Person) -> Result<Person, String> {
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

    /// Updates a given person in the data. The id of the person is
    /// not optional.
    pub fn update(&self, person: &Person) -> Result<(), String> {
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

    /// Deletes the person with the corresponding id from the db
    pub fn delete(&self, person_id: i32) -> Result<(), String> {
        match self.conn.execute(DELETE_QUERY, &[&person_id]) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Could not query db: {:?}", err)),
        }
    }

    /// Retrieves all the people from the database
    pub fn index(&self) -> Result<Vec<Person>, String> {
        match self.conn.query(INDEX_QUERY, &[]) {
            Ok(rows) => Ok(rows.iter().map(Person::from_row).collect()),
            Err(err) => Err(format!("Could not query db: {:?}", err))
        }
    }

    /// Retrieves a single person from the database.
    pub fn get(&self, person_id: i32) -> Result<Option<Person>, String> {
        match self.conn.query(GET_QUERY, &[&person_id]) {
            Ok(rows) => {
                if rows.len() == 1 {
                    Ok(Some(Person::from_row(rows.get(0))))
                } else {
                    Ok(None)
                }
            },
            Err(err) => Err(format!("Could not query db: {:?}", err))
        }
    }
}
