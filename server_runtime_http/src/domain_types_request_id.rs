#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
// Root-owned module compatibility wrappers.
mod http_header_to_str_error {}
mod request_id {}
mod request_id_try_from_http_header_value_error {}
mod request_id_try_from_string_error {}
