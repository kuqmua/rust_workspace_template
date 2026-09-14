#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct DockerComposeNotificationDatabaseService {
    environment: crate::configuration_field::ConfigurationField,
    healthcheck: crate::configuration_field::ConfigurationField,
    image: crate::configuration_field::ConfigurationField,
    networks: crate::configuration_field::ConfigurationField,
    ports: crate::configuration_field::ConfigurationField,
    profiles: crate::configuration_field::ConfigurationField,
    volumes: crate::configuration_field::ConfigurationField,
}
