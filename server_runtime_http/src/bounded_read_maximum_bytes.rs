#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::Display,
)]
pub struct BoundedReadMaximumBytes(usize);

impl BoundedReadMaximumBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
