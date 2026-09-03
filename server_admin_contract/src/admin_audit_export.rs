#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_frontend_contract_derive_contract_struct_api::ContractStructApi,
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
