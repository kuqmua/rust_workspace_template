pub(super) fn table_pagination(
    admin_page: server_admin_contract::admin_page::AdminPage,
    admin_table_query: &server_admin_contract::admin_table_query::AdminTableQuery,
    admin_page_total: server_admin_contract::admin_page_total::AdminPageTotal,
    table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
    table_filter: Option<
        &server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery,
    >,
) -> impl leptos::prelude::IntoView {
    let action = table.map_or_else(
        || {
            server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath::from(
                admin_page.spec().frontend_path(),
            )
        },
        server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath::from,
    );
    crate::admin_pagination::admin_pagination(
        &action,
        &crate::admin_pagination_query::AdminPaginationQuery::Ssr {
            admin_table_query,
            admin_data_table_filter_query: table_filter,
        },
        admin_page_total,
    )
}
