#[path = "synchronization_payload.rs"]
mod synchronization_payload;
#[path = "synchronization_payload_max_bytes.rs"]
mod synchronization_payload_max_bytes;
#[path = "synchronization_payload_too_large.rs"]
mod synchronization_payload_too_large;
#[path = "synchronization_runtime_configuration.rs"]
mod synchronization_runtime_configuration;
#[path = "synchronization_source.rs"]
mod synchronization_source;

pub use synchronization_payload::SynchronizationPayload;
pub use synchronization_payload_too_large::SynchronizationPayloadTooLarge;
pub use synchronization_runtime_configuration::SynchronizationRuntimeConfiguration;
pub use synchronization_source::SynchronizationSource;

#[cfg(test)]
mod tests {
    #[test]
    fn configuration_keeps_retry_and_execution_policies_together() {
        let attempts =
            server_runtime_core::domain_types::RetryAttemptsNonZeroUsize::try_from(2usize);
        let delay =
            server_runtime_core::domain_types::RetryDelayDuration::from(std::time::Duration::ZERO);
        let valid_attempts = attempts.expect("36b4ca8f configuration_keeps_retry_and_execution_policies_together invariant must hold");
        let retry_policy =
            server_runtime_core::domain_types::RetryPolicy::new(valid_attempts, Some(delay));
        let configuration = super::SynchronizationRuntimeConfiguration::new(
            retry_policy,
            server_runtime_core::domain_types::ExecutionMode::DryRun,
        );
        assert_eq!(configuration.retry_policy(), retry_policy);
        assert_eq!(
            configuration.execution_mode(),
            server_runtime_core::domain_types::ExecutionMode::DryRun
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
