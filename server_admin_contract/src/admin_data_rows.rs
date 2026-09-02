#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
#[serde(from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_data_row::AdminDataRow>")]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_data_row::AdminDataRow, 10_000>)]
pub struct AdminDataRows(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_data_row::AdminDataRow>,
);
impl TryFrom<Vec<crate::admin_data_row::AdminDataRow>> for AdminDataRows {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(vec: Vec<crate::admin_data_row::AdminDataRow>) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(vec).map(Self)
    }
}
impl AdminDataRows {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_data_row::AdminDataRow] {
        self.0.as_slice()
    }
}
