#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct ContractStr(&'static str);
impl From<ContractStr> for String {
    fn from(contract_str: ContractStr) -> Self {
        Self::from(contract_str.0)
    }
}
