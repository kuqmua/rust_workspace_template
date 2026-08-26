const ADMIN_CLEANUP_INTERVAL_SECONDS: u64 = 300u64;

#[allow(
    clippy::single_call_fn,
    reason = "keeps administrator retention policy out of service startup orchestration"
)]
pub(crate) fn cfg()
-> Result<server_admin::domain_types::AdminCleanupCfg, crate::domain_types::RunServerError> {
    let batch_size = server_admin::domain_types::AdminCleanupBatchSize::try_from(1_000i64)
        .map_err(|error| {
            crate::domain_types::RunServerError::AdminCleanupConfig(
                crate::domain_types::ServerAdminCleanupCfgError::from(error),
            )
        })?;
    let retention = |seconds| {
        server_admin::domain_types::AdminCleanupRetentionSeconds::try_from(seconds).map_err(
            |error| {
                crate::domain_types::RunServerError::AdminCleanupConfig(
                    crate::domain_types::ServerAdminCleanupCfgError::from(error),
                )
            },
        )
    };
    Ok(server_admin::domain_types::AdminCleanupCfg::new(
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
