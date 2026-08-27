#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct ContractI64(i64);
impl ContractI64 {
    #[must_use]
    pub fn i16_max() -> Self {
        Self::from(32_767i64)
    }
    #[must_use]
    pub fn i16_min() -> Self {
        Self::from(-32_768i64)
    }
    #[must_use]
    pub fn i32_max() -> Self {
        Self::from(2_147_483_647i64)
    }
    #[must_use]
    pub fn i32_min() -> Self {
        Self::from(-2_147_483_648i64)
    }
    #[must_use]
    pub fn max() -> Self {
        Self::from(i64::MAX)
    }
    #[must_use]
    pub fn min() -> Self {
        Self::from(i64::MIN)
    }
}
