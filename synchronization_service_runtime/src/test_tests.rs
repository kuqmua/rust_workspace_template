#[test]
fn test_configuration_keeps_retry_and_execution_policies_together() {
    let attempts =
        server_runtime_core::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::try_from(
            2usize,
        );
    let delay = server_runtime_core::retry_delay_duration::RetryDelayDuration::from(
        std::time::Duration::ZERO,
    );
    let valid_attempts = attempts.expect(
        "36b4ca8f configuration_keeps_retry_and_execution_policies_together invariant must hold",
    );
    let retry_policy =
        server_runtime_core::retry_policy::RetryPolicy::new(valid_attempts, Some(delay));
    let configuration =
        crate::synchronization_runtime_configuration::SynchronizationRuntimeConfiguration::new(
            retry_policy,
            server_runtime_core::execution_mode::ExecutionMode::DryRun,
        );
    assert_eq!(configuration.retry_policy(), retry_policy);
    assert_eq!(
        configuration.execution_mode(),
        server_runtime_core::execution_mode::ExecutionMode::DryRun
    );
}

#[test]
fn test_synchronization_payload_enforces_maximum_byte_length() {
    let Ok(payload) =
        crate::synchronization_payload::SynchronizationPayload::try_from(vec![0; 16 * 1024 * 1024])
    else {
        panic!("5c80aadf");
    };
    assert_eq!(payload.as_ref().len(), 16 * 1024 * 1024);
    let Err(_error) = crate::synchronization_payload::SynchronizationPayload::try_from(vec![
        0;
        16 * 1024 * 1024
            + 1
    ]) else {
        panic!("5e2a6145");
    };
}
