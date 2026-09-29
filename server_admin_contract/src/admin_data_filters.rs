#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(
    from = "bounded_types::bounded_vec::BoundedVec<crate::admin_data_filter::AdminDataFilter, 0, 100>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_data_filter::AdminDataFilter, 100>)]
pub struct AdminDataFilters(
    bounded_types::bounded_vec::BoundedVec<crate::admin_data_filter::AdminDataFilter, 0, 100>,
);
impl TryFrom<Vec<crate::admin_data_filter::AdminDataFilter>> for AdminDataFilters {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_data_filter::AdminDataFilter>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(crate::admin_collection_error::AdminCollectionError::TooLong)
    }
}
impl AdminDataFilters {
    #[must_use]
    pub const fn as_slice(&self) -> &[crate::admin_data_filter::AdminDataFilter] {
        self.0.as_slice()
    }
}
