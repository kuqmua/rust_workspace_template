pub async fn run_health_probe<Probe>(
    timeout: super::HealthProbeTimeoutDuration,
    probe: Probe,
) -> super::HealthProbeSucceeded
where
    Probe: Future<Output = bool>,
{
    super::HealthProbeSucceeded::from(matches!(
        tokio::time::timeout(timeout.0, probe).await,
        Ok(true)
    ))
}
