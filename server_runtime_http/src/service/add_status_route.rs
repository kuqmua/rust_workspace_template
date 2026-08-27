#[must_use]
pub fn add_status_route(router: super::super::AxumRouter) -> super::super::AxumRouter {
    super::super::AxumRouter::from(axum::Router::from(router).route(
        constants_str::STATUS,
        axum::routing::get(async || http::StatusCode::OK),
    ))
}
