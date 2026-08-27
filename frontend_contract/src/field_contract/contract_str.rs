#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ContractStr(&'static str);
impl From<ContractStr> for String {
    fn from(value: ContractStr) -> Self {
        Self::from(value.0)
    }
}
