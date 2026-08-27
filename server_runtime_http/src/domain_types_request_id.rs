#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "domain_types_request_id/http_header_to_str_error.rs"]
mod http_header_to_str_error;
#[path = "domain_types_request_id/request_id.rs"]
mod request_id;
#[path = "domain_types_request_id/request_id_try_from_http_header_value_error.rs"]
mod request_id_try_from_http_header_value_error;
#[path = "domain_types_request_id/request_id_try_from_string_error.rs"]
mod request_id_try_from_string_error;

pub use http_header_to_str_error::HttpHeaderToStrError;
pub use request_id::RequestId;
pub use request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError;
pub use request_id_try_from_string_error::RequestIdTryFromStringError;
