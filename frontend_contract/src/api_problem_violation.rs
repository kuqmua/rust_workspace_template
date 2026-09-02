#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct ApiProblemViolation {
    detail: crate::api_problem_detail::ApiProblemDetail,
    field: crate::api_problem_field::ApiProblemField,
}
