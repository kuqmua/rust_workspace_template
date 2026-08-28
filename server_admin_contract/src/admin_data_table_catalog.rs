#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataTableCatalog {
    #[contract_struct_api(slice = crate::domain_types::AdminDataTable)]
    items: crate::domain_types::AdminDataTables,
}
