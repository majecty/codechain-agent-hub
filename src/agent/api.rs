use super::super::router::Router;
use super::super::rpc::{RPCResult, response};

use super::types::ShellStartCodeChainRequest;

pub fn add_routing(router: &mut Router) {
    router.add_route("ping", Box::new(ping as fn() -> RPCResult<String>));
    router.add_route(
        "shell_startCodeChain",
        Box::new(shell_start_codechain as fn(req: ShellStartCodeChainRequest) -> RPCResult<()>),
    );
}

fn ping() -> RPCResult<String> {
    response("pong".to_string())
}

fn shell_start_codechain(_req: ShellStartCodeChainRequest) -> RPCResult<()> {
    response(())
}
