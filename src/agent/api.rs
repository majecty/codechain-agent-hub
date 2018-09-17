use super::super::jsonrpc;
use super::super::rpc::{response, RPCResult};
use super::types::ShellStartCodeChainRequest;

pub struct Agent {
    jsonrpc_context: jsonrpc::Context,
}

trait SendAgentRPC {
    fn shell_start_codechain(&self, _req: ShellStartCodeChainRequest) -> RPCResult<()>;
}

impl SendAgentRPC for Agent {
    fn shell_start_codechain(&self, req: ShellStartCodeChainRequest) -> RPCResult<()> {
        let _result: Result<(), _> = jsonrpc::call(self.jsonrpc_context.clone(), "shell_startCodeChain", req);
        response(())
    }
}
