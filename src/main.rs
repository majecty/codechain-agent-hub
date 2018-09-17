#[macro_use]
extern crate log;

extern crate codechain_rpc as crpc;
extern crate jsonrpc_core;
extern crate primitives as cprimitives;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ws;

#[macro_use]
mod logger;
mod agent;
mod frontend;
mod jsonrpc;
mod router;
mod rpc;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use ws::listen;

use self::agent::api::{add_routing as add_agent_routing};
use self::agent::handler::{WebSocketHandler as AgentHandler};
use self::frontend::api::{add_routing as add_frontend_routing};
use self::frontend::handler::{WebSocketHandler as FrontendHandler};
use self::logger::init as logger_init;
use self::router::Router;

fn main() {
    logger_init().expect("Logger should be initialized");

    let frontend_join = thread::Builder::new()
        .name("frontend listen".to_string())
        .spawn(|| {
            let count = Rc::new(Cell::new(0));
            let mut frontend_router = Arc::new(Router::new());
            add_frontend_routing(Arc::get_mut(&mut frontend_router).unwrap());
            listen("127.0.0.1:3012", |out| FrontendHandler {
                out,
                count: count.clone(),
                router: frontend_router.clone(),
            }).unwrap();
        }).expect("Should success listening frontend");

    let agent_join = thread::Builder::new()
        .name("agent listen".to_string())
        .spawn(|| {
            let count = Rc::new(Cell::new(0));
            let mut agent_router = Arc::new(Router::new());
            add_agent_routing(Arc::get_mut(&mut agent_router).unwrap());
            listen("127.0.0.1:4012", |out| AgentHandler {
                out,
                count: count.clone(),
                router: agent_router.clone(),
            }).unwrap();
        }).expect("Should success listening agent");

    frontend_join.join().expect("Join frotend listner");
    agent_join.join().expect("Join agent listner");
}
