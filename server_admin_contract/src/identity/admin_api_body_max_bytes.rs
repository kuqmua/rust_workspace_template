#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminApiBodyMaxBytes(usize);

#[path = "admin_api_body_max_bytes/admin_api_body_max_bytes.rs"]
mod admin_api_body_max_bytes;

pub(crate) use admin_api_body_max_bytes::ADMIN_API_BODY_MAX_BYTES_VALUE;
pub use admin_api_body_max_bytes::admin_api_body_max_bytes;
