#[cfg(test)]
mod tests {
    #[tokio::test(start_paused = true)]
    async fn test_probe_distinguishes_success_failure_and_timeout() {
        let timeout = crate::health_probe_timeout_duration::HealthProbeTimeoutDuration::from(
            std::time::Duration::from_secs(1u64),
        );
        assert!(bool::from(
            crate::run_health_probe::run_health_probe(timeout, async { true }).await
        ));
        assert!(!bool::from(
            crate::run_health_probe::run_health_probe(timeout, async { false }).await
        ));
        assert!(!bool::from(
            crate::run_health_probe::run_health_probe(timeout, std::future::pending::<bool>())
                .await
        ));
    }

    #[test]
    fn test_readiness_tracks_database_probe_without_affecting_liveness() {
        let readiness = crate::health_readiness::HealthReadiness::default();
        assert_eq!(
            readiness.snapshot().database(),
            crate::health_component_status::HealthComponentStatus::Error
        );
        assert_eq!(
            readiness.snapshot().service(),
            crate::health_component_status::HealthComponentStatus::Ok
        );
        readiness.store_database_probe(crate::health_probe_succeeded::HealthProbeSucceeded::from(
            true,
        ));
        assert_eq!(
            readiness.snapshot().database(),
            crate::health_component_status::HealthComponentStatus::Ok
        );
    }

    #[tokio::test]
    async fn test_health_routes_distinguish_live_and_ready_statuses() {
        let readiness = crate::health_readiness::HealthReadiness::default();
        let router = axum::Router::from(crate::add_health_routes::add_health_routes(
            crate::axum_router::AxumRouter::from(axum::Router::new()),
            &readiness,
        ));
        let live_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::get(constants_str::LIVE_PATH)
                .body(axum::body::Body::empty())
                .expect("a943ebaa health_routes_distinguish_live_and_ready_statuses invariant must hold"),
        )
        .await
        .expect("8112486b health_routes_distinguish_live_and_ready_statuses invariant must hold");
        assert_eq!(live_response.status(), http::StatusCode::OK);
        let unavailable_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::get(constants_str::READY_PATH)
                .body(axum::body::Body::empty())
                .expect("341e303a health_routes_distinguish_live_and_ready_statuses invariant must hold"),
        )
        .await
        .expect("ee4cfce6 health_routes_distinguish_live_and_ready_statuses invariant must hold");
        assert_eq!(
            unavailable_response.status(),
            http::StatusCode::SERVICE_UNAVAILABLE
        );
        readiness.store_database_probe(crate::health_probe_succeeded::HealthProbeSucceeded::from(
            true,
        ));
        let ready_response = tower::ServiceExt::oneshot(
            router,
            http::Request::get(constants_str::READY_PATH)
                .body(axum::body::Body::empty())
                .expect("67247299 health_routes_distinguish_live_and_ready_statuses invariant must hold"),
        )
        .await
        .expect("7cf14a1f health_routes_distinguish_live_and_ready_statuses invariant must hold");
        assert_eq!(ready_response.status(), http::StatusCode::OK);
    }
}

// Root-owned module compatibility wrappers.
mod add_health_routes {}
mod health_component_status {}
mod health_probe_succeeded {}
mod health_probe_timeout_duration {}
mod health_readiness {}
mod health_ready_error {}
mod health_snapshot {}
mod run_health_probe {}
mod service_liveness_snapshot {}
mod shared_health_readiness_arc {}
