#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct BytesBodyBytes(bytes::Bytes);

impl BytesBodyBytes {
    #[cfg(test)]
    pub(crate) fn matches(&self, expected: &'static [u8]) -> bool {
        self.0 == bytes::Bytes::from_static(expected)
    }
}
