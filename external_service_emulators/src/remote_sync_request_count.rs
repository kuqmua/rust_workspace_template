#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::FromInner,
)]
pub struct RemoteSyncRequestCount(usize);

impl RemoteSyncRequestCount {
    pub(super) const fn increment(&mut self) {
        self.0 = self.0.saturating_add(constants_usize::ONE);
    }
}
