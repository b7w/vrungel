extern crate actix_web;
extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate subprocess;


use actix_web::App;
use actix_web::HttpRequest;
use actix_web::server;
use docopt::Docopt;

mod utils;
mod core;

const USAGE: &'static str = "
Vrungel.

Usage:
  vrungel <path>
  vrungel (-h | --help)
  vrungel --version

Options:
  -h --help             Show this screen.
  -p <n>, --port <n>    Http port [default: 8080]
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String,
    flag_port: String,
}


fn index(_req: &HttpRequest) -> &'static str {
    println!("Hello world!");
    return "Hello world!";
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Searching in {}", args.arg_path);

    let mut state = core::State::new();
    state.start_discovering(args.arg_path);
    state.run();

    let factory = || {
        App::new().resource("/", |r| r.f(index))
    };

    let addr: String = format!("127.0.0.1:{}", args.flag_port);
    println!("Start http at {}", addr);
    server::new(factory)
        .bind(addr)
        .unwrap()
        .run();
}
