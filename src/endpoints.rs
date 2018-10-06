use actix_web::dev::Handler;
use actix_web::HttpRequest;


pub struct IndexEndpoint {}

impl IndexEndpoint {
    pub fn new() -> IndexEndpoint {
        IndexEndpoint {}
    }
}

impl<S> Handler<S> for IndexEndpoint {
    type Result = &'static str;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        return "Hello world!";
    }
}
