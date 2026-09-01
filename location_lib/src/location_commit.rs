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
)]
#[bounded_string(max = crate::domain_types::LOC_COMMIT_MAX_LEN )]
#[serde(try_from = "String")]
#[schema(value_type = bounded_types::bounded_string::BoundedString<0usize, { crate::domain_types::LOC_COMMIT_MAX_LEN }, false>)]
pub struct LocationCommit(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::domain_types::LOC_COMMIT_MAX_LEN },
        false,
    >,
);
