use serde::{Deserialize, Serialize};

pub type RequestId = usize;

pub const FIRST_REQUEST_ID: RequestId = 1;

pub const INITIAL_REQUEST_ID: RequestId = FIRST_REQUEST_ID - 1;

pub fn next_request_id(last_request_id: RequestId) -> RequestId {
    last_request_id.wrapping_add(1).max(FIRST_REQUEST_ID)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RequestHeader {
    pub client_token: Option<String>,
}

impl RequestHeader {
    pub const fn new() -> Self {
        Self { client_token: None }
    }

    pub fn into_response(self, request_id: RequestId) -> ResponseHeader {
        let Self { client_token } = self;
        ResponseHeader {
            client_token,
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
    pub client_token: Option<String>,
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
pub enum StreamingStartedStatus {
    /// The request has been accepted and will be processed.
    Accepted,
    /// The request has been rejected.
    Rejected {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamingFinishedStatus {
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
        status: StreamingStartedStatus,
    },
    /// A streaming item.
    ///
    /// The `item_count` serves as a 1-based index for (re-)ordering
    /// received item messages. Depending on the dispatching strategy
    /// messages might arrive out of order and receivers should account
    /// for that.
    Item {
        item_count: usize,
    },
    /// Request processing has finished.
    Finished {
        status: StreamingFinishedStatus,

        /// The total item count.
        ///
        /// Denotes the total number of item messages that have been sent.
        /// After the finished message has been sent no more item messages
        /// will be sent.
        ///
        /// In-flight item messages that have already been sent but have
        /// not yet been received might arrive later and out of order depending
        /// on the dispatching strategy.
        item_count: usize,
    },
}
