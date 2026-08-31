#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpAcceptHeaderMaximumBytes(usize);

impl HttpAcceptHeaderMaximumBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
