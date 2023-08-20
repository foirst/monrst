//! Client handling

use std::net::SocketAddr;

use anyhow::Result;
use log::info;
use tokio::io::{split, AsyncBufReadExt, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

/// Client implementation for the communication protocol
#[derive(Debug)]
struct Client {
    /// Reading part of the TLS over TCP stream to the client
    reading_buffer: BufReader<ReadHalf<TlsStream<TcpStream>>>,

    /// Reading part of the TLS over TCP stream to the client
    _writing_buffer: BufWriter<WriteHalf<TlsStream<TcpStream>>>,
}

/// Spawn a new websocket stream with a client
#[allow(clippy::unnecessary_wraps)]
pub async fn spawn(tls_stream: TlsStream<TcpStream>, addr: SocketAddr) -> Result<()> {
    let (reader, writer) = split(tls_stream);

    let mut client = Client {
        reading_buffer: BufReader::new(reader),
        _writing_buffer: BufWriter::new(writer),
    };

    let mut buffer = String::new();
    while let Ok(n) = client.reading_buffer.read_line(&mut buffer).await {
        if n == 0 {
            break;
        }
        info!("Echo from client {} - {}", addr, buffer);
        buffer.clear();
    }

    Ok(())
}
