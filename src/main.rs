extern crate http_sandbox;
use http_sandbox::start_server;
use http_sandbox::routes::load_router;

fn main() {
    http_sandbox::start_server(http_sandbox::routes::load_router());
}
