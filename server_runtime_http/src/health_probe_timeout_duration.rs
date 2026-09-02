#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct HealthProbeTimeoutDuration(std::time::Duration);

impl HealthProbeTimeoutDuration {
    pub(crate) const fn get(self) -> std::time::Duration {
        self.0
    }
}
