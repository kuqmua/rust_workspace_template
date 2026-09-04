#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(bare)]
pub(super) struct AdminTableSortValues {
    #[getters(copy)]
    key: frontend_contract::contract_str::ContractStr,
    #[getters(copy)]
    label: frontend_contract::contract_str::ContractStr,
}
