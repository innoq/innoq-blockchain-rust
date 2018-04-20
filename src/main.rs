extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate futures;
extern crate crypto_hash;
extern crate rayon;

use std::time::SystemTime;
use std::sync::{Arc, Mutex};


mod to_json;
mod block;
mod calculate_proof;

use hyper::{Response, StatusCode, Method};
use gotham::handler::{Handler, HandlerFuture, NewHandler};
use futures::future;

use gotham::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use block::{Blockchain, Transaction};
use to_json::ToJSON;
use gotham::state::{FromState, State};

#[derive(Debug)]
struct ServerState {
    blockchain: Blockchain,
    candidates: Vec<Transaction>,
}

#[derive(Clone, Debug)]
struct GetBlocksHandler {
    started_at: SystemTime,
    state: Arc<Mutex<ServerState>>,
}

impl GetBlocksHandler {
    fn new(state: Arc<Mutex<ServerState>>) -> GetBlocksHandler {
        GetBlocksHandler {
            started_at: SystemTime::now(),
            state: state,
        }
    }
}

impl Handler for GetBlocksHandler {
    fn handle(self, state: State) -> Box<HandlerFuture> {

        let mut status_code = StatusCode::Ok;

        let mut blockchain = &self.state.lock().unwrap().blockchain;

        let res = {
            create_response(
                &state,
                status_code,
                Some((blockchain.to_json().into_bytes(), mime::APPLICATION_JSON)),
            )
        };
        Box::new(future::ok((state, res)))
    }
}

impl NewHandler for GetBlocksHandler {
    type Instance = Self;

    fn new_handler(&self) -> std::io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

fn router() -> Router {
    let state = Arc::new(Mutex::new(ServerState {
        blockchain: Blockchain::new(),
        candidates: Vec::new(),
    }));

    build_simple_router(|route| {
        route.get("/").to(say_hello);
        route.get("/mine").to(say_hello);
        route.get("/blocks").to_new_handler(GetBlocksHandler::new(state.clone()));
    })
}

pub fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:7878";

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
