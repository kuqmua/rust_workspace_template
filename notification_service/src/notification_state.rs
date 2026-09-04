#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct NotificationState {
    metrics: crate::notification_metrics_exporter_prometheus_renderer::NotificationMetricsExporterPrometheusRenderer,
    pool: app_state::sqlx_pg_pool::SqlxPgPool,
    project_git_info: git_info::project_git_info::ProjectGitInfo<'static>,
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
