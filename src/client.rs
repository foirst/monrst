//! Client handling

use core::str::FromStr;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use async_std::sync::RwLock;
use async_std::task;
use async_tungstenite::tungstenite::handshake;
use async_tungstenite::tungstenite::handshake::server::Callback;
use derive_more::Deref;
use futures::channel::oneshot::{self, Sender};
use futures_util::SinkExt;
use log::{error, info};
use monrst_api::model::user::client::Client;
use monrst_api::protocol::{self, Version};
use querystring::querify;
use spin::Lazy;
use uuid::Uuid;

/// Structure used to hold one side of a channel to receive the parsed information
#[derive(Debug, Deref)]
pub struct WebsocketHandshakeCallback(Sender<protocol::Configuration>);

impl WebsocketHandshakeCallback {
    /// Creates a new instance from a given sender
    pub fn new(sender: Sender<protocol::Configuration>) -> Self {
        Self(sender)
    }
}

impl Callback for WebsocketHandshakeCallback {
    fn on_request(
        self,
        request: &handshake::server::Request,
        response: handshake::server::Response,
    ) -> Result<handshake::server::Response, handshake::server::ErrorResponse> {
        /// Returns the first element `y` such that `(x, y)` is in `list`
        fn vec_association<'element, S: PartialEq, T>(list: &'element [(S, T)], x: &'element S) -> Option<&'element T> {
            Some(&list.iter().find(|(z, _y)| x == z)?.1)
        }

        let query = request.uri().query().unwrap_or_default();
        let params = querify(query);

        let Some(&stringed_version) = vec_association(&params, &"version") else { return Err(handshake::server::ErrorResponse::new(Some("Version required in handshake callback but not found".to_owned()))) };
        let Some(&stringed_format) = vec_association(&params, &"format") else { return Err(handshake::server::ErrorResponse::new(Some("Format required in handshake callback but not found".to_owned()))) };

        let Ok(version) = Version::from_str(stringed_version) else { return Err(handshake::server::ErrorResponse::new(Some(format!("Bad version format : \"{stringed_version}\"")))) };
        let Ok(format) = protocol::Format::from_str(stringed_format) else { return Err(handshake::server::ErrorResponse::new(Some(format!("Bad format format : \"{stringed_format}\"")))) };

        if self.0.send(protocol::Configuration { format }).is_ok() && Version::are_compatible(&version, &protocol::VERSION) {
            Ok(response)
        } else {
            Err(handshake::server::ErrorResponse::new(None))
        }
    }
}

/// Spawn a new websocket stream with a client
pub fn spawn(stream: TcpStream, addr: SocketAddr) {
    task::spawn(async move {
        let (sender, _receiver) = oneshot::channel::<protocol::Configuration>();
        let mut ws_stream = match async_tungstenite::accept_hdr_async(stream, WebsocketHandshakeCallback::new(sender)).await {
            Ok(ws) => ws,
            Err(err) => {
                error!("Failed to upgrade a TCP stream to websocket: {:?}", err);
                return;
            },
        };

        while let Some(Ok(msg)) = ws_stream.next().await {
            if let Err(err) = ws_stream.send(msg).await {
                info!("Peer {} : {}", addr, err);
            }
        }
    });
}
