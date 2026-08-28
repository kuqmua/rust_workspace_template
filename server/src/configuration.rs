pub(crate) fn configuration()
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
