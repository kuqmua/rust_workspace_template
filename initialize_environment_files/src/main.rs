mod configuration_field;
mod docker_compose_content;
mod docker_compose_database_service;
mod docker_compose_file;
mod docker_compose_notification_database_service;
mod docker_compose_notification_service;
mod docker_compose_notification_service_migrate_service;
mod docker_compose_server_migrate_service;
mod docker_compose_server_service;
mod generate_environment_files;
mod notification_service_environment_content;
mod notification_service_environment_file;
mod server_environment_content;
mod server_environment_file;
#[cfg(test)]
mod test_initialize_environment_files;

fn main() -> std::io::Result<()> {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    generate_environment_files::generate_environment_files(workspace_root)
}
