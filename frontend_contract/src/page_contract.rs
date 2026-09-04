#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_new::New,
)]
pub struct PageContract {
    actions: crate::action_contracts::ActionContracts,
    fields: crate::field_contracts::FieldContracts,
    #[getters(copy)]
    path: crate::contract_str::ContractStr,
    routes: crate::route_contracts::RouteContracts,
    #[getters(copy)]
    title: crate::contract_str::ContractStr,
}
