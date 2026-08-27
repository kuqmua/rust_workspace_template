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
)]
#[serde(
    from = "bounded_types::domain_types::vector::BoundedVec<super::ApiProblemViolation, { constants_usize::ZERO }, 128usize>"
)]
pub(crate) struct ApiProblemViolations(
    bounded_types::domain_types::vector::BoundedVec<
        super::ApiProblemViolation,
        { constants_usize::ZERO },
        128usize,
    >,
);

impl utoipa::PartialSchema for ApiProblemViolations {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
            super::ApiProblemViolation,
            { constants_usize::ZERO },
            128usize,
        > as utoipa::PartialSchema>::schema()
    }
}

impl utoipa::ToSchema for ApiProblemViolations {}
