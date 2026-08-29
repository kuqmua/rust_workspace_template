#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NotificationErrorCode {
    MetricsRender,
    Persistence,
    Validation,
}
impl NotificationErrorCode {
    pub(crate) const fn get(self) -> &'static str {
        match self {
            Self::MetricsRender => {
                constants_str::catalog::NOTIFICATION_OBSERVED_ERROR_METRICS_RENDER
            }
            Self::Persistence => constants_str::catalog::NOTIFICATION_OBSERVED_ERROR_PERSISTENCE,
            Self::Validation => constants_str::catalog::NOTIFICATION_OBSERVED_ERROR_VALIDATION,
        }
    }
}
