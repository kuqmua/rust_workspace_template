#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub(super) struct StdRangeContains(bool);

impl StdRangeContains {
    pub(super) const fn get(self) -> bool {
        self.0
    }
}
