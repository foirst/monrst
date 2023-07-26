//! Events initiated by the client

use crate::model::channel::message::Message;

/// Events that can be sent by a client to the server
#[derive(Debug)]
pub enum Events {
    /// Ping the server to check the connection
    Ping,

    /// New message sent
    MessageSent(Message),
}
