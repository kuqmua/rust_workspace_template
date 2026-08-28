use crate::{AdminGeneratedTable, SharedAdminGeneratedTableStateArc};

#[must_use]
pub fn generated_routes(
    app_state: &SharedAdminGeneratedTableStateArc,
) -> server_runtime_http::domain_types::AxumRouter {
    server_runtime_http::domain_types::AxumRouter::from(
        AdminGeneratedTable::ALL
            .into_iter()
            .fold(axum::Router::new(), |routes, table| {
                routes.merge(axum::Router::from(table.routes(app_state)))
            }),
    )
}
