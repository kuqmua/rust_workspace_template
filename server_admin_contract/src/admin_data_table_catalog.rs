#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract_macros::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataTableCatalog {
    #[contract_struct_api(slice = crate::admin_data_table::AdminDataTable)]
    items: crate::admin_data_tables::AdminDataTables,
}
