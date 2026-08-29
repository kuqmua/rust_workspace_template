#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct HttpHeaderTextMaximumBytes(pub(super) std::num::NonZeroUsize);

impl From<HttpHeaderTextMaximumBytes> for usize {
    fn from(value: HttpHeaderTextMaximumBytes) -> Self {
        value.0.get()
    }
}

impl TryFrom<usize> for HttpHeaderTextMaximumBytes {
    type Error = crate::http_header_text_maximum_bytes_error::HttpHeaderTextMaximumBytesError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or(crate::http_header_text_maximum_bytes_error::HttpHeaderTextMaximumBytesError)
    }
}
