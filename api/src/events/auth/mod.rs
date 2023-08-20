//! Authentification of users and bots on clients

use anyhow::Result;

use crate::model::user::client::Client;
use crate::model::user::User;

/// Provides a method to authenticate a user
pub trait Authentifier {
    /// Authenticates a user
    ///
    /// Returns the [`User`] authenticated in case of success
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](anyhow::Error) if the authentication fails
    fn auth(username: &str, client: &Client) -> Result<User>;
}
