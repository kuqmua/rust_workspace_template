#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[bounded_string(max = 128usize)]
#[serde(try_from = "String")]
pub struct ApiProblemField(bounded_types::bounded_string::BoundedString<0usize, 128usize, false>);
