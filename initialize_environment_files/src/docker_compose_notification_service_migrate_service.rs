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

impl DockerComposeNotificationServiceMigrateService {
    pub(crate) fn content(&self) -> crate::std_byte_vector::StdByteVector {
        let mut std_byte_vector = crate::std_byte_vector::StdByteVector::default();
        crate::configuration_field::ConfigurationField::from(
            b"  notification_service_migrate:\n".as_slice(),
        )
        .append_to(&mut std_byte_vector);
        self.get_profiles().append_to(&mut std_byte_vector);
        self.get_image().append_to(&mut std_byte_vector);
        self.get_depends_on().append_to(&mut std_byte_vector);
        self.get_env_file().append_to(&mut std_byte_vector);
        self.get_environment().append_to(&mut std_byte_vector);
        self.get_networks().append_to(&mut std_byte_vector);
        self.get_read_only().append_to(&mut std_byte_vector);
        self.get_restart().append_to(&mut std_byte_vector);
        self.get_tmpfs().append_to(&mut std_byte_vector);
        std_byte_vector
    }
}
