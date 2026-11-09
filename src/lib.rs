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

/// Validates the numeric range accepted by Actix's HTTP status type.
#[doc(hidden)]
pub const fn status_code(status: u16) -> u16 {
    assert!(status >= 100 && status <= 999, "Invalid HTTP status code");
    status
}

pub mod status_code;

/// Builds a response envelope using a compile-time numeric status (`u16`).
/// Message and data expressions are evaluated at runtime.
///
/// ```
/// use generate_actix_response::generate_response_content;
/// const ACCEPTED: u16 = 202;
/// let message = String::from("Queued");
/// let (code, body) = generate_response_content!(ACCEPTED, message, vec![42]);
/// assert_eq!(code.as_u16(), 202);
/// assert_eq!(body.data, Some(vec![42]));
/// ```
///
/// Invalid statuses are compilation errors in every argument form:
///
/// ```compile_fail,E0080
/// let _ = generate_actix_response::generate_response_content!(99);
/// ```
/// ```compile_fail,E0080
/// let _ = generate_actix_response::generate_response_content!(1000, "Invalid");
/// ```
/// ```compile_fail,E0080
/// let _ = generate_actix_response::make_json_response!(0, "Invalid", 42);
/// ```
/// Negative and overflowing literals are rejected without narrowing casts:
///
/// ```compile_fail
/// let _ = generate_actix_response::generate_response_content!(-1);
/// ```
/// ```compile_fail
/// let _ = generate_actix_response::generate_response_content!(65736);
/// ```
///
/// Runtime status expressions are not supported by these macros:
///
/// ```compile_fail,E0435
/// let status = 200;
/// let _ = generate_actix_response::generate_response_content!(status);
/// ```
#[macro_export]
macro_rules! generate_response_content {
    ($status:expr, $message:expr, $data:expr) => {{
        let status = const { $crate::status_code($status) };
        let code = $crate::__private::actix_web::http::StatusCode::from_u16(status)
            .expect("HTTP status code was validated at compile time");
        let data = $crate::StandardResponse {
            status: i32::from(code.as_u16()),
            message: $message.to_string(),
            data: Some($data),
        };

        (code, data)
    }};
    ($status:expr, $message:expr) => {{
        let status = const { $crate::status_code($status) };
        let code = $crate::__private::actix_web::http::StatusCode::from_u16(status)
            .expect("HTTP status code was validated at compile time");
        let data = $crate::StandardResponse {
            status: i32::from(code.as_u16()),
            message: $message.to_string(),
            data: None::<()>,
        };

        (code, data)
    }};
    ($status:expr) => {{
        let status = const { $crate::status_code($status) };
        let code = $crate::__private::actix_web::http::StatusCode::from_u16(status)
            .expect("HTTP status code was validated at compile time");
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
