//! Reusable Actix Web JSON response macros with no application-crate dependencies.
//!
//! ```
//! use generate_actix_response::generate_ok_json_response;
//! let response = generate_ok_json_response!(vec!["hello"]);
//! assert_eq!(response.status().as_u16(), 200);
//! ```

use serde::{Deserialize, Serialize};

/// The shared JSON envelope used by all response macros.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct StandardResponse<T> {
    pub status: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[doc(hidden)]
pub mod __private {
    pub use actix_web;
}

/// Validates an integer HTTP status code, panicking if it is invalid.
///
/// Unlike a narrowing cast, this rejects negative values and overflow.
#[doc(hidden)]
pub fn status_code<T>(status: T) -> actix_web::http::StatusCode
where
    T: TryInto<u16>,
{
    status
        .try_into()
        .ok()
        .and_then(|value| actix_web::http::StatusCode::from_u16(value).ok())
        .expect("Invalid HTTP status code")
}

pub mod status_code;

#[macro_export]
macro_rules! generate_response_content {
    ($status:expr, $message:expr, $data:expr) => {{
        let status = $status;
        let code = $crate::status_code(status);
        let data = $crate::StandardResponse {
            status: i32::from(code.as_u16()),
            message: $message.to_string(),
            data: Some($data),
        };

        (code, data)
    }};
    ($status:expr, $message:expr) => {{
        let status = $status;
        let code = $crate::status_code(status);
        let data = $crate::StandardResponse {
            status: i32::from(code.as_u16()),
            message: $message.to_string(),
            data: None::<()>,
        };

        (code, data)
    }};
    ($status:expr) => {{
        let status = $status;
        let code = $crate::status_code(status);
        let data = $crate::StandardResponse {
            status: i32::from(code.as_u16()),
            message: code.canonical_reason().unwrap_or("Unknown").to_string(),
            data: None::<()>,
        };

        (code, data)
    }};
}

#[macro_export]
macro_rules! make_json_response {
    ($status:expr, $message:expr, $data:expr) => {{
        let (code, data) = $crate::generate_response_content!($status, $message, $data);
        $crate::__private::actix_web::HttpResponse::build(code).json(data)
    }};
    ($status:expr, $message:expr) => {{
        let (code, data) = $crate::generate_response_content!($status, $message);
        $crate::__private::actix_web::HttpResponse::build(code).json(data)
    }};
    ($status:expr) => {{
        let (code, data) = $crate::generate_response_content!($status);
        $crate::__private::actix_web::HttpResponse::build(code).json(data)
    }};
}
