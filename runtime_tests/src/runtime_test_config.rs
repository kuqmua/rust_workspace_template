#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeTestConfig {
    application_base_url: crate::service_base_url::ServiceBaseUrl,
    notification_service_base_url: crate::service_base_url::ServiceBaseUrl,
}

impl RuntimeTestConfig {
    #[must_use]
    pub const fn new(
        application_base_url: crate::service_base_url::ServiceBaseUrl,
        notification_service_base_url: crate::service_base_url::ServiceBaseUrl,
    ) -> Self {
        Self {
            application_base_url,
            notification_service_base_url,
        }
    }
}
