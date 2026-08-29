pub(crate) use super::admin_metrics_error::*;
pub(crate) use super::axum_api_routes::*;
pub(crate) use super::http_body_maximum_bytes::*;
pub(crate) use super::metrics_exporter_prometheus_build_error::*;
pub(crate) use super::metrics_exporter_prometheus_renderer::*;
pub(crate) use super::run_server_error::*;
pub(crate) use super::server_exit_code::*;
pub(crate) use super::server_io_error::*;
pub(crate) use super::shared_server_app_state_arc::*;
pub(crate) use super::sqlx_server_pg_connect_error::*;
pub(crate) use super::tokio_server_runtime::*;
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn administrator_asset_route_preserves_static_file_serving() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(server_admin_frontend::domain_types::admin_frontend_routes()),
            axum::http::Request::get(constants_str::VALUE_688DB289)
                .body(axum::body::Body::empty())
                .expect("d694b6f6 administrator_asset_route_preserves_static_file_serving invariant must hold"),
        )
        .await
        .expect("499f35e2 administrator_asset_route_preserves_static_file_serving invariant must hold");
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn operational_routes_are_root_mounted_and_api_routes_are_v1_mounted() {
        let operational_path = common_routes::CommonRoute::HealthLive.path();
        let router = axum::Router::from(crate::mount_service_routes::mount_service_routes(
            server_runtime_http::domain_types::AxumRouter::from(
                axum::Router::new()
                    .route(
                        operational_path.as_ref(),
                        axum::routing::get(async || axum::http::StatusCode::NO_CONTENT),
                    )
                    .fallback(async || axum::http::StatusCode::IM_A_TEAPOT),
            ),
            super::AxumApiRoutes::from(axum::Router::new().route(
                constants_str::VALUE_87D0B7F8,
                axum::routing::get(async || constants_str::VALUE_14C2529E),
            )),
            super::HttpBodyMaximumBytes::from(1_024usize),
        ))
        .merge(axum::Router::from(
            crate::frontend_fallback_routes::frontend_fallback_routes(),
        ));
        let status = |path: &str| {
            tower::ServiceExt::oneshot(
                router.clone(),
                axum::http::Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .expect("7496f84f operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold"),
            )
        };
        assert_eq!(
            status(operational_path.as_ref())
                .await
                .expect("0a94fcc5 operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold")
                .status(),
            axum::http::StatusCode::NO_CONTENT
        );
        assert_eq!(
            status("/v1/probe").await.expect("6bb8e3f5 operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold").status(),
            axum::http::StatusCode::OK
        );
        assert_eq!(
            status("/api/v1/probe").await.expect("11fd3e4a operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold").status(),
            axum::http::StatusCode::SEE_OTHER
        );
        assert_eq!(
            status("/v1/health/live").await.expect("6e17db87 operational_routes_are_root_mounted_and_api_routes_are_v1_mounted invariant must hold").status(),
            axum::http::StatusCode::SEE_OTHER
        );
    }

    #[tokio::test]
    async fn missing_page_redirects_to_default_authentication_page() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(crate::frontend_fallback_routes::frontend_fallback_routes()),
            axum::http::Request::builder()
                .uri(constants_str::VALUE_10D40EF4)
                .body(axum::body::Body::empty())
                .expect("cfe228d8 missing_page_redirects_to_default_authentication_page invariant must hold"),
        )
        .await
        .expect("bd9f2b00 missing_page_redirects_to_default_authentication_page invariant must hold");
        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(axum::http::header::LOCATION),
            Some(&axum::http::HeaderValue::from_static(
                server_admin_contract::domain_types::AdminFrontendPath::SignIn.get()
            ))
        );
    }
    #[test]
    fn tracing_default_filter_is_stable() {
        assert_eq!(constants_str::CONFIG_TRACING_INFO, "info");
    }
}
