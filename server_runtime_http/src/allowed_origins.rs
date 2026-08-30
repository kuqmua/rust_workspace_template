#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigins(
    pub(super) bounded_types::bounded_vec::BoundedVec<crate::allowed_origin::AllowedOrigin, 0, 128>,
);

impl TryFrom<Vec<String>> for AllowedOrigins {
    type Error = crate::allowed_origins_error::AllowedOriginsError;

    fn try_from(values: Vec<String>) -> Result<Self, Self::Error> {
        let parsed = values
            .into_iter()
            .map(crate::allowed_origin::AllowedOrigin::try_from)
            .collect::<Result<
                Vec<crate::allowed_origin::AllowedOrigin>,
                crate::allowed_origin_error::AllowedOriginError,
            >>()
            .map_err(|_error| crate::allowed_origins_error::AllowedOriginsError::Invalid)?;
        bounded_types::bounded_vec::BoundedVec::try_from(parsed)
            .map(Self)
            .map_err(crate::allowed_origins_error::AllowedOriginsError::from)
    }
}
