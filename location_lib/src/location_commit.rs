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
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = crate::domain_types::LOC_COMMIT_MAX_LEN )]
#[serde(try_from = "String")]
pub struct LocationCommit(String);
