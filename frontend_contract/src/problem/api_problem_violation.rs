#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct ApiProblemViolation {
    detail: super::ApiProblemDetail,
    field: super::ApiProblemField,
}
