#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_administrator_asset_route_preserves_static_file_serving() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(
                server_admin_frontend::admin_frontend_routes::admin_frontend_routes(),
            ),
            axum::http::Request::get(constants_str::VALUE_688DB289)
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_D694B6F6),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_499F35E2);
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_operational_routes_are_root_mounted_and_api_routes_are_v1_mounted() {
        let operational_path = common_routes::common_route::CommonRoute::HealthLive.path();
        let router = axum::Router::from(crate::mount_service_routes::mount_service_routes(
            server_runtime_http::axum_router::AxumRouter::from(
                axum::Router::new()
                    .route(
                        operational_path.as_ref(),
                        axum::routing::get(async || axum::http::StatusCode::NO_CONTENT),
                    )
                    .fallback(async || axum::http::StatusCode::IM_A_TEAPOT),
            ),
            crate::axum_api_routes::AxumApiRoutes::from(axum::Router::new().route(
                constants_str::VALUE_87D0B7F8,
                axum::routing::get(async || constants_str::VALUE_14C2529E),
            )),
            crate::http_body_maximum_bytes::HttpBodyMaximumBytes::from(1_024usize),
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
                    .expect(constants_str::DIAGNOSTIC_7496F84F),
            )
        };
        assert_eq!(
            status(operational_path.as_ref())
                .await
                .expect(constants_str::DIAGNOSTIC_0A94FCC5)
                .status(),
            axum::http::StatusCode::NO_CONTENT
        );
        assert_eq!(
            status("/v1/probe")
                .await
                .expect(constants_str::DIAGNOSTIC_6BB8E3F5)
                .status(),
            axum::http::StatusCode::OK
        );
        assert_eq!(
            status("/api/v1/probe")
                .await
                .expect(constants_str::DIAGNOSTIC_11FD3E4A)
                .status(),
            axum::http::StatusCode::SEE_OTHER
        );
        assert_eq!(
            status("/v1/health/live")
                .await
                .expect(constants_str::DIAGNOSTIC_6E17DB87)
                .status(),
            axum::http::StatusCode::SEE_OTHER
        );
    }

    #[tokio::test]
    async fn test_missing_page_redirects_to_default_authentication_page() {
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(crate::frontend_fallback_routes::frontend_fallback_routes()),
            axum::http::Request::builder()
                .uri(constants_str::VALUE_10D40EF4)
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_CFE228D8),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_BD9F2B00);
        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(axum::http::header::LOCATION),
            Some(&axum::http::HeaderValue::from_static(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn.get()
            ))
        );
    }
    #[test]
    fn test_tracing_default_filter_is_stable() {
        assert_eq!(constants_str::CONFIG_TRACING_INFO, "info");
    }
}
