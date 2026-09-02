#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::Display,
)]
pub struct BoundedReadMaximumBytes(usize);

impl BoundedReadMaximumBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
