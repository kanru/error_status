use std::error::Error as StdError;
use std::fmt::Display;

use crate::{ErrorKind, ErrorStatus, Inner};

impl ErrorStatus {
    fn from<E>(value: (ErrorKind, String, E)) -> Self
    where
        E: StdError + Sync + Send + 'static,
    {
        ErrorStatus {
            inner: Box::new(Inner {
                kind: value.0,
                context: value.1,
                source: Box::new(value.2),
            }),
        }
    }
}

macro_rules! trait_method {
    ($kind:expr, $method:ident) => {
        fn $method<C>(self, context: C) -> Result<T, ErrorStatus>
        where
            Self: Sized,
            C: Display + Send + Sync + 'static,
        {
            self.msg($kind, context)
        }
    };
}
macro_rules! trait_method_lazy {
    ($kind:expr, $method:ident) => {
        fn $method<C, F>(self, f: F) -> Result<T, ErrorStatus>
        where
            Self: Sized,
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
        {
            self.msg_lazy($kind, f)
        }
    };
}

/// Methods to construct different [`ErrorStatus`]s from `Result`
#[allow(private_bounds)]
pub trait ResultExt<T, E>: Sealed<T, E> {
    trait_method!(ErrorKind::BadRequest, bad_request);
    trait_method!(ErrorKind::Unauthorized, unauthorized);
    trait_method!(ErrorKind::Forbidden, forbidden);
    trait_method!(ErrorKind::NotFound, not_found);
    trait_method!(ErrorKind::MethodNotAllowed, method_not_allowed);
    trait_method!(ErrorKind::RequestTimeout, request_timeout);
    trait_method!(ErrorKind::Gone, gone);
    trait_method!(ErrorKind::PreconditionFailed, precondition_failed);
    trait_method!(ErrorKind::PayloadTooLarge, content_too_large);
    trait_method!(ErrorKind::Unsupported, unsupported);
    trait_method!(ErrorKind::UnprocessableEntity, unprocessable_content);
    trait_method!(ErrorKind::TooManyRequests, too_many_requests);
    trait_method!(ErrorKind::InternalError, internal_error);
    trait_method!(ErrorKind::NotImplemented, not_implemented);
    trait_method!(ErrorKind::Unavailable, unavailable);

    trait_method_lazy!(ErrorKind::BadRequest, bad_request_lazy);
    trait_method_lazy!(ErrorKind::Unauthorized, unauthorized_lazy);
    trait_method_lazy!(ErrorKind::Forbidden, forbidden_lazy);
    trait_method_lazy!(ErrorKind::NotFound, not_found_lazy);
    trait_method_lazy!(ErrorKind::MethodNotAllowed, method_not_allowed_lazy);
    trait_method_lazy!(ErrorKind::RequestTimeout, request_timeout_lazy);
    trait_method_lazy!(ErrorKind::Gone, gone_lazy);
    trait_method_lazy!(ErrorKind::PreconditionFailed, precondition_failed_lazy);
    trait_method_lazy!(ErrorKind::PayloadTooLarge, content_too_large_lazy);
    trait_method_lazy!(ErrorKind::Unsupported, unsupported_lazy);
    trait_method_lazy!(ErrorKind::UnprocessableEntity, unprocessable_content_lazy);
    trait_method_lazy!(ErrorKind::TooManyRequests, too_many_requests_lazy);
    trait_method_lazy!(ErrorKind::InternalError, internal_error_lazy);
    trait_method_lazy!(ErrorKind::NotImplemented, not_implemented_lazy);
    trait_method_lazy!(ErrorKind::Unavailable, unavailable_lazy);
}

trait Sealed<T, E> {
    fn msg<C>(self, kind: ErrorKind, context: C) -> Result<T, ErrorStatus>
    where
        C: Display + Send + Sync + 'static;
    fn msg_lazy<C, F>(self, kind: ErrorKind, f: F) -> Result<T, ErrorStatus>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> ResultExt<T, E> for Result<T, E> where E: StdError + Sync + Send + 'static {}

impl<T, E> Sealed<T, E> for Result<T, E>
where
    E: StdError + Sync + Send + 'static,
{
    fn msg<C>(self, kind: ErrorKind, context: C) -> Result<T, ErrorStatus>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|e| ErrorStatus::from((kind, context.to_string(), e)))
    }
    fn msg_lazy<C, F>(self, kind: ErrorKind, f: F) -> Result<T, ErrorStatus>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| {
            let context = f();
            ErrorStatus::from((kind, context.to_string(), e))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Error as IoError, ErrorKind};

    use super::ResultExt;

    #[test]
    fn msg() {
        let io_error: Result<(), IoError> = Err(ErrorKind::Other.into());
        let error_msg = io_error.as_ref().unwrap_err().to_string();
        let error = io_error.internal_error("context").unwrap_err();
        assert!(matches!(error.inner.kind, crate::ErrorKind::InternalError));
        assert_eq!(error.inner.context, "context");
        assert_eq!(error.inner.source.to_string(), error_msg);
    }

    #[test]
    fn with_msg() {
        let io_error: Result<(), IoError> = Err(ErrorKind::Other.into());
        let error_msg = io_error.as_ref().unwrap_err().to_string();
        let error = io_error.internal_error_lazy(|| "context").unwrap_err();
        assert!(matches!(error.inner.kind, crate::ErrorKind::InternalError));
        assert_eq!(error.inner.context, "context");
        assert_eq!(error.inner.source.to_string(), error_msg);
    }
}
