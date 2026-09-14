#[allow(
    clippy::single_call_fn,
    reason = "the named filesystem owner is required for direct unit-test coverage"
)]
pub(crate) fn generate_environment_files(workspace_root: &std::path::Path) -> std::io::Result<()> {
    let field = || crate::configuration_field::ConfigurationField::Declared;
    let server_environment_file = crate::server_environment_file::ServerEnvironmentFile::new(
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
        field(),
    );
    let notification_service_environment_file =
        crate::notification_service_environment_file::NotificationServiceEnvironmentFile::new(
            field(),
            field(),
            field(),
            field(),
            field(),
            field(),
        );
    let docker_compose_file = crate::docker_compose_file::DockerComposeFile::new(
        crate::docker_compose_database_service::DockerComposeDatabaseService::new(
            field(),
            field(),
            field(),
            field(),
            field(),
            field(),
            field(),
        ),
        field(),
        crate::docker_compose_notification_database_service::DockerComposeNotificationDatabaseService::new(
            field(), field(), field(), field(), field(), field(), field(),
        ),
        crate::docker_compose_notification_service::DockerComposeNotificationService::new(
            field(), field(), field(), field(), field(), field(), field(), field(), field(), field(),
            field(), field(),
        ),
        crate::docker_compose_notification_service_migrate_service::DockerComposeNotificationServiceMigrateService::new(
            field(), field(), field(), field(), field(), field(), field(), field(), field(),
        ),
        crate::docker_compose_server_service::DockerComposeServerService::new(
            field(), field(), field(), field(), field(), field(), field(), field(), field(), field(),
            field(), field(),
        ),
        crate::docker_compose_server_migrate_service::DockerComposeServerMigrateService::new(
            field(), field(), field(), field(), field(), field(), field(), field(), field(),
        ),
        field(),
    );
    [
        (
            constants_str::VALUE_0A7A2313,
            notification_service_environment_file.content(),
        ),
        (constants_str::VALUE_E45E45BA, docker_compose_file.content()),
    ]
    .into_iter()
    .try_for_each(|(relative_path, content)| {
        std::fs::write(workspace_root.join(relative_path), content)
    })?;
    std::fs::write(
        workspace_root
            .join(constants_str::VALUE_B3EACD33)
            .join(constants_str::ENV),
        server_environment_file.content(),
    )?;
    std::fs::write(
        workspace_root
            .join(constants_str::VALUE_B3EACD33)
            .join(constants_str::ENV_EXAMPLE),
        server_environment_file.content(),
    )?;
    std::fs::write(
        workspace_root
            .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
            .join(constants_str::ENV),
        notification_service_environment_file.content(),
    )
}
