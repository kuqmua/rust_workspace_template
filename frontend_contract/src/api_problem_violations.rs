#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::FromInner,
    serde::Deserialize,
    serde::Serialize,
    newtype::UtoipaSchema,
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
