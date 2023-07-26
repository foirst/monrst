//! Representation of messages

use uuid::Uuid;

use super::Channel;

/// Main structure for messages
#[derive(Debug)]
pub struct Message {
    /// Unique identifier
    pub uuid: Uuid,

    /// Channel in which this message was sent
    pub channel: Channel,

    /// UUID of the message sender
    pub author: Uuid,

    /// Content of the message
    pub content: String,
}
