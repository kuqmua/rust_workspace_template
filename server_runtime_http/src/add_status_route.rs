#[must_use]
pub fn add_status_route(router: crate::AxumRouter) -> crate::AxumRouter {
    crate::AxumRouter::from(axum::Router::from(router).route(
        constants_str::STATUS,
        axum::routing::get(async || http::StatusCode::OK),
    ))
}
