#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct MultipartTextPart {
    name: crate::multipart_field_name::MultipartFieldName,
    value: crate::multipart_text_value::MultipartTextValue,
}

impl MultipartTextPart {
    #[must_use]
    pub const fn name(&self) -> &crate::multipart_field_name::MultipartFieldName {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> &crate::multipart_text_value::MultipartTextValue {
        &self.value
    }
}
