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
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_data_filter::AdminDataFilter>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_data_filter::AdminDataFilter, 100>)]
pub struct AdminDataFilters(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_data_filter::AdminDataFilter>,
);
impl TryFrom<Vec<crate::admin_data_filter::AdminDataFilter>> for AdminDataFilters {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_data_filter::AdminDataFilter>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataFilters {
    #[must_use]
    pub const fn as_slice(&self) -> &[crate::admin_data_filter::AdminDataFilter] {
        self.0.as_slice()
    }
}
