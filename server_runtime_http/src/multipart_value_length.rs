#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::Display,
)]
pub struct MultipartValueLength(usize);

impl MultipartValueLength {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
