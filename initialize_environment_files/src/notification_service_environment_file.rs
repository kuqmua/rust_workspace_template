#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct NotificationServiceEnvironmentFile {
    notification_database_url: crate::configuration_field::ConfigurationField,
    notification_service_socket_address: crate::configuration_field::ConfigurationField,
    pg_pool_max_connections: crate::configuration_field::ConfigurationField,
    request_timeout_seconds: crate::configuration_field::ConfigurationField,
    svc_mode: crate::configuration_field::ConfigurationField,
    tracing_format: crate::configuration_field::ConfigurationField,
}

impl NotificationServiceEnvironmentFile {
    pub(crate) const fn content(&self) -> &'static [u8] {
        match self.get_notification_database_url() {
            crate::configuration_field::ConfigurationField::Declared => {
                crate::notification_service_environment_content::NOTIFICATION_SERVICE_ENVIRONMENT_CONTENT
            }
        }
    }
}
