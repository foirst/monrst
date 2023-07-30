//! Databases and information storage

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::model::channel::message::Message;
use crate::model::channel::Channel;
use crate::model::server::Server;
use crate::model::user::User;

/// Representation of a database
#[async_trait]
pub trait Database: Sync + Send {
    /// Fetches a channel by its UUID
    async fn channel_fetch(&self, uuid: Uuid) -> Result<Channel>;

    /// Inserts a new channel
    async fn channel_insert(&mut self, channel: Channel) -> Result<()>;

    /// Deletes a channel by its UUID
    async fn channel_delete(&mut self, uuid: Uuid) -> Result<()>;

    /// Fetches a message by its UUID
    async fn message_fetch(&self, uuid: Uuid) -> Result<Message>;

    /// Inserts a new message
    async fn message_insert(&mut self, message: Message) -> Result<()>;

    /// Deletes a message by its UUID
    async fn message_delete(&mut self, uuid: Uuid) -> Result<()>;

    /// Fetches a server by its UUID
    async fn server_fetch(&self, uuid: Uuid) -> Result<Server>;

    /// Inserts a new server
    async fn server_insert(&mut self, server: Server) -> Result<()>;

    /// Deletes a server by its UUID
    async fn server_delete(&mut self, uuid: Uuid) -> Result<()>;

    /// Fetches a user by its UUID
    async fn user_fetch(&self, uuid: Uuid) -> Result<User>;

    /// Inserts a new user
    async fn user_insert(&mut self, user: User) -> Result<()>;

    /// Deletes a user by its UUID
    async fn user_delete(&mut self, uuid: Uuid) -> Result<()>;
}
