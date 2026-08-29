#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) fn mount_service_routes(
    operational_routes: server_runtime_http::axum_router::AxumRouter,
    api_routes: crate::axum_api_routes::AxumApiRoutes,
    body_maximum_bytes: crate::http_body_maximum_bytes::HttpBodyMaximumBytes,
) -> server_runtime_http::axum_router::AxumRouter {
    server_runtime_http::axum_router::AxumRouter::from(
        axum::Router::new()
            .merge(axum::Router::from(operational_routes).reset_fallback())
            .nest(
                constants_str::catalog::V1,
                axum::Router::from(api_routes).layer(axum::extract::DefaultBodyLimit::max(
                    body_maximum_bytes.get(),
                )),
            ),
    )
}
