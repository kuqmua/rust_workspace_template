#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(super) struct AllowOriginSuffix(bool);

impl AllowOriginSuffix {
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}
