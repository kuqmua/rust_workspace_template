use bounded_types::bounded_string::BoundedString;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = crate::domain_types::LOC_FILE_MAX_LEN )]
#[serde(try_from = "String")]
pub struct LocationFile(BoundedString<0usize, { crate::domain_types::LOC_FILE_MAX_LEN }, false>);
