use actix_web::dev::Handler;
use actix_web::HttpRequest;
use core::State;

pub fn index(_req: &HttpRequest) -> &'static str {
    println!("Hello world!");
    return "Hello world!";
}

pub struct IndexEndpoint {}

impl IndexEndpoint {
    pub fn new() -> IndexEndpoint {
        IndexEndpoint {}
    }
}

impl<S> Handler<S> for IndexEndpoint {
    type Result = &'static str;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        return "Hello world!";
    }
}

pub struct QueueSizeEndpoint {
    state: State
}

impl QueueSizeEndpoint {
    pub fn new(state: State) -> QueueSizeEndpoint {
        QueueSizeEndpoint {
            state
        }
    }
}

impl<S> Handler<S> for QueueSizeEndpoint {
    type Result = String;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let s = self.state.queue_size();
        return format!("{}", s);
    }
}