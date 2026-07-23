const SYNCHRONIZATION_PAYLOAD_MAX_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SynchronizationRuntimeConfiguration {
    execution_mode: server_runtime::ExecutionMode,
    retry_policy: server_runtime::RetryPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}", std::any::type_name::<Self>())]
pub struct SynchronizationPayloadTooLarge;

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefTarget)]
pub struct SynchronizationPayload(Vec<u8>);

impl TryFrom<Vec<u8>> for SynchronizationPayload {
    type Error = SynchronizationPayloadTooLarge;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > SYNCHRONIZATION_PAYLOAD_MAX_BYTES {
            Err(SynchronizationPayloadTooLarge)
        } else {
            Ok(Self(value))
        }
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

    #[test]
    fn synchronization_payload_enforces_maximum_byte_length() {
        let Ok(payload) = super::SynchronizationPayload::try_from(vec![0; 16 * 1024 * 1024]) else {
            panic!("5c80aadf");
        };
        assert_eq!(payload.as_ref().len(), 16 * 1024 * 1024);
        let Err(_error) = super::SynchronizationPayload::try_from(vec![0; 16 * 1024 * 1024 + 1])
        else {
            panic!("5e2a6145");
        };
    }
}
