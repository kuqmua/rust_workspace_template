#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct HealthProbeTimeoutDuration(std::time::Duration);

impl HealthProbeTimeoutDuration {
    pub(crate) const fn get(self) -> std::time::Duration {
        self.0
    }
}
