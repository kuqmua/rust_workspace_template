#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(super) struct FilterPlaceholderCount(usize);

impl FilterPlaceholderCount {
    pub(super) fn one() -> Self {
        Self::from(constants_usize::ONE)
    }
}
