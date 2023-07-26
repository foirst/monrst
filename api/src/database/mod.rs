//! Databases and information storage

use anyhow::Result;
use async_trait::async_trait;

use crate::model::channel::Channel;

/// Representation of a database
#[async_trait]
pub trait Database: Sync + Send {
    async fn channel_fetch() -> Result<Channel>;
}
