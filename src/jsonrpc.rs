use std::option::Option;
use std::result::Result::{Err, Ok};
use std::sync::mpsc::{channel, RecvError, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

use jsonrpc_core::types::{
    Call, Error as JSONRPCError, ErrorCode, Failure, Id, MethodCall, Output, Params, Response, Success,
};
use rand;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use serde_json::{Error as SerdeError, Value};

use super::router::Error as RouterError;
use super::ws::{Error as WSError, Message, Sender as WSSender};

pub fn handle<F>(router: F, text: String) -> Option<String>
where
    F: FnOnce(String, Value) -> Result<Option<Value>, RouterError>, {
    let deserialized = serde_json::from_str(&text);
    let response: Option<Response> = match deserialized {
        Err(_) => Some(
            Failure {
                jsonrpc: None,
                id: Id::Null,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }.into(),
        ),
        Ok(Call::Invalid(id)) => Some(
            Failure {
                jsonrpc: None,
                id,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }.into(),
        ),
        Ok(Call::MethodCall(MethodCall {
            id,
            method,
            params,
            ..
        })) => {
            let value_params = serde_json::to_value(params).expect("Change to value always success");
            match router(method, value_params) {
                Ok(Some(value)) => Some(
                    Success {
                        jsonrpc: None,
                        result: value,
                        id,
                    }.into(),
                ),
                Ok(None) => {
                    let mut error = JSONRPCError::new(ErrorCode::InternalError);
                    error.data = Some(serde_json::Value::String("API returns no value".to_string()));
                    Some(
                        Failure {
                            jsonrpc: None,
                            id,
                            error,
                        }.into(),
                    )
                }
                Err(RouterError::MethodNotFound) => Some(
                    Failure {
                        jsonrpc: None,
                        id,
                        error: JSONRPCError::new(ErrorCode::MethodNotFound),
                    }.into(),
                ),
                Err(RouterError::RPC(err)) => Some(
                    Failure {
                        jsonrpc: None,
                        id,
                        error: err.to_jsonrpc_error(),
                    }.into(),
                ),
            }
        }
        Ok(Call::Notification(_)) => None,
    };
    response.map(|response| serde_json::to_string(&response).expect("Should success serialize"))
}

pub fn invalid_format() -> String {
    serde_json::to_string(&Failure {
        jsonrpc: None,
        id: Id::Null,
        error: JSONRPCError::new(ErrorCode::ParseError),
    }).expect("Should success serialize")
}

#[derive(Clone)]
pub struct Context {
    pub ws_sender: WSSender,
    pub ws_callback: Arc<Mutex<Option<Sender<String>>>>,
}

impl Context {
    pub fn new(sender: WSSender) -> Self {
        Self {
            ws_sender: sender,
            ws_callback: Arc::new(Mutex::new(None)),
        }
    }
}

pub enum CallError {
    InternalWS(WSError),
    InternalRecv(RecvError),
    InternalSerde(SerdeError),
    InternalSync(String),
    Response(JSONRPCError),
}

impl From<WSError> for CallError {
    fn from(error: WSError) -> Self {
        CallError::InternalWS(error)
    }
}

impl From<RecvError> for CallError {
    fn from(error: RecvError) -> Self {
        CallError::InternalRecv(error)
    }
}

impl From<SerdeError> for CallError {
    fn from(error: SerdeError) -> Self {
        CallError::InternalSerde(error)
    }
}

impl<T> From<PoisonError<T>> for CallError {
    fn from(error: PoisonError<T>) -> Self {
        CallError::InternalSync(format!("{:?}", error))
    }
}

impl From<JSONRPCError> for CallError {
    fn from(error: JSONRPCError) -> Self {
        CallError::Response(error)
    }
}

pub fn call<Arg, Res>(context: Context, method: &str, arg: Arg) -> Result<Res, CallError>
where
    Arg: Serialize,
    Res: DeserializeOwned, {
    let (tx, rx) = channel();
    let arg_value = serde_json::to_value(arg)?;
    let request = MethodCall {
        jsonrpc: None,
        method: method.to_string(),
        params: Some(Params::Array(vec![arg_value])),
        id: Id::Num(rand::random()),
    };
    let serialized_request = serde_json::to_string(&request)?;

    let mut callback_manager = context.ws_callback.lock()?;
    *callback_manager = Some(tx);
    drop(callback_manager);

    context.ws_sender.send(Message::Text(serialized_request))?;
    let received_string = rx.recv()?;

    let mut callback_manager = context.ws_callback.lock()?;
    *callback_manager = None;
    drop(callback_manager);

    let res = serde_json::from_str(&received_string)?;

    match res {
        Output::Success(success) => {
            let result = serde_json::from_value(success.result)?;
            Ok(result)
        }
        Output::Failure(failure) => Err(failure.error.into()),
    }
}

// Called on websocket thread
pub fn on_receive(context: Context, text: String) {
    let sender = context.ws_callback.lock();
    if sender.is_err() {
        cerror!("Cannot get callback from lock {:?}", sender);
        return
    }
    let sender = sender.expect("Error is checked");
    match *sender {
        None => {
            cerror!("Callback is empty {:?}", sender);
        }
        Some(ref sender) => {
            let send_result = sender.send(text);
            if send_result.is_err() {
                cerror!("Callback call failed {:?}", send_result);
            }
        }
    }
}
