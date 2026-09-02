#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct MultipartBytesParts(Vec<crate::multipart_bytes_part::MultipartBytesPart>);

impl MultipartBytesParts {
    pub(crate) const fn as_slice(&self) -> &[crate::multipart_bytes_part::MultipartBytesPart] {
        self.0.as_slice()
    }
}
