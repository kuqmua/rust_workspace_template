#[allow(
    clippy::single_call_fn,
    reason = "the named filesystem owner is required for direct unit-test coverage"
)]
pub(crate) fn generate_environment_files(workspace_root: &std::path::Path) -> std::io::Result<()> {
    let field = |bytes| crate::configuration_field::ConfigurationField::from(bytes);
    let docker_compose_file = crate::docker_compose_file::DockerComposeFile::new(
        crate::docker_compose_database_service::DockerComposeDatabaseService::new(
            field(br#"    environment:
      POSTGRES_DB: rust_workspace_template
      POSTGRES_PASSWORD: "Dev-admin-2026-Ready!"
      POSTGRES_USER: postgres
"#.as_slice()),
            field(br#"    healthcheck:
      test: ["CMD-SHELL", "PGPASSWORD=\"$$POSTGRES_PASSWORD\" psql -h 127.0.0.1 -U \"$$POSTGRES_USER\" -d \"$$POSTGRES_DB\" -v ON_ERROR_STOP=1 -c 'SELECT 1' >/dev/null"]
      interval: 5s
      timeout: 3s
      retries: 10
"#.as_slice()),
            field(b"    image: ghcr.io/kuqmua/postgresql_16_with_pg_jsonschema@sha256:e95f0b8fce882a4cadbd7a1e01451a1440b356299af7a2a17859018914d0c178\n".as_slice()),
            field(b"    networks:\n      - application\n".as_slice()),
            field(b"    ports:\n      - \"127.0.0.1:5432:5432\"\n".as_slice()),
            field(b"    restart: unless-stopped\n".as_slice()),
            field(b"    volumes:\n      - postgres_local_data:/var/lib/postgresql/data\n".as_slice()),
        ),
        field(b"networks:\n  application:\n".as_slice()),
        crate::docker_compose_notification_database_service::DockerComposeNotificationDatabaseService::new(
            field(b"    environment:\n      POSTGRES_DB: notification_service\n      POSTGRES_PASSWORD: change-me\n      POSTGRES_USER: notification_service\n".as_slice()),
            field(b"    healthcheck:\n      test: [\"CMD-SHELL\", \"pg_isready -U notification_service -d notification_service\"]\n      interval: 5s\n      timeout: 3s\n      retries: 10\n".as_slice()),
            field(b"    image: postgres:16-bookworm@sha256:92620daddcd947f8d5ab5ba66e848702fe443d87fed30c4cea8e389fd78dfc55\n".as_slice()),
            field(b"    networks:\n      - application\n".as_slice()),
            field(b"    ports:\n      - \"127.0.0.1:5433:5432\"\n".as_slice()),
            field(b"    profiles: [application]\n".as_slice()),
            field(b"    volumes:\n      - notification_postgres_data:/var/lib/postgresql/data\n".as_slice()),
        ),
        crate::docker_compose_notification_service::DockerComposeNotificationService::new(
            field(b"    build:\n      context: .\n      dockerfile: notification_service/Dockerfile\n".as_slice()),
            field(b"    depends_on:\n      notification_database:\n        condition: service_healthy\n      notification_service_migrate:\n        condition: service_completed_successfully\n".as_slice()),
            field(b"    env_file:\n      - notification_service_config/.env\n".as_slice()),
            crate::docker_compose_notification_service_environment::DockerComposeNotificationServiceEnvironment::new(
                field(b"    environment:\n      NOTIFICATION_DATABASE_URL: \"postgres://notification_service:change-me@notification_database:5432/notification_service\"\n".as_slice()),
                field(b"      # BEGIN GENERATED COMPOSE SOCKET notification_service\n      NOTIFICATION_SERVICE_SOCKET_ADDRESS: \"0.0.0.0:8081\"\n      # END GENERATED COMPOSE SOCKET notification_service\n".as_slice()),
                field(b"      PG_POOL_MAX_CONNECTIONS: \"10\"\n".as_slice()),
                field(b"      REQUEST_TIMEOUT_SECONDS: \"30\"\n".as_slice()),
                field(b"      SVC_MODE: serve\n".as_slice()),
                field(b"      TRACING_FORMAT: text\n".as_slice()),
            ),
            field(b"    healthcheck:\n      # BEGIN GENERATED COMPOSE HEALTH notification_service\n      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:8081/health/ready/read\"]\n      # END GENERATED COMPOSE HEALTH notification_service\n      interval: 10s\n      timeout: 5s\n      retries: 12\n      start_period: 20s\n".as_slice()),
            field(b"    image: rust-workspace-template-notification-service:local\n".as_slice()),
            field(b"    networks:\n      - application\n".as_slice()),
            field(b"    # BEGIN GENERATED COMPOSE PORT notification_service\n    ports:\n      - \"127.0.0.1:8081:8081\"\n    # END GENERATED COMPOSE PORT notification_service\n".as_slice()),
            field(b"    profiles: [application]\n".as_slice()),
            field(b"    read_only: true\n".as_slice()),
            field(b"    restart: unless-stopped\n".as_slice()),
            field(b"    tmpfs:\n      - /tmp:size=16m,mode=1777\n".as_slice()),
        ),
        crate::docker_compose_notification_service_migrate_service::DockerComposeNotificationServiceMigrateService::new(
            field(b"    depends_on:\n      notification_database:\n        condition: service_healthy\n".as_slice()),
            field(b"    env_file:\n      - notification_service_config/.env\n".as_slice()),
            crate::docker_compose_notification_service_environment::DockerComposeNotificationServiceEnvironment::new(
                field(b"    environment:\n      NOTIFICATION_DATABASE_URL: \"postgres://notification_service:change-me@notification_database:5432/notification_service\"\n".as_slice()),
                field(b"      NOTIFICATION_SERVICE_SOCKET_ADDRESS: \"0.0.0.0:8081\"\n".as_slice()),
                field(b"      PG_POOL_MAX_CONNECTIONS: \"10\"\n".as_slice()),
                field(b"      REQUEST_TIMEOUT_SECONDS: \"30\"\n".as_slice()),
                field(b"      SVC_MODE: migrate\n".as_slice()),
                field(b"      TRACING_FORMAT: text\n".as_slice()),
            ),
            field(b"    image: rust-workspace-template-notification-service:local\n".as_slice()),
            field(b"    networks:\n      - application\n".as_slice()),
            field(b"    profiles: [application]\n".as_slice()),
            field(b"    read_only: true\n".as_slice()),
            field(b"    restart: \"no\"\n".as_slice()),
            field(b"    tmpfs:\n      - /tmp:size=16m,mode=1777\n".as_slice()),
        ),
        crate::docker_compose_server_service::DockerComposeServerService::new(
            field(b"    build:\n      context: .\n      dockerfile: Dockerfile\n".as_slice()),
            field(b"    depends_on:\n      database:\n        condition: service_healthy\n      server_migrate:\n        condition: service_completed_successfully\n".as_slice()),
            field(b"    env_file:\n      - server/.env\n".as_slice()),
            crate::docker_compose_server_environment::DockerComposeServerEnvironment::new(
                field(b"    environment:\n      DATABASE_URL: \"postgres://postgres:Dev-admin-2026-Ready!@database:5432/rust_workspace_template\"\n".as_slice()),
                field(b"      # BEGIN GENERATED COMPOSE SOCKET server\n      SERVICE_SOCKET_ADDRESS: \"0.0.0.0:8080\"\n      # END GENERATED COMPOSE SOCKET server\n".as_slice()),
                field(b"      SVC_MODE: serve\n".as_slice()),
            ),
            field(b"    healthcheck:\n      # BEGIN GENERATED COMPOSE HEALTH server\n      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:8080/health/ready/read\"]\n      # END GENERATED COMPOSE HEALTH server\n      interval: 10s\n      timeout: 5s\n      retries: 12\n      start_period: 20s\n".as_slice()),
            field(b"    image: rust-workspace-template-application:local\n".as_slice()),
            field(b"    networks:\n      - application\n".as_slice()),
            field(b"    # BEGIN GENERATED COMPOSE PORT server\n    ports:\n      - \"127.0.0.1:8080:8080\"\n    # END GENERATED COMPOSE PORT server\n".as_slice()),
            field(b"    profiles: [application]\n".as_slice()),
            field(b"    read_only: true\n".as_slice()),
            field(b"    restart: unless-stopped\n".as_slice()),
            field(b"    tmpfs:\n      - /tmp:size=16m,mode=1777\n".as_slice()),
        ),
        crate::docker_compose_server_migrate_service::DockerComposeServerMigrateService::new(
            field(b"    depends_on:\n      database:\n        condition: service_healthy\n".as_slice()),
            field(b"    env_file:\n      - server/.env\n".as_slice()),
            crate::docker_compose_server_environment::DockerComposeServerEnvironment::new(
                field(b"    environment:\n      DATABASE_URL: \"postgres://postgres:Dev-admin-2026-Ready!@database:5432/rust_workspace_template\"\n".as_slice()),
                field(b"      SERVICE_SOCKET_ADDRESS: \"0.0.0.0:8080\"\n".as_slice()),
                field(b"      SVC_MODE: migrate\n".as_slice()),
            ),
            field(b"    image: rust-workspace-template-application:local\n".as_slice()),
            field(b"    networks:\n      - application\n".as_slice()),
            field(b"    profiles: [application]\n".as_slice()),
            field(b"    read_only: true\n".as_slice()),
            field(b"    restart: \"no\"\n".as_slice()),
            field(b"    tmpfs:\n      - /tmp:size=16m,mode=1777\n".as_slice()),
        ),
        field(b"volumes:\n  notification_postgres_data:\n  postgres_local_data:\n".as_slice()),
    );
    let docker_compose_content = docker_compose_file.content();
    std::fs::write(
        workspace_root.join(constants_str::VALUE_E45E45BA),
        docker_compose_content.get_bounded_string().as_str(),
    )?;
    std::fs::write(
        workspace_root
            .join(constants_str::VALUE_B3EACD33)
            .join(constants_str::ENV),
        server_config::server_config::ServerConfig::env_example(),
    )?;
    std::fs::write(
        workspace_root
            .join(constants_str::VALUE_B3EACD33)
            .join(constants_str::ENV_EXAMPLE),
        server_config::server_config::ServerConfig::env_example(),
    )?;
    std::fs::write(
        workspace_root.join(constants_str::VALUE_0A7A2313),
        notification_service_config::notification_service_config::NotificationServiceConfig::env_example(),
    )?;
    std::fs::write(
        workspace_root
            .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
            .join(constants_str::ENV),
        notification_service_config::notification_service_config::NotificationServiceConfig::env_example(),
    )
}
