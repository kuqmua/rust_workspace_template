#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct HttpHeaderTextMaximumBytes(pub(super) super::HttpHeaderTextMaximumBytesNonZeroUsize);

impl From<HttpHeaderTextMaximumBytes> for usize {
    fn from(value: HttpHeaderTextMaximumBytes) -> Self {
        value.0.0.get()
    }
}

impl TryFrom<usize> for HttpHeaderTextMaximumBytes {
    type Error = super::HttpHeaderTextMaximumBytesError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(super::HttpHeaderTextMaximumBytesNonZeroUsize::from)
            .map(Self)
            .ok_or(super::HttpHeaderTextMaximumBytesError)
    }
}
