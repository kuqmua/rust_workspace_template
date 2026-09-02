#[must_use]
pub fn add_status_route(
    axum_router: crate::axum_router::AxumRouter,
) -> crate::axum_router::AxumRouter {
    crate::axum_router::AxumRouter::from(axum::Router::from(axum_router).route(
        constants_str::STATUS,
        axum::routing::get(async || http::StatusCode::OK),
    ))
}
