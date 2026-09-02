#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype::BoundedStringWrapper,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[bounded_string(max = 128usize)]
#[serde(try_from = "String")]
pub struct ApiProblemRequestId(
    bounded_types::bounded_string::BoundedString<0usize, 128usize, false>,
);
