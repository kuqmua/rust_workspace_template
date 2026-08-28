#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeTestConfig {
    application_base_url: crate::domain_types::ServiceBaseUrl,
    notification_service_base_url: crate::domain_types::ServiceBaseUrl,
}

impl RuntimeTestConfig {
    #[must_use]
    pub const fn application_base_url(&self) -> &crate::domain_types::ServiceBaseUrl {
        &self.application_base_url
    }
    #[must_use]
    pub const fn new(
        application_base_url: crate::domain_types::ServiceBaseUrl,
        notification_service_base_url: crate::domain_types::ServiceBaseUrl,
    ) -> Self {
        Self {
            application_base_url,
            notification_service_base_url,
        }
    }
    #[must_use]
    pub const fn notification_service_base_url(&self) -> &crate::domain_types::ServiceBaseUrl {
        &self.notification_service_base_url
    }
}
