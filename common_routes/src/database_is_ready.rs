pub(super) async fn database_is_ready(
    app_state: &crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> crate::health_check_succeeded::HealthCheckSucceeded {
    let pool = app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider::sqlx_pg_pool(app_state.get());
    let probe = async {
        sqlx::query(constants_str::catalog::COMMON_ROUTES_HEALTH_CHECK_SQL)
            .execute(pool.as_ref())
            .await
            .is_ok()
    };
    crate::health_check_succeeded::HealthCheckSucceeded::from(bool::from(
        server_runtime_http::run_health_probe::run_health_probe(
            server_runtime_http::health_probe_timeout_duration::HealthProbeTimeoutDuration::from(
                crate::health_probe_timeout::HEALTH_PROBE_TIMEOUT,
            ),
            probe,
        )
        .await,
    ))
}
