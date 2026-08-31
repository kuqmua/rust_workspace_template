#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct MultipartPayloadMaximum(usize);

impl MultipartPayloadMaximum {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
