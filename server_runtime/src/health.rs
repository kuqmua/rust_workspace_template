#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdHealthProbeTimeout(std::time::Duration);
impl From<std::time::Duration> for StdHealthProbeTimeout {
    fn from(value: std::time::Duration) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HealthProbeSucceeded(bool);
impl From<HealthProbeSucceeded> for bool {
    fn from(value: HealthProbeSucceeded) -> Self {
        value.0
    }
}
pub async fn run_health_probe<Probe>(
    timeout: StdHealthProbeTimeout,
    probe: Probe,
) -> HealthProbeSucceeded
where
    Probe: Future<Output = bool>,
{
    HealthProbeSucceeded(matches!(
        tokio::time::timeout(timeout.0, probe).await,
        Ok(true)
    ))
}
#[cfg(test)]
mod tests {
    #[tokio::test(start_paused = true)]
    async fn probe_distinguishes_success_failure_and_timeout() {
        let timeout = super::StdHealthProbeTimeout::from(std::time::Duration::from_secs(1u64));
        assert!(bool::from(
            super::run_health_probe(timeout, async { true }).await
        ));
        assert!(!bool::from(
            super::run_health_probe(timeout, async { false }).await
        ));
        assert!(!bool::from(
            super::run_health_probe(timeout, async {
                tokio::time::sleep(std::time::Duration::from_secs(2u64)).await;
                true
            })
            .await
        ));
    }
}
