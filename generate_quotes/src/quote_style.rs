#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) struct QuoteStyle {
    pub(super) panic_id: crate::domain_types::QuotePanicId,
    pub(super) prefix: crate::domain_types::QuotePrefix,
    pub(super) quote_ch: crate::domain_types::QuoteChar,
}
