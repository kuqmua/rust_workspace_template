use super::{AdminDataColumn, AdminDataColumns, AdminDataRow};

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
#[optimal_memory_layout(skip)]
pub struct AdminDataTableView {
    #[contract_struct_api(slice = AdminDataColumn)]
    columns: AdminDataColumns,
    #[contract_struct_api(slice = AdminDataRow)]
    items: crate::domain_types::AdminDataRows,
    #[contract_struct_api(copy_ref)]
    table: crate::domain_types::AdminDataTable,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::domain_types::AdminPageTotal,
}
