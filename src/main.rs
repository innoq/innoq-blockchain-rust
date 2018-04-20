extern crate crypto_hash;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate rayon;

use std::sync::{Arc, Mutex};

mod to_json;
mod block;
mod calculate_proof;

use hyper::{Response, StatusCode};
use gotham::handler::HandlerFuture;
use futures::future;

use gotham::middleware::{Middleware, NewMiddleware};
use gotham::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use block::{Blockchain, Transaction};
use to_json::ToJSON;
use gotham::state::{State, StateData};

#[derive(Debug)]
struct ServerState {
    blockchain: Blockchain,
    candidates: Vec<Transaction>,
}

impl ServerState {
    fn update(&mut self, blockchain: Blockchain) {
        self.blockchain = blockchain;
    }
}

fn get_blocks_handler(state: State) -> Box<HandlerFuture> {
    let server_state = state.borrow::<InjectedStateData>().state.clone();
    let status_code = StatusCode::Ok;

    let blockchain = &server_state.lock().unwrap().blockchain;

    let res = {
        create_response(
            &state,
            status_code,
            Some((blockchain.to_json().into_bytes(), mime::APPLICATION_JSON)),
        )
    };
    Box::new(future::ok((state, res)))
}

fn mine_handler(mut state: State) -> Box<HandlerFuture> {
    let arc = state.borrow_mut::<InjectedStateData>().state.clone();
    let mut server_state = arc.lock().unwrap();
    let status_code = StatusCode::Created;

    let block = server_state.blockchain.generate_next_block();
    let new_chain = server_state.blockchain.add(block);
    server_state.update(new_chain);

    let res = {
        create_response(
            &state,
            status_code,
            Some((server_state.blockchain.to_json().into_bytes(), mime::APPLICATION_JSON)),
        )
    };
    Box::new(future::ok((state, res)))
}

pub fn say_hello(mut state: State) -> (State, Response) {
    let server_state = state.borrow_mut::<InjectedStateData>().state.clone();
    eprintln!("{:?}", server_state.lock().unwrap().blockchain);
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

#[derive(Clone)]
pub struct StateInjectingMiddleware {
    state: Arc<Mutex<ServerState>>,
}

impl NewMiddleware for StateInjectingMiddleware {
    type Instance = Self;

    fn new_middleware(&self) -> std::io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

pub struct InjectedStateData {
    state: Arc<Mutex<ServerState>>,
}

impl StateData for InjectedStateData {}

impl Middleware for StateInjectingMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        state.put(InjectedStateData {
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
    let mw = StateInjectingMiddleware { state: state };
    let (chain, pipelines) = single_pipeline(new_pipeline().add(mw).build());

    build_router(chain, pipelines, |route| {
        route.get("/").to(say_hello);
        route.get("/mine").to(mine_handler);
        route.get("/blocks").to(get_blocks_handler);
    })
}

pub fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:7878";

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
