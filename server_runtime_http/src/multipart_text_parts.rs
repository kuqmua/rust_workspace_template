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
pub(super) struct MultipartTextParts(Vec<crate::multipart_text_part::MultipartTextPart>);

impl MultipartTextParts {
    pub(crate) const fn as_slice(&self) -> &[crate::multipart_text_part::MultipartTextPart] {
        self.0.as_slice()
    }
}
