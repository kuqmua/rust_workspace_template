#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub struct BytesBodyBytes(bytes::Bytes);

impl BytesBodyBytes {
    #[cfg(test)]
    pub(crate) fn matches(&self, expected: &'static [u8]) -> bool {
        self.0 == bytes::Bytes::from_static(expected)
    }
}
