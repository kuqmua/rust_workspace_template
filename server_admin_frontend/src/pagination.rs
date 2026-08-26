#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    reason = "pagination accessors follow calculation order, the deterministic test helper converts raw inputs in place, and the shared constructor has one SSR production caller plus focused unit tests"
)]

#[path = "admin_page_nav_disabled.rs"]
pub(crate) mod admin_page_nav_disabled;
#[path = "admin_page_range.rs"]
pub(crate) mod admin_page_range;
