#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct StdBool(bool);
impl StdBool {
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}
