use super::MetricsExporterPrometheusRenderer;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(crate) struct NotificationState {
    pub(crate) metrics: MetricsExporterPrometheusRenderer,
    pub(crate) pool: app_state::domain_types::SqlxPgPool,
    pub(crate) project_git_info: git_info::domain_types::ProjectGitInfo<'static>,
}
impl app_state::domain_types::SqlxPgPoolProvider for NotificationState {
    fn sqlx_pg_pool(&self) -> app_state::domain_types::SqlxPgPoolRef<'_> {
        app_state::domain_types::SqlxPgPoolRef::from(self.pool.as_ref())
    }
}
impl AsRef<str> for NotificationState {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
impl common_routes::domain_types::CommonRoutesParameters for NotificationState {}
