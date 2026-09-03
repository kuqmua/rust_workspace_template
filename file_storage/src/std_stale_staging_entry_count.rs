#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_display::Display,
)]
pub struct StdStaleStagingEntryCount(usize);

impl StdStaleStagingEntryCount {
    pub(super) const fn increment(&mut self) {
        self.0 = self.0.saturating_add(constants_usize::ONE);
    }
}
