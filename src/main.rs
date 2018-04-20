extern crate crypto_hash;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate rayon;

use std::time::SystemTime;
use std::sync::{Arc, Mutex};

mod to_json;
mod block;
mod calculate_proof;

use hyper::{Method, Response, StatusCode};
use gotham::handler::{Handler, HandlerFuture, NewHandler};
use futures::future;

use gotham::middleware::{Middleware, NewMiddleware};
use gotham::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use block::{Blockchain, Transaction};
use to_json::ToJSON;
use gotham::state::{FromState, State, StateData};

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

pub fn say_hello(mut state: State) -> (State, Response) {
    let mut server_state = state.borrow_mut::<ExampleMiddlewareData>().state.clone();
    eprintln!("{:?}", server_state.lock().unwrap().blockchain);
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

#[derive(Clone)]
pub struct ExampleMiddleware {
    state: Arc<Mutex<ServerState>>,
}

impl NewMiddleware for ExampleMiddleware {
    type Instance = Self;

    fn new_middleware(&self) -> std::io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

pub struct ExampleMiddlewareData {
    state: Arc<Mutex<ServerState>>,
}

impl StateData for ExampleMiddlewareData {}

impl Middleware for ExampleMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        state.put(ExampleMiddlewareData {
            state: self.state.clone(),
        });

        chain(state)
    }
}

fn router() -> Router {
    let state = Arc::new(Mutex::new(ServerState {
        blockchain: Blockchain::new(),
        candidates: Vec::new(),
    }));
    let mw = ExampleMiddleware { state: state.clone() };
    let (chain, pipelines) = single_pipeline(new_pipeline().add(mw).build());

    build_router(chain, pipelines, |route| {
        route.get("/").to(say_hello);
        route.get("/mine").to(say_hello);
        route
            .get("/blocks")
            .to_new_handler(GetBlocksHandler::new(state.clone()));
    })
}

pub fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:7878";

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
