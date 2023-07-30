//! Channels and their content

use uuid::Uuid;

pub mod message;

/// Main structure for channels
///
/// Channels are places where messages between users are sent
#[derive(Debug, Clone)]
pub enum Channel {
    /// Direct message channel between one or two users
    DirectMessage {
        /// Unique identifier
        uuid: Uuid,

        /// UUIDs of users in this channel
        ///
        /// If both UUIDs are equal, this channel is a direct message from a user to him/herself
        users: (Uuid, Uuid),
    },
}

impl Channel {
    /// Return the UUID of the channel
    #[inline]
    #[must_use]
    pub const fn uuid(&self) -> Uuid {
        match self {
            Self::DirectMessage { uuid, .. } => *uuid,
        }
    }

    /// Creates a new direct message channel
    #[inline]
    #[must_use]
    pub fn new_direct_message(user1: Uuid, user2: Uuid) -> Self {
        Self::DirectMessage {
            uuid: Uuid::new_v4(),
            users: (user1, user2),
        }
    }
}
