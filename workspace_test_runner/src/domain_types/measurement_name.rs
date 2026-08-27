#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MeasurementName(&'static str);
impl MeasurementName {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
