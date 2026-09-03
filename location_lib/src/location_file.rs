#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(max = crate::domain_types::LOC_FILE_MAX_LEN )]
#[serde(try_from = "String")]
#[schema(value_type = bounded_types::bounded_string::BoundedString<0usize, { crate::domain_types::LOC_FILE_MAX_LEN }, false>)]
pub struct LocationFile(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::domain_types::LOC_FILE_MAX_LEN },
        false,
    >,
);
