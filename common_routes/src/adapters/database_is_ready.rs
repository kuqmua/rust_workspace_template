pub(super) async fn database_is_ready(
    app_state: &crate::domain_types::ArcCommonRoutesAppState,
) -> crate::domain_types::HealthCheckSucceeded {
    let pool = app_state::domain_types::SqlxPgPoolProvider::sqlx_pg_pool(app_state.get());
    let probe = async {
        sqlx::query(constants_str::COMMON_ROUTES_HEALTH_CHECK_SQL)
            .execute(pool.as_ref())
            .await
            .is_ok()
    };
    crate::domain_types::HealthCheckSucceeded::from(bool::from(
        server_runtime_http::domain_types::run_health_probe(
            server_runtime_http::domain_types::HealthProbeTimeoutDuration::from(
                crate::domain_types::HEALTH_PROBE_TIMEOUT,
            ),
            probe,
        )
        .await,
    ))
}
