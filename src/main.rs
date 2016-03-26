extern crate http_sandbox;

fn main() {
    http_sandbox::start_server(http_sandbox::routes::load_router());
}
