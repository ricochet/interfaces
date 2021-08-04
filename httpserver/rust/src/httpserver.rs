// This file is generated automatically using wasmcloud-weld and smithy model definitions
//

#![allow(clippy::ptr_arg)]
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use wasmbus_rpc::{
    client, context, deserialize, serialize, Message, MessageDispatch, RpcError, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

/// map data structure for holding http headers
///
pub type HeaderMap = std::collections::HashMap<String, HeaderValues>;

pub type HeaderValues = Vec<String>;

/// HttpRequest contains data sent to actor about the http request
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HttpRequest {
    /// Request body as a byte array. May be empty.
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
    /// map of request headers (string key, string value)
    pub header: HeaderMap,
    /// HTTP method. One of: GET,POST,PUT,DELETE,HEAD,OPTIONS,CONNECT,PATCH,TRACE
    #[serde(default)]
    pub method: String,
    /// full request path
    #[serde(default)]
    pub path: String,
    /// query string. May be an empty string if there were no query parameters.
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
}

/// HttpResponse contains the actor's response to return to the http client
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HttpResponse {
    /// Body of response as a byte array. May be an empty array.
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
    /// Map of headers (string keys, list of values)
    pub header: HeaderMap,
    /// statusCode is a three-digit number, usually in the range 100-599,
    /// A value of 200 indicates success.
    #[serde(rename = "statusCode")]
    pub status_code: u16,
}

/// HttpServer is the contract to be implemented by actor
/// wasmbus.contractId: wasmcloud:httpserver
/// wasmbus.actorReceive
#[async_trait]
pub trait HttpServer {
    async fn handle_request(
        &self,
        ctx: &context::Context<'_>,
        arg: &HttpRequest,
    ) -> Result<HttpResponse, RpcError>;
}

/// HttpServerReceiver receives messages defined in the HttpServer service trait
/// HttpServer is the contract to be implemented by actor
#[async_trait]
pub trait HttpServerReceiver: MessageDispatch + HttpServer {
    async fn dispatch(
        &self,
        ctx: &context::Context<'_>,
        message: &Message<'_>,
    ) -> Result<Message<'_>, RpcError> {
        match message.method {
            "HandleRequest" => {
                let value: HttpRequest = deserialize(message.arg.as_ref())?;
                let resp = HttpServer::handle_request(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "HttpServer.HandleRequest",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "HttpServer::{}",
                message.method
            ))),
        }
    }
}

/// HttpServerSender sends messages to a HttpServer service
/// HttpServer is the contract to be implemented by actor
#[derive(Debug)]
pub struct HttpServerSender<T> {
    transport: T,
    config: client::SendConfig,
}

impl<T: Transport> HttpServerSender<T> {
    pub fn new(config: client::SendConfig, transport: T) -> Self {
        HttpServerSender { transport, config }
    }
}

#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> HttpServer for HttpServerSender<T> {
    #[allow(unused)]
    async fn handle_request(
        &self,
        ctx: &context::Context<'_>,
        arg: &HttpRequest,
    ) -> Result<HttpResponse, RpcError> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                &self.config,
                Message {
                    method: "HandleRequest",
                    arg: Cow::Borrowed(&arg),
                },
            )
            .await?;
        let value = deserialize(resp.arg.as_ref())?;
        Ok(value)
    }
}