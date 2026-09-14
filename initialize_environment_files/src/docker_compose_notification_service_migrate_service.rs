#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct DockerComposeNotificationServiceMigrateService {
    depends_on: crate::configuration_field::ConfigurationField,
    env_file: crate::configuration_field::ConfigurationField,
    environment: crate::configuration_field::ConfigurationField,
    image: crate::configuration_field::ConfigurationField,
    networks: crate::configuration_field::ConfigurationField,
    profiles: crate::configuration_field::ConfigurationField,
    read_only: crate::configuration_field::ConfigurationField,
    restart: crate::configuration_field::ConfigurationField,
    tmpfs: crate::configuration_field::ConfigurationField,
}
