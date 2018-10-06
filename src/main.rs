extern crate actix;
extern crate actix_web;
extern crate core;
extern crate docopt;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate simple_logger;
extern crate subprocess;


use actix::Actor;
use actix::Addr;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::System;
use actix_web::App;
use actix_web::server;
use docopt::Docopt;
use log::Level;
use std::path::PathBuf;
use std::thread;

mod utils;
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

#[derive(Debug, Clone, Copy)]
pub enum Status {
    NEW,
    DONE,
    ERROR,
}

#[derive(Debug, Clone)]
struct Movie(PathBuf, Status);

impl Message for Movie {
    type Result = bool;
}

unsafe impl Sync for Movie {}

unsafe impl Send for Movie {}

struct Discoverer {
    state: Addr<State>
}

impl Actor for Discoverer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Discoverer started");
        thread::spawn(move || {
            let mut i = 0;
            loop {
                i = i + 1;
                let path = PathBuf::from(format!("/media/{}.mp4", i));
                let movie = Movie(path, Status::NEW);
                self.state.send(movie);
                thread::sleep(utils::WAITE_TIME);
            }
        });
    }
}

impl Handler<Movie> for Discoverer {
    type Result = bool;   // <- Message response type

    fn handle(&mut self, msg: Movie, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");
        true
    }
}


struct State;

impl Actor for State {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("State started");
    }
}

impl Handler<Movie> for State {
    type Result = bool;   // <- Message response type

    fn handle(&mut self, msg: Movie, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received {}", msg);
        true
    }
}

//struct Converter {
//    sender: Sender<Movie>,
//}
//
//impl Converter {
//    fn run(&mut self) {}
//}


fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let sys = System::new("vrungel");

    // Start MyActor in current thread
    let addr_s = State.start();
    let addr_d = Discoverer { state: addr_s }.start();
    sys.run();


    let factory = move || {
        return App::new()
            .resource("/", |r| r.h(endpoints::IndexEndpoint::new()));
    };

    let addr: String = format!("127.0.0.1:{}", args.flag_port);
    server::new(factory)
        .bind(addr)
        .unwrap()
        .run();
}
