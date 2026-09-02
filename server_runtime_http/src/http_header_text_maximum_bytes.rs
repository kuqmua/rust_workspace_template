#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct HttpHeaderTextMaximumBytes(std::num::NonZeroUsize);

impl From<HttpHeaderTextMaximumBytes> for usize {
    fn from(http_header_text_maximum_bytes: HttpHeaderTextMaximumBytes) -> Self {
        http_header_text_maximum_bytes.0.get()
    }
}

impl TryFrom<usize> for HttpHeaderTextMaximumBytes {
    type Error = crate::http_header_text_maximum_bytes_error::HttpHeaderTextMaximumBytesError;

    fn try_from(usize: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(usize).map(Self).ok_or(
            crate::http_header_text_maximum_bytes_error::HttpHeaderTextMaximumBytesError::Zero,
        )
    }
}
