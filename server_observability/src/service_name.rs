#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ServiceName(&'static str);
impl ServiceName {
    pub(super) const fn get(self) -> &'static str {
        self.0
    }
}
