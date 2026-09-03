#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(crate))]
#[borrow]
pub struct AllowedOrigins(
    bounded_types::bounded_vec::BoundedVec<crate::allowed_origin::AllowedOrigin, 0, 128>,
);

impl TryFrom<Vec<String>> for AllowedOrigins {
    type Error = crate::allowed_origins_error::AllowedOriginsError;

    fn try_from(vec: Vec<String>) -> Result<Self, Self::Error> {
        let parsed = vec
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
