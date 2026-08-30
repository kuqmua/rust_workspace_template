#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct DevelopmentIdentityCount(usize);

impl DevelopmentIdentityCount {
    pub(super) const fn increment(&mut self) {
        self.0 = self.0.saturating_add(constants_usize::ONE);
    }
}
