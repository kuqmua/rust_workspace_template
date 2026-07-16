#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SynchronizationRuntimeConfiguration {
    execution_mode: server_runtime::ExecutionMode,
    retry_policy: server_runtime::RetryPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SynchronizationPayload(Vec<u8>);

impl From<Vec<u8>> for SynchronizationPayload {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for SynchronizationPayload {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

pub trait SynchronizationSource {
    type Error: std::error::Error + Send + Sync + 'static;

    fn read(&mut self) -> impl Future<Output = Result<SynchronizationPayload, Self::Error>> + Send;
}

impl SynchronizationRuntimeConfiguration {
    #[must_use]
    pub const fn execution_mode(&self) -> server_runtime::ExecutionMode {
        self.execution_mode
    }

    #[must_use]
    pub const fn new(
        retry_policy: server_runtime::RetryPolicy,
        execution_mode: server_runtime::ExecutionMode,
    ) -> Self {
        Self {
            execution_mode,
            retry_policy,
        }
    }

    #[must_use]
    pub const fn retry_policy(&self) -> server_runtime::RetryPolicy {
        self.retry_policy
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn configuration_keeps_retry_and_execution_policies_together() {
        let attempts = server_runtime::StdRetryAttempts::try_from(2usize);
        let delay = server_runtime::StdRetryDelay::from(std::time::Duration::ZERO);
        let valid_attempts = attempts.expect("36b4ca8f");
        let retry_policy = server_runtime::RetryPolicy::new(valid_attempts, Some(delay));
        let configuration = super::SynchronizationRuntimeConfiguration::new(
            retry_policy,
            server_runtime::ExecutionMode::DryRun,
        );
        assert_eq!(configuration.retry_policy(), retry_policy);
        assert_eq!(
            configuration.execution_mode(),
            server_runtime::ExecutionMode::DryRun
        );
    }
}
