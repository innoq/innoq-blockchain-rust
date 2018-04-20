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

use hyper::{Response, StatusCode};
use gotham::handler::HandlerFuture;
use futures::{future, Stream};

use gotham::middleware::{Middleware, NewMiddleware};
use gotham::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use block::{Blockchain, Transaction};
use to_json::ToJSON;
use gotham::state::{State, StateData};
use futures::Future;

#[derive(Debug)]
struct ServerState {
    blockchain: Blockchain,
    candidates: Vec<Transaction>,
}

fn get_blocks_handler(mut state: State) -> Box<HandlerFuture> {
    let mut server_state = state.borrow_mut::<InjectedStateData>().state.clone();
    let mut status_code = StatusCode::Ok;

    let mut blockchain = &server_state.lock().unwrap().blockchain;

    let res = {
        create_response(
            &state,
            status_code,
            Some((blockchain.to_json().into_bytes(), mime::APPLICATION_JSON)),
        )
    };
    Box::new(future::ok((state, res)))
}

fn add_transaction(mut state: State) -> Box<HandlerFuture> {
    let mut server_state = state.borrow_mut::<InjectedStateData>().state.clone();

    let mut candidates = &server_state.lock().unwrap().candidates;

    let body = state.take::<hyper::Body>().concat2();

    let full_body = body.map(|b| format!("{:?}", b)).map_err(|_| String::from("Kaboom"));
    let tuple_state_res = full_body.map(|body| {
        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((body.into_bytes(), mime::APPLICATION_JSON))
        );
        (state, res)
    });


    Box::new(future::ok(tuple_state_res.f))

    // let res = body.map(|src| {
    //             create_response(
    //                 &state,
    //                 StatusCode::Ok,
    //                 Some((format!("{:?}", src).into_bytes(), mime::APPLICATION_JSON)),
    //             )
    // }).or_else(|_|{
    //             create_response(
    //                 &state,
    //                 StatusCode::NotAcceptable,
    //                 Some((format!("error occurred").into_bytes(), mime::APPLICATION_JSON)),
    //             )
    // });
    // Box::new(future::ok((state, res)))


    // match body.wait() {
    //     Ok(src) => {
    //         let res = {
    //             create_response(
    //                 &state,
    //                 StatusCode::Ok,
    //                 Some((format!("{:?}", src).into_bytes(), mime::APPLICATION_JSON)),
    //             )
    //         };
    //         Box::new(future::ok((state, res)))
    //     }
    //     Err(e) => {
    //         let res = {
    //             create_response(
    //                 &state,
    //                 StatusCode::NotAcceptable,
    //                 Some((format!("error occurred {:?}", e).into_bytes(), mime::APPLICATION_JSON)),
    //             )
    //         };
    //         Box::new(future::ok((state, res)))
    //     }
    // }
}

pub fn say_hello(mut state: State) -> (State, Response) {
    let mut server_state = state.borrow_mut::<InjectedStateData>().state.clone();
    eprintln!("{:?}", server_state.lock().unwrap().blockchain);
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("Hello World!").into_bytes(),
            mime::TEXT_PLAIN,
        )),
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
        route.get("/mine").to(say_hello);
        route.get("/blocks").to(get_blocks_handler);
        route.post("/transactions").to(add_transaction);
    })
}

pub fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:7878";

    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
