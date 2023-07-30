//! Representation of messages

use uuid::Uuid;

/// Main structure for messages
#[derive(Debug, Clone)]
pub struct Message {
    /// Unique identifier
    pub uuid: Uuid,

    /// Channel in which this message was sent
    pub channel: Uuid,

    /// UUID of the message sender
    pub author: Uuid,

    /// Content of the message
    pub content: String,
}

impl Message {
    /// Creates a new message
    #[inline]
    #[must_use]
    pub fn new(channel: Uuid, author: Uuid, content: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            channel,
            author,
            content,
        }
    }
}
