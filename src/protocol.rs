//! Everything linked with the protocol used through websocket streams

/// Kinds of format supported by the protocol
#[derive(Debug)]
pub enum Format {
    /// Bytes-base communication
    Binary,

    /// JSON-based communication
    Json,
}

/// Protocol configuration
#[derive(Debug)]
pub struct Configuration {
    /// Format used
    pub format: Format,
}

impl Configuration {
    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }
}
