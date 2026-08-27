#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpHeaderTextResolution<'header> {
    ExceedsMaximumBytes {
        actual_bytes: super::HttpHeaderTextBytes,
    },
    InvalidText,
    Missing,
    Value(super::HttpHeaderTextRef<'header>),
}
