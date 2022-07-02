use serde::{Deserialize, Serialize};

/// Global request message.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    GetOptionsInfo,
}

/// Global response message.
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    OptionsInfo { version: String },
}

/// Port-local request message.
#[derive(Debug, Serialize, Deserialize)]
pub enum PortRequest {
    Ping,
    StreamingExample,
}

/// Port-local response message.
#[derive(Debug, Serialize, Deserialize)]
pub enum PortResponse {
    Pong,
    StreamingExample(StreamResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamResponse {
    /// The request was accepted
    Accepted,
    /// A streaming item
    Item { item_number: usize },
    /// The operation has finished.
    Finished,
}
