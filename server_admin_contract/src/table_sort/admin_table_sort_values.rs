#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AdminTableSortValues {
    pub(super) key: frontend_contract::domain_types::ContractStr,
    pub(super) label: frontend_contract::domain_types::ContractStr,
}
