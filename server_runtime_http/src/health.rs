pub use crate::add_health_routes::add_health_routes;
pub use crate::health_component_status::HealthComponentStatus;
pub use crate::health_probe_succeeded::HealthProbeSucceeded;
pub use crate::health_probe_timeout_duration::HealthProbeTimeoutDuration;
pub use crate::health_readiness::HealthReadiness;
use crate::health_ready_error::HealthReadyError;
pub use crate::health_snapshot::HealthSnapshot;
pub use crate::run_health_probe::run_health_probe;
pub use crate::service_liveness_snapshot::ServiceLivenessSnapshot;
use crate::shared_health_readiness_arc::SharedHealthReadinessArc;
#[cfg(test)]
mod tests {
    #[tokio::test(start_paused = true)]
    async fn probe_distinguishes_success_failure_and_timeout() {
        let timeout = super::HealthProbeTimeoutDuration::from(std::time::Duration::from_secs(1u64));
        assert!(bool::from(
            super::run_health_probe(timeout, async { true }).await
        ));
        assert!(!bool::from(
            super::run_health_probe(timeout, async { false }).await
        ));
        assert!(!bool::from(
            super::run_health_probe(timeout, std::future::pending::<bool>()).await
        ));
    }

    #[test]
    fn readiness_tracks_database_probe_without_affecting_liveness() {
        let readiness = super::HealthReadiness::default();
        assert_eq!(
            readiness.snapshot().database(),
            super::HealthComponentStatus::Error
        );
        assert_eq!(
            readiness.snapshot().service(),
            super::HealthComponentStatus::Ok
        );
        readiness.store_database_probe(super::HealthProbeSucceeded::from(true));
        assert_eq!(
            readiness.snapshot().database(),
            super::HealthComponentStatus::Ok
        );
    }

    #[tokio::test]
    async fn health_routes_distinguish_live_and_ready_statuses() {
        let readiness = super::HealthReadiness::default();
        let router = axum::Router::from(super::add_health_routes(
            crate::domain_types::AxumRouter::from(axum::Router::new()),
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
        readiness.store_database_probe(super::HealthProbeSucceeded::from(true));
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
mod add_health_routes {
    pub use crate::add_health_routes::*;
}
mod health_component_status {
    pub use crate::health_component_status::*;
}
mod health_probe_succeeded {
    pub use crate::health_probe_succeeded::*;
}
mod health_probe_timeout_duration {
    pub use crate::health_probe_timeout_duration::*;
}
mod health_readiness {
    pub use crate::health_readiness::*;
}
mod health_ready_error {
    pub use crate::health_ready_error::*;
}
mod health_snapshot {
    pub use crate::health_snapshot::*;
}
mod run_health_probe {
    pub use crate::run_health_probe::*;
}
mod service_liveness_snapshot {
    pub use crate::service_liveness_snapshot::*;
}
mod shared_health_readiness_arc {
    pub use crate::shared_health_readiness_arc::*;
}
