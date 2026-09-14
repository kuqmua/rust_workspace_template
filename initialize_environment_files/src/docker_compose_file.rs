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
    pub(crate) const fn content(&self) -> &'static [u8] {
        let _database = self.get_database();
        crate::docker_compose_content::DOCKER_COMPOSE_CONTENT
    }
}
