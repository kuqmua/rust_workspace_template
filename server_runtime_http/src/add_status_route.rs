#[must_use]
pub fn add_status_route(router: crate::axum_router::AxumRouter) -> crate::axum_router::AxumRouter {
    crate::axum_router::AxumRouter::from(axum::Router::from(router).route(
        constants_str::catalog::STATUS,
        axum::routing::get(async || http::StatusCode::OK),
    ))
}
