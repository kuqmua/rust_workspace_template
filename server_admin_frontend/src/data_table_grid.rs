#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(super) fn data_table_grid(
    view: &server_admin_contract::admin_data_table_view::AdminDataTableView,
    query: &server_admin_contract::admin_data_table_query::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    let operation = query
        .filter()
        .operation()
        .map(server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from);
    crate::admin_data_table_grid::admin_data_table_grid(
        view,
        query.filter().field(),
        operation.as_ref(),
        query.filter().value(),
        query.filter().end(),
        query.page().limit(),
    )
}
