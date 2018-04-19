extern crate gotham;
extern crate hyper;
extern crate mime;

mod to_json;

use hyper::{Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::router::builder::*;
use gotham::router::Router;

pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(say_hello);
        route.get("/mine").to(say_hello);
        route.get("/blocks").to(say_hello);
        route.post("/blocks").to(say_hello);

    })
}
pub fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:7878";

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
