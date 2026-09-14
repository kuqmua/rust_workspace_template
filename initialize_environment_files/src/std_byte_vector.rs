#[derive(
    Default, proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_getters::Getters,
)]
pub(crate) struct StdByteVector {
    bounded_string: bounded_types::bounded_string::BoundedString,
}

impl StdByteVector {
    pub(crate) fn append_configuration_field(
        &mut self,
        configuration_field: &crate::configuration_field::ConfigurationField,
    ) {
        self.bounded_string
            .as_mut_string()
            .push_str(configuration_field.value());
    }

    pub(crate) fn append_std_byte_vector(&mut self, std_byte_vector: &Self) {
        self.bounded_string
            .as_mut_string()
            .push_str(std_byte_vector.bounded_string.as_str());
    }
}
