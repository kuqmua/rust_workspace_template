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
}
