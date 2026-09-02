#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::Display,
)]
pub struct MultipartValueLength(usize);

impl MultipartValueLength {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
