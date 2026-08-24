const ADMIN_CLEANUP_INTERVAL_SECONDS: u64 = 300u64;

#[allow(
    clippy::single_call_fn,
    reason = "keeps administrator retention policy out of service startup orchestration"
)]
pub(super) fn cfg() -> Result<server_admin::AdminCleanupCfg, super::RunServerError> {
    let batch_size = server_admin::AdminCleanupBatchSize::try_from(1_000i64).map_err(|error| {
        super::RunServerError::AdminCleanupConfig(super::ServerAdminCleanupCfgError::from(error))
    })?;
    let retention = |seconds| {
        server_admin::AdminCleanupRetentionSeconds::try_from(seconds).map_err(|error| {
            super::RunServerError::AdminCleanupConfig(super::ServerAdminCleanupCfgError::from(
                error,
            ))
        })
    };
    Ok(server_admin::AdminCleanupCfg::new(
        batch_size,
        retention(604_800i64)?,
        retention(7_776_000i64)?,
        retention(86_400i64)?,
        retention(86_400i64)?,
        retention(3_600i64)?,
    ))
}

#[allow(
    clippy::single_call_fn,
    reason = "keeps administrator maintenance scheduling policy in its owning module"
)]
pub(super) fn interval() -> Result<server_runtime_http::RunIntervalDuration, super::RunServerError>
{
    server_runtime_http::RunIntervalDuration::try_from(std::time::Duration::from_secs(
        ADMIN_CLEANUP_INTERVAL_SECONDS,
    ))
    .map_err(|error| {
        super::RunServerError::RuntimeInterval(super::ServerRuntimeRunIntervalError::from(error))
    })
}
