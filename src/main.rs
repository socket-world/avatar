extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn health(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "health")))
}

fn manifest(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "manifest")))
}

fn main() {
    let mut router = Router::new();

    router.get("/", health, "health");
    router.post("/", manifest, "manifest");

    Iron::new(router).http("0.0.0.0:8080").unwrap();
}
