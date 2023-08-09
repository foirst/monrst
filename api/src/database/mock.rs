//! Mock database
//!
//! It should only be used in tests as **ALL DATAS CONTAINED ARE DROP AT THE END OF THE EXECUTION**

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use uuid::Uuid;

use crate::database;
use crate::model::channel::message::Message;
use crate::model::channel::Channel;
use crate::model::user::User;

/// Integrated database
#[derive(Debug, Default)]
pub struct Database {
    /// Existing channels
    channels: HashMap<Uuid, Channel>,

    /// Existing messages
    messages: HashMap<Uuid, Message>,

    /// Existing users
    users: HashMap<Uuid, User>,
}

#[async_trait]
impl database::Interface for Database {
    #[inline]
    async fn channel_fetch(&self, uuid: Uuid) -> Result<Channel> {
        self.channels
            .get(&uuid)
            .map_or_else(|| Err(anyhow!("Channel not found")), |channel| Ok(channel.clone()))
    }

    #[inline]
    async fn channel_insert(&mut self, channel: Channel) -> Result<()> {
        if let Ok(channel) = self.channel_fetch(channel.uuid()).await {
            Err(anyhow!("The UUID {} is already used by the channel: {:?}", channel.uuid(), channel))
        } else {
            match channel {
                Channel::DirectMessage {
                    users: (user1, user2),
                    ..
                } => {
                    if self.user_fetch(user1).await.is_ok() && self.user_fetch(user2).await.is_ok() {
                        self.channels.insert(channel.uuid(), channel);
                        Ok(())
                    } else {
                        Err(anyhow!(
                            "Cannot add a direct message channel with an unknown user (user 1: {} / user 2: {})",
                            user1,
                            user2
                        ))
                    }
                },
            }
        }
    }

    #[inline]
    #[allow(clippy::needless_collect)]
    async fn channel_delete(&mut self, uuid: Uuid) -> Result<()> {
        match self.channels.remove(&uuid) {
            Some(Channel::DirectMessage { .. }) => {
                for message in self
                    .messages
                    .values()
                    .filter(|&message| message.channel == uuid)
                    .map(ToOwned::to_owned)
                    .collect::<Vec<Message>>()
                {
                    self.message_delete(message.uuid).await?;
                }
                Ok(())
            },
            None => Err(anyhow!("No channel with UUID {uuid} exists")),
        }
    }

    #[inline]
    async fn message_fetch(&self, uuid: Uuid) -> Result<Message> {
        self.messages
            .get(&uuid)
            .map_or_else(|| Err(anyhow!("Message not found")), |message| Ok(message.clone()))
    }

    #[inline]
    async fn message_insert(&mut self, message: Message) -> Result<()> {
        if let Ok(channel) = self.message_fetch(message.uuid).await {
            Err(anyhow!("The UUID {} is already used by the message: {:?}", message.uuid, channel))
        } else if self.user_fetch(message.author).await.is_ok() && self.channel_fetch(message.channel).await.is_ok() {
            self.messages.insert(message.uuid, message);
            Ok(())
        } else {
            Err(anyhow!(
                "Cannot send a message from an unknown user (author: {}) or in an unknown channel (channel : {})",
                message.author,
                message.channel
            ))
        }
    }

    #[inline]
    async fn message_delete(&mut self, uuid: Uuid) -> Result<()> {
        match self.messages.remove(&uuid) {
            Some(_) => Ok(()),
            None => Err(anyhow!("No message with UUID {uuid} exists")),
        }
    }

    #[inline]
    async fn user_fetch(&self, uuid: Uuid) -> Result<User> {
        self.users
            .get(&uuid)
            .map_or_else(|| Err(anyhow!("User not found")), |user| Ok(user.clone()))
    }

    #[inline]
    async fn user_insert(&mut self, user: User) -> Result<()> {
        if let Ok(user) = self.user_fetch(user.uuid).await {
            Err(anyhow!("The UUID {} is already used by the user: {:?}", user.uuid, user))
        } else {
            self.users.insert(user.uuid, user);
            Ok(())
        }
    }

    #[inline]
    async fn user_delete(&mut self, uuid: Uuid) -> Result<()> {
        match self.users.remove(&uuid) {
            Some(_) => Ok(()),
            None => Err(anyhow!("No user with UUID {uuid} exists")),
        }
    }
}

#[cfg(test)]
mod test {
    use async_std::sync::Mutex;
    use spin::Lazy;

    use super::Database;
    use crate::database::Interface;
    use crate::model::channel::message::Message;
    use crate::model::channel::Channel;
    use crate::model::user::User;

    /// Integrated database used for the tests
    static DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| Mutex::new(Database::default()));

    #[async_std::test]
    async fn channels_and_messages() {
        let mut database = DATABASE.lock().await;

        let alice = User::new("Alice");
        let bob = User::new("Bob");
        let direct_messages = Channel::new_direct_message(alice.uuid, bob.uuid);
        let direct_messages_uuid = direct_messages.uuid();
        let message_alice = Message::new(direct_messages_uuid, alice.uuid, "Hello".to_owned());
        let message_alice_uuid = message_alice.uuid;
        let message_bob = Message::new(direct_messages_uuid, bob.uuid, "world!".to_owned());

        assert!(database.user_insert(alice).await.is_ok());
        assert!(database.user_insert(bob).await.is_ok());
        assert!(database.channel_insert(direct_messages).await.is_ok());
        assert!(database.message_insert(message_alice).await.is_ok());
        assert!(database.message_insert(message_bob).await.is_ok());

        assert!(database.message_fetch(message_alice_uuid).await.is_ok());
        assert!(database.channel_delete(direct_messages_uuid).await.is_ok());
        assert!(database.message_fetch(message_alice_uuid).await.is_err());
    }

    #[async_std::test]
    async fn users() {
        let mut database = DATABASE.lock().await;

        let alice = User::new("Alice");
        let bob = User::new("Bob");
        let alice_uuid = alice.uuid;
        let bob_uuid = bob.uuid;

        assert!(database.user_insert(alice.clone()).await.is_ok());
        assert!(database.user_insert(alice).await.is_err());
        assert!(database.user_insert(bob.clone()).await.is_ok());
        assert_eq!(database.user_fetch(alice_uuid).await.unwrap().username, "Alice");
        assert!(database.user_delete(bob_uuid).await.is_ok());
    }
}
