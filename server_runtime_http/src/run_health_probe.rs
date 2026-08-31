pub async fn run_health_probe<Probe>(
    timeout: crate::health_probe_timeout_duration::HealthProbeTimeoutDuration,
    probe: Probe,
) -> crate::health_probe_succeeded::HealthProbeSucceeded
where
    Probe: Future<Output = bool>,
{
    crate::health_probe_succeeded::HealthProbeSucceeded::from(matches!(
        tokio::time::timeout(timeout.get(), probe).await,
        Ok(true)
    ))
}
