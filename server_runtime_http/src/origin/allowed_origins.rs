#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigins(
    pub(super) bounded_types::domain_types::vector::BoundedVec<super::AllowedOrigin, 0, 128>,
);

impl TryFrom<Vec<String>> for AllowedOrigins {
    type Error = super::AllowedOriginsError;

    fn try_from(values: Vec<String>) -> Result<Self, Self::Error> {
        let parsed = values
            .into_iter()
            .map(super::AllowedOrigin::try_from)
            .collect::<Result<Vec<super::AllowedOrigin>, super::AllowedOriginError>>()
            .map_err(|_error| super::AllowedOriginsError)?;
        bounded_types::domain_types::vector::BoundedVec::try_from(parsed)
            .map(Self)
            .map_err(super::AllowedOriginsError::from)
    }
}
