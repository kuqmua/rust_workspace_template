#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct DockerComposeServerEnvironment {
    database_url: crate::configuration_field::ConfigurationField,
    service_socket_address: crate::configuration_field::ConfigurationField,
    svc_mode: crate::configuration_field::ConfigurationField,
}

impl DockerComposeServerEnvironment {
    pub(crate) fn content(
        &self,
        docker_compose_server_environment_order: &crate::docker_compose_server_environment_order::DockerComposeServerEnvironmentOrder,
    ) -> crate::std_byte_vector::StdByteVector {
        let mut std_byte_vector = crate::std_byte_vector::StdByteVector::default();
        self.get_database_url().append_to(&mut std_byte_vector);
        match docker_compose_server_environment_order {
            crate::docker_compose_server_environment_order::DockerComposeServerEnvironmentOrder::Migrate => {
                self.get_service_socket_address()
                    .append_to(&mut std_byte_vector);
                self.get_svc_mode().append_to(&mut std_byte_vector);
            }
            crate::docker_compose_server_environment_order::DockerComposeServerEnvironmentOrder::Serve => {
                self.get_svc_mode().append_to(&mut std_byte_vector);
                self.get_service_socket_address()
                    .append_to(&mut std_byte_vector);
            }
        }
        std_byte_vector
    }
}
