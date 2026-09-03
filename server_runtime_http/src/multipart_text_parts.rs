#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct MultipartTextParts(Vec<crate::multipart_text_part::MultipartTextPart>);

impl MultipartTextParts {
    pub(crate) const fn as_slice(&self) -> &[crate::multipart_text_part::MultipartTextPart] {
        self.0.as_slice()
    }
}
