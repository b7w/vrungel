extern crate actix_web;
extern crate docopt;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate simple_logger;
extern crate subprocess;


use actix_web::App;
use actix_web::server;
use docopt::Docopt;
use log::Level;
use std::sync::Arc;

mod utils;
mod core;
mod endpoints;

const USAGE: &'static str = "
Vrungel.

Usage:
  vrungel [options] <path>
  vrungel (-h | --help)

Options:
  -h --help             Show this screen.
  -p <n>, --port <n>    Http port [default: 8080]
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String,
    flag_port: String,
}


fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut state = core::State::new();
    state.start_discovering(args.arg_path);
    state.start_conveter();

    let arc_state = Arc::new(state);

    let factory = move || {
        let s = arc_state.clone();
        return App::new()
            .resource("/", |r| r.h(endpoints::IndexEndpoint::new()))
            .resource("/queue/size", |r| r.h(endpoints::QueueSizeEndpoint::new(s)));
    };

    let addr: String = format!("127.0.0.1:{}", args.flag_port);
    server::new(factory)
        .bind(addr)
        .unwrap()
        .run();
}
