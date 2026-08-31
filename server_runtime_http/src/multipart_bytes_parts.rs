#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct MultipartBytesParts(Vec<crate::multipart_bytes_part::MultipartBytesPart>);

impl MultipartBytesParts {
    pub(crate) const fn as_slice(&self) -> &[crate::multipart_bytes_part::MultipartBytesPart] {
        self.0.as_slice()
    }
}
