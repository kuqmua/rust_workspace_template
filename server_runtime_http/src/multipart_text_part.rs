#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct MultipartTextPart {
    name: super::MultipartFieldName,
    value: super::MultipartTextValue,
}

impl MultipartTextPart {
    #[must_use]
    pub const fn name(&self) -> &super::MultipartFieldName {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> &super::MultipartTextValue {
        &self.value
    }
}
