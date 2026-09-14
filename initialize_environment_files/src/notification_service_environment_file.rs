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
    pub(crate) fn content(&self) -> crate::std_byte_vector::StdByteVector {
        let mut std_byte_vector = crate::std_byte_vector::StdByteVector::default();
        self.get_notification_database_url()
            .append_to(&mut std_byte_vector);
        self.get_notification_service_socket_address()
            .append_to(&mut std_byte_vector);
        self.get_pg_pool_max_connections()
            .append_to(&mut std_byte_vector);
        self.get_request_timeout_seconds()
            .append_to(&mut std_byte_vector);
        self.get_svc_mode().append_to(&mut std_byte_vector);
        self.get_tracing_format().append_to(&mut std_byte_vector);
        std_byte_vector
    }
}
