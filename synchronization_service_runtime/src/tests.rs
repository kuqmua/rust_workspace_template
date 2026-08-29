#[test]
fn configuration_keeps_retry_and_execution_policies_together() {
    let attempts = server_runtime_core::RetryAttemptsNonZeroUsize::try_from(2usize);
    let delay = server_runtime_core::RetryDelayDuration::from(std::time::Duration::ZERO);
    let valid_attempts = attempts.expect(
        "36b4ca8f configuration_keeps_retry_and_execution_policies_together invariant must hold",
    );
    let retry_policy = server_runtime_core::RetryPolicy::new(valid_attempts, Some(delay));
    let configuration = crate::SynchronizationRuntimeConfiguration::new(
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
    let Ok(payload) = crate::SynchronizationPayload::try_from(vec![0; 16 * 1024 * 1024]) else {
        panic!("5c80aadf");
    };
    assert_eq!(payload.as_ref().len(), 16 * 1024 * 1024);
    let Err(_error) = crate::SynchronizationPayload::try_from(vec![0; 16 * 1024 * 1024 + 1]) else {
        panic!("5e2a6145");
    };
}
