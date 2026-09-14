#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
pub(crate) struct ConfigurationField {
    bounded_string: bounded_types::bounded_string::BoundedString,
}

impl ConfigurationField {
    pub(crate) fn append_to(&self, std_byte_vector: &mut crate::std_byte_vector::StdByteVector) {
        std_byte_vector.append_configuration_field(self);
    }

    pub(crate) const fn value(&self) -> &str {
        self.bounded_string.as_str()
    }
}

impl From<&'static [u8]> for ConfigurationField {
    fn from(value: &'static [u8]) -> Self {
        Self::new(
            bounded_types::bounded_string::BoundedString::from_unbounded(
                String::from_utf8_lossy(value).into_owned(),
            ),
        )
    }
}
