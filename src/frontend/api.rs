use super::super::common_rpc_types::{NodeStatus, ShellStartCodeChainRequest};
use super::super::router::Router;
use super::super::rpc::{response, RPCResponse};
use super::types::{
    BlockId, Context, DashboardGetNetworkResponse, DashboardNode, HardwareInfo, HardwareUsage, NetworkPermission,
    NodeConnection, NodeGetInfoResponse, NodeVersion,
};

pub fn add_routing(router: &mut Router<Context>) {
    router.add_route("ping", Box::new(ping as fn(Context) -> RPCResponse<String>));
    router.add_route(
        "dashboard_getNetwork",
        Box::new(dashboard_get_network as fn(Context) -> RPCResponse<DashboardGetNetworkResponse>),
    );
    router.add_route("node_getInfo", Box::new(node_get_info as fn(Context) -> RPCResponse<NodeGetInfoResponse>));
    router.add_route(
        "real_dashboard_getNetwork",
        Box::new(real_dashboard_get_network as fn(Context) -> RPCResponse<DashboardGetNetworkResponse>),
    );
    router.add_route(
        "shell_startCodeChain",
        Box::new(shell_start_codechain as fn(Context) -> RPCResponse<DashboardGetNetworkResponse>),
    )
}

fn ping(_: Context) -> RPCResponse<String> {
    response("pong".to_string())
}

fn dashboard_get_network(_: Context) -> RPCResponse<DashboardGetNetworkResponse> {
    response(DashboardGetNetworkResponse {
        nodes: vec![
            DashboardNode::Normal {
                name: Some("Gilyoung".to_string()),
                status: NodeStatus::Run,
                address: "127.0.0.1:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::Normal {
                name: None,
                status: NodeStatus::Run,
                address: "127.0.0.2:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::Normal {
                name: Some("Hi stopped test node1".to_string()),
                status: NodeStatus::Stop,
                address: "42.124.241.2:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::Normal {
                name: Some("Test Error node".to_string()),
                status: NodeStatus::Error,
                address: "127.0.0.3:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::UFO {
                status: NodeStatus::UFO,
                address: "2.2.2.2:3485".parse().unwrap(),
            },
        ],
        connections: vec![NodeConnection {
            node_a: "127.0.0.1:3485".parse().unwrap(),
            node_b: "127.0.0.2:3485".parse().unwrap(),
        }],
    })
}

fn real_dashboard_get_network(context: Context) -> RPCResponse<DashboardGetNetworkResponse> {
    let agent_infos = context.agent_service.read_state().get_agent_info();
    let dashboard_nodes = agent_infos.iter().filter_map(DashboardNode::from_state).collect();
    response(DashboardGetNetworkResponse {
        nodes: dashboard_nodes,
        connections: Vec::new(),
    })
}

fn node_get_info(_: Context) -> RPCResponse<NodeGetInfoResponse> {
    response(NodeGetInfoResponse {
        address: "127.0.0.1:3485".parse().unwrap(),
        version: NodeVersion {
            version: "0.1.0".to_string(),
            hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
        },
        status: NodeStatus::Run,
        commit_hash: "84e70586dea8e6b4021d65b8164bbac28cb88ecb".to_string(),
        best_block_id: BlockId {
            block_number: 0,
            hash: Default::default(),
        },
        pending_parcels: Vec::new(),
        peers: Vec::new(),
        whitelist: NetworkPermission {
            list: Vec::new(),
            enabled: false,
        },
        blacklist: NetworkPermission {
            list: Vec::new(),
            enabled: false,
        },
        hardware: HardwareInfo {
            cpu_usage: vec![0.34, 0.03, 0.58],
            disk_usage: HardwareUsage {
                total: 3 * 1000 * 1000 * 1000,
                available: 5 * 1000 * 1000 * 1000,
                percentage_used: 0.6,
            },
            memory_usage: HardwareUsage {
                total: 3 * 1000 * 1000 * 1000,
                available: 5 * 1000 * 1000 * 1000,
                percentage_used: 0.6,
            },
        },
        events: vec!["Network connected".to_string(), "Block received".to_string()],
    })
}
