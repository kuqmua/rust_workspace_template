#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct HealthDatabaseAvailable(bool);

impl HealthDatabaseAvailable {
    pub(crate) const fn is_available(self) -> bool {
        self.0
    }
}
