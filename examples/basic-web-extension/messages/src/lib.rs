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
}

/// Port-local response message.
#[derive(Debug, Serialize, Deserialize)]
pub enum PortResponse {
    Pong,
}
