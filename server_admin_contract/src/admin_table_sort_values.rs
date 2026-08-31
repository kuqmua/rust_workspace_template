#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_constructor::New)]
pub(super) struct AdminTableSortValues {
    key: frontend_contract::contract_str::ContractStr,
    label: frontend_contract::contract_str::ContractStr,
}
impl AdminTableSortValues {
    pub(super) const fn into_parts(
        self,
    ) -> (
        frontend_contract::contract_str::ContractStr,
        frontend_contract::contract_str::ContractStr,
    ) {
        (self.key, self.label)
    }
}
