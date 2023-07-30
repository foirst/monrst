//! Structures related to the server

use uuid::Uuid;

/// Main structure representing the server
#[derive(Debug)]
pub struct Server {
    /// Unique identifier
    pub uuid: Uuid,

    /// UUID of the owner's user
    pub owner: Uuid,

    /// Name of the server
    pub name: String,

    /// Description of the server
    pub description: String,
}
