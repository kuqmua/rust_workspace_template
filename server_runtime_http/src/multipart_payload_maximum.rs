#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct MultipartPayloadMaximum(usize);

impl MultipartPayloadMaximum {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
