#[derive(generate_accessor::Getters)]
#[getters(bare)]
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
