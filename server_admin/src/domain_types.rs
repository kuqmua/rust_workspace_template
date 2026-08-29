// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
// Root-owned module compatibility wrappers.
pub mod auth {}
mod generated_auth {}
pub mod generated_tables {}
mod hash_opaque_token {}
mod rbac {}
mod maintenance {}
mod security {}
