#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct FilterPlaceholderCount(usize);

impl FilterPlaceholderCount {
    pub(super) fn one() -> Self {
        Self::from(constants_usize::ONE)
    }
}
