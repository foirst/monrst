//! Events initiated by the client

use crate::model::channel::message::Message;

/// Events that can be sent by a client to the server
#[derive(Debug)]
pub enum Events {
    /// Introduction to the server
    Handshake,

    /// New message sent
    MessageSent(Message),
}
