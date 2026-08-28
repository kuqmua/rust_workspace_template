const ADMIN_CLEANUP_INTERVAL_SECONDS: u64 = 300u64;

#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) fn interval() -> Result<
    server_runtime_http::domain_types::RunIntervalDuration,
    crate::domain_types::RunServerError,
> {
    server_runtime_http::domain_types::RunIntervalDuration::try_from(
        std::time::Duration::from_secs(ADMIN_CLEANUP_INTERVAL_SECONDS),
    )
    .map_err(|error| {
        crate::domain_types::RunServerError::RuntimeInterval(
            crate::domain_types::ServerRuntimeRunIntervalError::from(error),
        )
    })
}
