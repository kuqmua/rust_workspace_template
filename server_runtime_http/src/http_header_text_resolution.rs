#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum HttpHeaderTextResolution<'header> {
    ExceedsMaximumBytes {
        actual_bytes: crate::http_header_text_bytes::HttpHeaderTextBytes,
    },
    InvalidText,
    Missing,
    Value(crate::http_header_text_ref::HttpHeaderTextRef<'header>),
}
