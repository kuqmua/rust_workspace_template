#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct DockerComposeFile {
    database: crate::docker_compose_database_service::DockerComposeDatabaseService,
    networks: crate::configuration_field::ConfigurationField,
    notification_database: crate::docker_compose_notification_database_service::DockerComposeNotificationDatabaseService,
    notification_service: crate::docker_compose_notification_service::DockerComposeNotificationService,
    notification_service_migrate: crate::docker_compose_notification_service_migrate_service::DockerComposeNotificationServiceMigrateService,
    server: crate::docker_compose_server_service::DockerComposeServerService,
    server_migrate: crate::docker_compose_server_migrate_service::DockerComposeServerMigrateService,
    volumes: crate::configuration_field::ConfigurationField,
}

impl DockerComposeFile {
    #[allow(
        clippy::single_call_fn,
        reason = "the typed Docker Compose model owns access to its generated file content"
    )]
    pub(crate) fn content(&self) -> crate::std_byte_vector::StdByteVector {
        let mut std_byte_vector = crate::std_byte_vector::StdByteVector::default();
        crate::configuration_field::ConfigurationField::from(b"services:\n".as_slice())
            .append_to(&mut std_byte_vector);
        std_byte_vector.append_std_byte_vector(&self.get_database().content());
        std_byte_vector.append_std_byte_vector(&self.get_notification_database().content());
        std_byte_vector.append_std_byte_vector(&self.get_server_migrate().content());
        std_byte_vector.append_std_byte_vector(&self.get_server().content());
        std_byte_vector.append_std_byte_vector(&self.get_notification_service_migrate().content());
        std_byte_vector.append_std_byte_vector(&self.get_notification_service().content());
        self.get_networks().append_to(&mut std_byte_vector);
        self.get_volumes().append_to(&mut std_byte_vector);
        std_byte_vector
    }
}
