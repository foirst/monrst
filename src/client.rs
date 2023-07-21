//! Client handling

use std::net::SocketAddr;

use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use async_std::task;
use futures_util::SinkExt;
use log::{error, info};

/// Spawn a new websocket stream with a client
pub fn spawn(stream: TcpStream, addr: SocketAddr) {
    task::spawn(async move {
        let mut ws_stream = match async_tungstenite::accept_async_with_config(stream, None).await {
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
