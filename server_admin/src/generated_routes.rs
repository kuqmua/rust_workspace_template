#[must_use]
pub fn generated_routes(
    app_state: &crate::shared_admin_generated_table_state_arc::SharedAdminGeneratedTableStateArc,
) -> server_runtime_http::axum_router::AxumRouter {
    server_runtime_http::axum_router::AxumRouter::from(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .into_iter()
            .fold(axum::Router::new(), |routes, table| {
                routes.merge(axum::Router::from(table.routes(app_state)))
            }),
    )
}
