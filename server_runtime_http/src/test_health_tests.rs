#[cfg(test)]
mod tests {
    #[tokio::test(start_paused = true)]
    async fn test_zero_deadline_health_probe_polls_ready_results_and_drops_pending_future() {
        let timeout = crate::health_probe_timeout_duration::HealthProbeTimeoutDuration::from(
            std::time::Duration::ZERO,
        );
        assert!(bool::from(
            crate::run_health_probe::run_health_probe(timeout, std::future::ready(true)).await
        ));
        assert!(!bool::from(
            crate::run_health_probe::run_health_probe(timeout, std::future::ready(false)).await
        ));
        let (sender, mut receiver) = tokio::sync::oneshot::channel::<()>();
        let pending_probe = async move {
            let result = std::future::pending::<bool>().await;
            drop(sender);
            result
        };
        assert!(!bool::from(
            crate::run_health_probe::run_health_probe(timeout, pending_probe).await
        ));
        assert!(matches!(
            receiver.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Closed)
        ));
    }

    #[test]
    fn test_readiness_clones_share_probe_updates_and_independent_instances_do_not() {
        let readiness = crate::health_readiness::HealthReadiness::default();
        let shared = readiness.clone();
        let independent = crate::health_readiness::HealthReadiness::default();
        assert!([true, false, true].into_iter().all(|succeeded| {
            shared.store_database_probe(crate::health_probe_succeeded::HealthProbeSucceeded::from(
                succeeded,
            ));
            let expected_database = if succeeded {
                crate::health_component_status::HealthComponentStatus::Ok
            } else {
                crate::health_component_status::HealthComponentStatus::Error
            };
            readiness.snapshot() == shared.snapshot()
                && readiness.snapshot().database() == expected_database
                && readiness.snapshot().service()
                    == crate::health_component_status::HealthComponentStatus::Ok
                && independent.snapshot().database()
                    == crate::health_component_status::HealthComponentStatus::Error
                && independent.snapshot().service()
                    == crate::health_component_status::HealthComponentStatus::Ok
        }));
    }

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

    #[test]
    fn test_health_snapshots_serialize_exact_fields_and_snake_case_statuses() {
        assert!(
            [
                (
                    crate::health_component_status::HealthComponentStatus::Error,
                    stringify!(error)
                ),
                (
                    crate::health_component_status::HealthComponentStatus::Ok,
                    stringify!(ok)
                ),
            ]
            .into_iter()
            .all(|(status, text)| {
                let expected_readiness = serde_json::json!({
                    (stringify!(database)): text,
                    (stringify!(service)): stringify!(ok),
                });
                let expected_liveness = serde_json::json!({ (stringify!(service)): text });
                serde_json::to_value(crate::health_snapshot::HealthSnapshot::new(
                    status,
                    crate::health_component_status::HealthComponentStatus::Ok,
                ))
                .is_ok_and(|value| value == expected_readiness)
                    && serde_json::to_value(
                        crate::service_liveness_snapshot::ServiceLivenessSnapshot::new(status),
                    )
                    .is_ok_and(|value| value == expected_liveness)
            })
        );
    }
}
