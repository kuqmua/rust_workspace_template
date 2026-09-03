#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
)]
pub struct FirstCommaStripped(bool);
impl std::ops::Not for FirstCommaStripped {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
