#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct DockerComposeDatabaseService {
    environment: crate::configuration_field::ConfigurationField,
    healthcheck: crate::configuration_field::ConfigurationField,
    image: crate::configuration_field::ConfigurationField,
    networks: crate::configuration_field::ConfigurationField,
    ports: crate::configuration_field::ConfigurationField,
    restart: crate::configuration_field::ConfigurationField,
    volumes: crate::configuration_field::ConfigurationField,
}

impl DockerComposeDatabaseService {
    pub(crate) fn content(&self) -> crate::std_byte_vector::StdByteVector {
        let mut std_byte_vector = crate::std_byte_vector::StdByteVector::default();
        crate::configuration_field::ConfigurationField::from(b"  database:\n".as_slice())
            .append_to(&mut std_byte_vector);
        self.get_restart().append_to(&mut std_byte_vector);
        self.get_image().append_to(&mut std_byte_vector);
        self.get_environment().append_to(&mut std_byte_vector);
        self.get_healthcheck().append_to(&mut std_byte_vector);
        self.get_networks().append_to(&mut std_byte_vector);
        self.get_ports().append_to(&mut std_byte_vector);
        self.get_volumes().append_to(&mut std_byte_vector);
        std_byte_vector
    }
}
