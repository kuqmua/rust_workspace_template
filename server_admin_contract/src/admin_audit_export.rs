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
pub struct AdminAuditExport {
    #[schema(value_type = String, max_length = 262_144)]
    #[contract_struct_api(borrow)]
    csv: crate::admin_audit_export_csv::AdminAuditExportCsv,
}
