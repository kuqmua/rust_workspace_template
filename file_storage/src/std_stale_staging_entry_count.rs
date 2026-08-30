#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
    newtype::IntoInnerFrom,
    newtype::Display,
)]
pub struct StdStaleStagingEntryCount(usize);

impl StdStaleStagingEntryCount {
    pub(super) const fn increment(&mut self) {
        self.0 = self.0.saturating_add(constants_usize::ONE);
    }
}
