#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
)]
pub struct BytesBodyBytes(bytes::Bytes);

impl BytesBodyBytes {
    #[cfg(test)]
    pub(crate) fn matches(&self, expected: &'static [u8]) -> bool {
        self.0 == bytes::Bytes::from_static(expected)
    }
}
