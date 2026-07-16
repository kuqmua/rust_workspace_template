#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SynchronizationRuntimeConfiguration {
    execution_mode: server_runtime::ExecutionMode,
    retry_policy: server_runtime::RetryPolicy,
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
