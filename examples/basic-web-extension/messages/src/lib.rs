use serde::{Deserialize, Serialize};

pub type RequestId = usize;

pub const FIRST_REQUEST_ID: RequestId = 1;

pub const INITIAL_REQUEST_ID: RequestId = FIRST_REQUEST_ID - 1;

pub fn next_request_id(last_request_id: RequestId) -> RequestId {
    last_request_id.wrapping_add(1).max(FIRST_REQUEST_ID)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RequestHeader {
    pub client_id: Option<String>,
}

impl RequestHeader {
    pub const fn new() -> Self {
        Self { client_id: None }
    }

    pub fn into_response(self, request_id: RequestId) -> ResponseHeader {
        let Self { client_id } = self;
        ResponseHeader {
            client_id,
            request_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    pub header: RequestHeader,
    pub payload: T,
}

impl<T> Request<T> {
    pub const fn new(payload: T) -> Self {
        Self {
            header: RequestHeader::new(),
            payload,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseHeader {
    pub client_id: Option<String>,
    pub request_id: RequestId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub header: ResponseHeader,
    pub payload: T,
}

/// App request message.
#[derive(Debug, Serialize, Deserialize)]
pub enum AppRequestPayload {
    GetOptionsInfo,
}

pub type AppRequest = Request<AppRequestPayload>;

/// App response message.
#[derive(Debug, Serialize, Deserialize)]
pub enum AppResponsePayload {
    OptionsInfo { version: String },
}

pub type AppResponse = Response<AppResponsePayload>;

/// Port-local request message.
#[derive(Debug, Serialize, Deserialize)]
pub enum PortRequestPayload {
    Ping,
    StartStreaming { num_items: usize },
}

pub type PortRequest = Request<PortRequestPayload>;

/// Port-local response message.
#[derive(Debug, Serialize, Deserialize)]
pub enum PortResponsePayload {
    Pong,
    Streaming(StreamingResponsePayload),
}

pub type PortResponse = Response<PortResponsePayload>;

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamingStarted {
    /// The request has been accepted and will be processed.
    Accepted,
    /// The request has been rejected.
    Rejected {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamingFinished {
    /// Processing has been completed.
    Completed,
    /// Processing has been aborted prematurely.
    Aborted {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamingResponsePayload {
    Started {
        started: StreamingStarted,
    },
    /// A streaming item.
    ///
    /// Items are sequentially numbered by a 0-based index
    /// and might be received out of order.
    Item {
        index: usize,
    },
    /// Request processing has finished.
    Finished {
        finished: StreamingFinished,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_item_index: Option<usize>,
    },
}
