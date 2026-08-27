#[path = "quote_style/quote_style.rs"]
mod quote_style;

pub(crate) use quote_style::quote_style;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) struct QuoteStyle {
    pub(super) panic_id: super::QuotePanicId,
    pub(super) prefix: super::QuotePrefix,
    pub(super) quote_ch: super::QuoteChar,
}
