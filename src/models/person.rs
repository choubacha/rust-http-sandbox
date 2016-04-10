//use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Person {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String
}
