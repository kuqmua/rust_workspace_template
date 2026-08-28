#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::shadow_reuse,
    reason = "pagination accessors follow calculation order, the deterministic test helper converts raw inputs in place, and the shared constructor has one SSR production caller plus focused unit tests"
)]

// Root-owned module compatibility wrappers.
pub(crate) mod admin_page_nav_disabled {
    pub use crate::admin_page_nav_disabled::*;
}
pub(crate) mod admin_page_range {
    pub use crate::admin_page_range::*;
}
