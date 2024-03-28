mod interface;
mod jobs;

mod structs;

pub use interface::*;

use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt;

/// The error type for [`axum::handler`] operations.
/// Constructable from [`anyhow::Error`] and convertable into [`Response`].
#[derive(Debug, Clone, Serialize)]
pub struct Error {
    reason: String,
    #[serde(skip_serializing)]
    status: StatusCode,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

/// The specialized Result type for [`axum::handler`] operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Creates the error from the given reason.
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Creates the error from the given reason.
    pub fn from_str(reason: &str) -> Self {
        Self::new(reason)
    }

    /// Creates the error from [`anyhow::Error`].
    pub fn from_any(inner: anyhow::Error) -> Self {
        Self::new(inner.to_string())
    }

    /// Replaces the status code of the error.
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
}

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(inner: E) -> Self {
        Self::from_any(inner.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // TODO: Tracing.
        (self.status, Json(self)).into_response()
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::new("Unhandled internal server error")
    }
}

/// Transforms any known [`tower::BoxError`] into the [`Error`].
pub async fn handle_box_error(err: tower::BoxError) -> Error {
    let error = if err.is::<tower::timeout::error::Elapsed>() {
        ("request took too long", StatusCode::REQUEST_TIMEOUT)
    } else {
        ("internal server error", StatusCode::INTERNAL_SERVER_ERROR)
    };

    Error::from_str(error.0).with_status(error.1)
}
