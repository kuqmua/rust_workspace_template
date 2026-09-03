#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn mount_service_routes(
    axum_router: server_runtime_http::axum_router::AxumRouter,
    axum_api_routes: crate::axum_api_routes::AxumApiRoutes,
    http_body_maximum_bytes: crate::http_body_maximum_bytes::HttpBodyMaximumBytes,
) -> server_runtime_http::axum_router::AxumRouter {
    server_runtime_http::axum_router::AxumRouter::from(
        axum::Router::new()
            .merge(axum::Router::from(axum_router).reset_fallback())
            .nest(
                constants_str::V1,
                axum::Router::from(axum_api_routes).layer(axum::extract::DefaultBodyLimit::max(
                    http_body_maximum_bytes.get(),
                )),
            ),
    )
}
