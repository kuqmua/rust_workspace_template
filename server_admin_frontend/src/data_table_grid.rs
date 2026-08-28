pub(super) fn data_table_grid(
    view: &server_admin_contract::domain_types::AdminDataTableView,
    query: &server_admin_contract::domain_types::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    let operation = query
        .filter()
        .operation()
        .map(server_admin_contract::domain_types::AdminFilterOperationKey::from);
    crate::domain_types::shared::admin_data_table_grid::admin_data_table_grid(
        view,
        query.filter().field(),
        operation.as_ref(),
        query.filter().value(),
        query.filter().end(),
        query.page().limit(),
    )
}
