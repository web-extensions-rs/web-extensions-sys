use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Ping,
    GetOptionsInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Pong,
    OptionsInfo { version: String },
}
