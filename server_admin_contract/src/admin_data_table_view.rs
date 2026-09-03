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
#[optimal_memory_layout(skip)]
pub struct AdminDataTableView {
    #[contract_struct_api(slice = crate::admin_data_column::AdminDataColumn)]
    columns: crate::admin_data_columns::AdminDataColumns,
    #[contract_struct_api(slice = crate::admin_data_row::AdminDataRow)]
    items: crate::admin_data_rows::AdminDataRows,
    #[contract_struct_api(copy_ref)]
    table: crate::admin_data_table::AdminDataTable,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::admin_page_total::AdminPageTotal,
}
