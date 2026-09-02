#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_naming::EnumWithUnitFieldsToUpperSnakeCaseStr,
)]
pub enum RouteMethod {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}
impl RouteMethod {
    #[must_use]
    pub fn as_str(self) -> crate::contract_str::ContractStr {
        crate::contract_str::ContractStr::from(self.as_upper_snake_case_str())
    }
}
