#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "request_origin_allowed/request_origin_allowed.rs"]
mod request_origin_allowed;

pub use request_origin_allowed::request_origin_allowed;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct RequestOriginAllowed(bool);
