extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate futures;

use std::time::SystemTime;
use std::sync::{Arc, Mutex};


mod to_json;
mod block;

use hyper::{Response, StatusCode};
use gotham::handler::{Handler, HandlerFuture, NewHandler};
use futures::future;

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::router::builder::*;
use gotham::router::Router;
use block::Blockchain;
use to_json::ToJSON;

#[derive(Clone, Debug)]
struct BlockchainHandler {
    started_at: SystemTime,
    blockchain: Arc<Mutex<Blockchain>>,
}

impl BlockchainHandler {
    fn new() -> BlockchainHandler {
        BlockchainHandler {
            started_at: SystemTime::now(),
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
        }
    }
}

impl Handler for BlockchainHandler {
    fn handle(self, state: State) -> Box<HandlerFuture> {
        let uptime = SystemTime::now().duration_since(self.started_at).unwrap();

        let blockchain = {
            let mut b = self.blockchain.lock().unwrap();
            b
        };

        let response_text = blockchain.blocks[0].to_json();

        let res = {
            create_response(
                &state,
                StatusCode::Ok,
                Some((response_text.into_bytes(), mime::TEXT_PLAIN)),
            )
        };
        Box::new(future::ok((state, res)))
    }
}

impl NewHandler for BlockchainHandler {
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
    build_simple_router(|route| {
        route.get("/").to(say_hello);
        route.get("/mine").to(say_hello);
        route.get("/blocks").to_new_handler(BlockchainHandler::new());
        route.post("/blocks").to(say_hello);
    })
}

pub fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:7878";

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
