//! Structures related to clients

use anyhow::{anyhow, Result};
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
    ///
    /// It is [`None`] if the kind is unknown.
    pub kind: Option<Kind>,
}

impl Client {
    /// Creates a new client
    #[inline]
    #[must_use]
    pub fn new(kind: Option<Kind>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            attached_users: vec![],
            kind,
        }
    }

    /// Attaches a user to the given client
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](anyhow::Error) if the given user is already attached to this client
    #[inline]
    pub fn attach(&mut self, user: Uuid) -> Result<()> {
        if self.attached_users.contains(&user) {
            Err(anyhow!("The user is already attached to this client"))
        } else {
            self.attached_users.push(user);
            Ok(())
        }
    }
}
