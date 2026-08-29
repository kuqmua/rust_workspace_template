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
pub struct AdminDataRow {
    #[contract_struct_api(slice = crate::admin_text::AdminText)]
    values: crate::admin_texts::AdminTexts,
}
