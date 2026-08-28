#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]

pub use crate::http_header_to_str_error::HttpHeaderToStrError;
pub use crate::request_id::RequestId;
pub use crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError;
pub use crate::request_id_try_from_string_error::RequestIdTryFromStringError;

// Root-owned module compatibility wrappers.
mod http_header_to_str_error {
    pub use crate::http_header_to_str_error::*;
}
mod request_id {
    pub use crate::request_id::*;
}
mod request_id_try_from_http_header_value_error {
    pub use crate::request_id_try_from_http_header_value_error::*;
}
mod request_id_try_from_string_error {
    pub use crate::request_id_try_from_string_error::*;
}
