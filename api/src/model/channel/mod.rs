//! Channels and their content

use uuid::Uuid;

pub mod message;

/// Main structure for channels
///
/// Channels are places where messages between users are sent
#[derive(Debug)]
pub enum Channel {
    /// Direct message channel between one or two users
    DirectMessage {
        /// Unique identifier
        uuid: Uuid,

        /// UUIDs of users in this channel
        ///
        /// If both UUIDs are equal, this channel is a direct message from a user to him/herself
        users: [Uuid; 2],
    },
}
