#![forbid(unsafe_code)]

//! A generic Error container with HTTP 4xx or 5xx status code semantics and
//! string context.
//!
//! This crate tries to find the balance between too many error variants and
//! zero context.
//!
//! # Features
//!
//! * **http**: when **http** is enabled, [`ErrorStatus`] can be converted to
//!   `StatusCode` or `(StatusCode, String)` for easier integration with HTTP
//!   services.
//!
//! # Examples
//!
//! ```should_panic
//! use std::io::{self, ErrorKind};
//!
//! use anyhow::Result;
//! use error_status::ResultExt;
//!
//! fn find_file() -> Result<(), io::Error> {
//!     Err(ErrorKind::NotFound.into())
//! }
//!
//! fn main() -> Result<()> {
//!     find_file()
//!         .not_found("Failed to read file")
//!         .internal_error("Config file is not available")?;
//!     Ok(())
//! }
//! ```

#[cfg(feature = "http")]
mod http;
mod result;

pub use result::ResultExt;

use std::error::Error as StdError;
use std::fmt::{Debug, Display};

/// Error with context
pub struct ErrorStatus {
    inner: Box<Inner>,
}

struct Inner {
    kind: ErrorKind,
    context: String,
    source: Box<dyn StdError + Sync + Send + 'static>,
}

impl ErrorStatus {
    pub fn kind(&self) -> ErrorKind {
        self.inner.kind
    }
    pub fn context(&self) -> &str {
        &self.inner.context
    }
}

/// Error code modeled after HTTP StatusCode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    // ==== Client Errors ====
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    RequestTimeout,
    Gone,
    PreconditionFailed,
    PayloadTooLarge,
    Unsupported,
    UnprocessableEntity,
    TooManyRequests,

    // ==== Server Errors ====
    InternalError,
    NotImplemented,
    Unavailable,
}

impl StdError for ErrorStatus {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.source.as_ref())
    }
}

impl Display for ErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.inner.kind, self.inner.context)
    }
}

impl Debug for ErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.inner.kind)
            .field("source", &self.inner.source)
            .finish()
    }
}
