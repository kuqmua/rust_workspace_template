#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(super) fn data_table_grid(
    admin_data_table_view: &server_admin_contract::admin_data_table_view::AdminDataTableView,
    admin_data_table_query: &server_admin_contract::admin_data_table_query::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    let operation = admin_data_table_query
        .filter()
        .operation()
        .map(server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from);
    crate::admin_data_table_grid::admin_data_table_grid(
        admin_data_table_view,
        admin_data_table_query.filter().field(),
        operation.as_ref(),
        admin_data_table_query.filter().value(),
        admin_data_table_query.filter().end(),
        admin_data_table_query.page().limit(),
    )
}
