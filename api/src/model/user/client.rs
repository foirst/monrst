//! Structures related to clients

use uuid::Uuid;

/// Kind of client
///
/// It can be desktop, mobile, ...
#[derive(Debug)]
pub enum Kind {
    /// Terminal client
    Tui,

    /// Desktop client
    Desktop,

    /// Mobile client
    Mobile,

    /// Web client
    Web,
}

/// Main structure for clients
///
/// A client is not a user: a user can have several clients (mobile app, desktop app, ...).
#[derive(Debug)]
pub struct Client {
    /// Unique identifier
    pub uuid: Uuid,

    /// List of users attached to this client represented by their UUIDs
    pub attached_users: Vec<Uuid>,

    /// Kind of client
    pub kind: Kind,
}
