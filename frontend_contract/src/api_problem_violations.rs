#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
    serde::Deserialize,
    serde::Serialize,
    proc_macro_newtype_utoipa_schema::UtoipaSchema,
)]
#[serde(
    from = "bounded_types::bounded_vec::BoundedVec<crate::api_problem_violation::ApiProblemViolation, { constants_usize::ZERO }, 128usize>"
)]
pub(crate) struct ApiProblemViolations(
    bounded_types::bounded_vec::BoundedVec<
        crate::api_problem_violation::ApiProblemViolation,
        { constants_usize::ZERO },
        128usize,
    >,
);
