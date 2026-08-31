#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, generate_accessor::Getters)]
pub(crate) struct NotificationState {
    metrics: crate::notification_metrics_exporter_prometheus_renderer::NotificationMetricsExporterPrometheusRenderer,
    pool: app_state::sqlx_pg_pool::SqlxPgPool,
    project_git_info: git_info::project_git_info::ProjectGitInfo<'static>,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl NotificationState {
    #[allow(clippy::single_call_fn)] // service startup owns construction while test_tests reuse it under cfg(test)
    pub(crate) const fn new(
        metrics: crate::notification_metrics_exporter_prometheus_renderer::NotificationMetricsExporterPrometheusRenderer,
        pool: app_state::sqlx_pg_pool::SqlxPgPool,
        project_git_info: git_info::project_git_info::ProjectGitInfo<'static>,
    ) -> Self {
        Self {
            metrics,
            pool,
            project_git_info,
        }
    }
}
impl app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider for NotificationState {
    fn sqlx_pg_pool(&self) -> app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_> {
        app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(self.pool.as_ref())
    }
}
impl AsRef<str> for NotificationState {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
impl common_routes::common_routes_parameters::CommonRoutesParameters for NotificationState {}
