#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct RemoteSyncRequestCount(usize);

impl RemoteSyncRequestCount {
    pub(super) const fn increment(&mut self) {
        self.0 = self.0.saturating_add(constants_usize::ONE);
    }
}
