#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, generate_accessor::Getters)]
pub(crate) struct NotificationState {
    metrics: super::MetricsExporterPrometheusRenderer,
    pool: app_state::SqlxPgPool,
    project_git_info: git_info::ProjectGitInfo<'static>,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl NotificationState {
    #[allow(clippy::single_call_fn)] // service startup owns construction while tests reuse it under cfg(test)
    pub(crate) const fn new(
        metrics: super::MetricsExporterPrometheusRenderer,
        pool: app_state::SqlxPgPool,
        project_git_info: git_info::ProjectGitInfo<'static>,
    ) -> Self {
        Self {
            metrics,
            pool,
            project_git_info,
        }
    }
}
impl app_state::SqlxPgPoolProvider for NotificationState {
    fn sqlx_pg_pool(&self) -> app_state::SqlxPgPoolRef<'_> {
        app_state::SqlxPgPoolRef::from(self.pool.as_ref())
    }
}
impl AsRef<str> for NotificationState {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
impl common_routes::CommonRoutesParameters for NotificationState {}
