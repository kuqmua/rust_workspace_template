#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct AllowedOrigins(
    bounded_types::bounded_vec::BoundedVec<crate::allowed_origin::AllowedOrigin, 0, 128>,
);

impl AllowedOrigins {
    pub(crate) const fn get(
        &self,
    ) -> &bounded_types::bounded_vec::BoundedVec<crate::allowed_origin::AllowedOrigin, 0, 128> {
        &self.0
    }
}

impl TryFrom<Vec<String>> for AllowedOrigins {
    type Error = crate::allowed_origins_error::AllowedOriginsError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let parsed = value
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
