#![allow(clippy::single_call_fn)] // service route composition has one application owner

pub(crate) fn mount_service_routes(
    operational_routes: server_runtime_http::domain_types::AxumRouter,
    api_routes: crate::domain_types::AxumApiRoutes,
    body_maximum_bytes: crate::domain_types::HttpBodyMaximumBytes,
) -> server_runtime_http::domain_types::AxumRouter {
    server_runtime_http::domain_types::AxumRouter::from(
        axum::Router::new()
            .merge(axum::Router::from(operational_routes).reset_fallback())
            .nest(
                constants_str::V1,
                axum::Router::from(api_routes).layer(axum::extract::DefaultBodyLimit::max(
                    body_maximum_bytes.get(),
                )),
            ),
    )
}
