use http::StatusCode;

use crate::{ErrorKind, ErrorStatus};

impl From<ErrorStatus> for StatusCode {
    fn from(value: ErrorStatus) -> Self {
        match value.inner.kind {
            ErrorKind::BadRequest => StatusCode::BAD_REQUEST,
            ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorKind::Forbidden => StatusCode::FORBIDDEN,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ErrorKind::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            ErrorKind::Gone => StatusCode::GONE,
            ErrorKind::PreconditionFailed => StatusCode::PRECONDITION_FAILED,
            ErrorKind::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            ErrorKind::Unsupported => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ErrorKind::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorKind::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            ErrorKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ErrorKind::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl From<ErrorStatus> for (StatusCode, String) {
    fn from(value: ErrorStatus) -> Self {
        let code = match value.inner.kind {
            ErrorKind::BadRequest => StatusCode::BAD_REQUEST,
            ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorKind::Forbidden => StatusCode::FORBIDDEN,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ErrorKind::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            ErrorKind::Gone => StatusCode::GONE,
            ErrorKind::PreconditionFailed => StatusCode::PRECONDITION_FAILED,
            ErrorKind::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            ErrorKind::Unsupported => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ErrorKind::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorKind::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            ErrorKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ErrorKind::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        };
        (code, value.inner.context)
    }
}
