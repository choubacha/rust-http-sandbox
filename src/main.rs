extern crate http_sandbox;
extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = Options::new();
    opts.optopt("i", "ip", "The ip address that the host will run on. Defaults to 0.0.0.0", "IP");
    opts.optopt("p", "port", "The port to mount the server on. Defaults to 3000", "PORT");
    opts.optflag("h", "help", "Print the help menu");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let host = matches.opt_str("i").unwrap_or(String::from("0.0.0.0"));
    let port = matches.opt_str("p").unwrap_or(String::from("3000")).parse::<u16>().unwrap();
    println!("Starting server on {}:{}", host, port);
    http_sandbox::start_server((&*host, port), http_sandbox::routes::load_router());
}
