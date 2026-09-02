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
pub(super) struct MultipartTextParts(Vec<crate::multipart_text_part::MultipartTextPart>);

impl MultipartTextParts {
    pub(crate) const fn as_slice(&self) -> &[crate::multipart_text_part::MultipartTextPart] {
        self.0.as_slice()
    }
}
