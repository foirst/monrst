//! Users, bots and clients

pub mod client;

use uuid::Uuid;

/// Main structure for users
///
/// A user corresponds to an account owned by a physical or moral person: it is not a bot
#[derive(Debug)]
pub struct User {
    /// Unique identifier
    pub uuid: Uuid,

    /// Username
    pub username: String,

    /// Discriminator
    ///
    /// This is used to differentiate several users with a same username
    ///
    /// It is represented with an hexadecimal format
    pub discriminator: [u8; 4],

    /// Is this user currently online ?
    pub online: bool,
}
