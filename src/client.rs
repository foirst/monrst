//! Client handling

use std::net::SocketAddr;

use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use async_std::task;
use async_tungstenite::tungstenite::handshake::server::Callback;
use async_tungstenite::tungstenite::handshake::{self};
use derive_more::Deref;
use futures::channel::oneshot::{self, Sender};
use futures_util::SinkExt;
use log::{error, info};
use querystring::querify;

use crate::protocol::Configuration;

/// Structure used to hold one side of a channel to receive the parsed information
#[derive(Debug, Deref)]
pub struct WebsocketHandshakeCallback(Sender<Configuration>);

impl WebsocketHandshakeCallback {
    /// Creates a new instance from a given sender
    pub fn new(sender: Sender<Configuration>) -> Self {
        Self(sender)
    }
}

impl Callback for WebsocketHandshakeCallback {
    fn on_request(
        self,
        request: &handshake::server::Request,
        response: handshake::server::Response,
    ) -> Result<handshake::server::Response, handshake::server::ErrorResponse> {
        let query = request.uri().query().unwrap_or_default();
        let params = querify(query);

        todo!()
    }
}

/// Spawn a new websocket stream with a client
pub fn spawn(stream: TcpStream, addr: SocketAddr) {
    task::spawn(async move {
        let (sender, receiver) = oneshot::channel::<Configuration>();
        let mut ws_stream = match async_tungstenite::accept_hdr_async(stream, WebsocketHandshakeCallback::new(sender)).await {
            Ok(ws) => ws,
            Err(err) => {
                error!("{:?}", err);
                return;
            },
        };

        while let Some(Ok(msg)) = ws_stream.next().await {
            println!("{:?}", msg);
            if let Err(err) = ws_stream.send(msg).await {
                info!("Peer {} : {}", addr, err);
            }
        }
    });
}
