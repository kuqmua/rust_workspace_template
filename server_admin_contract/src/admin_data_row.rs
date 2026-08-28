#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataRow {
    #[contract_struct_api(slice = crate::domain_types::AdminText)]
    values: crate::domain_types::AdminTexts,
}
