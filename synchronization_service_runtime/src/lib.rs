const SYNCHRONIZATION_PAYLOAD_MAX_BYTES: usize = 16 * 1024 * 1024;

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optml takes precedence over alphabetical field order
pub struct SynchronizationRuntimeConfiguration {
    retry_policy: server_runtime_core::RetryPolicy,
    execution_mode: server_runtime_core::ExecutionMode,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}", std::any::type_name::<Self>())]
pub struct SynchronizationPayloadTooLarge;
impl From<bounded_types::BoundedValueError> for SynchronizationPayloadTooLarge {
    fn from(_value: bounded_types::BoundedValueError) -> Self {
        Self
    }
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget)]
pub struct SynchronizationPayload(
    bounded_types::BoundedVec<u8, 0, SYNCHRONIZATION_PAYLOAD_MAX_BYTES>,
);

impl TryFrom<Vec<u8>> for SynchronizationPayload {
    type Error = SynchronizationPayloadTooLarge;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(SynchronizationPayloadTooLarge::from)
    }
}

pub trait SynchronizationSource {
    type Error: std::error::Error + Send + Sync + 'static;

    fn read(&mut self) -> impl Future<Output = Result<SynchronizationPayload, Self::Error>> + Send;
}

impl SynchronizationRuntimeConfiguration {
    #[must_use]
    pub const fn execution_mode(&self) -> server_runtime_core::ExecutionMode {
        self.execution_mode
    }

    #[must_use]
    pub const fn new(
        retry_policy: server_runtime_core::RetryPolicy,
        execution_mode: server_runtime_core::ExecutionMode,
    ) -> Self {
        Self {
            retry_policy,
            execution_mode,
        }
    }

    #[must_use]
    pub const fn retry_policy(&self) -> server_runtime_core::RetryPolicy {
        self.retry_policy
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn configuration_keeps_retry_and_execution_policies_together() {
        let attempts = server_runtime_core::StdRetryAttempts::try_from(2usize);
        let delay = server_runtime_core::StdRetryDelay::from(std::time::Duration::ZERO);
        let valid_attempts = attempts.expect("36b4ca8f");
        let retry_policy = server_runtime_core::RetryPolicy::new(valid_attempts, Some(delay));
        let configuration = super::SynchronizationRuntimeConfiguration::new(
            retry_policy,
            server_runtime_core::ExecutionMode::DryRun,
        );
        assert_eq!(configuration.retry_policy(), retry_policy);
        assert_eq!(
            configuration.execution_mode(),
            server_runtime_core::ExecutionMode::DryRun
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
