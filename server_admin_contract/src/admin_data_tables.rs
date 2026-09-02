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
#[serde(
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_data_table::AdminDataTable>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_data_table::AdminDataTable, 10_000>)]
pub struct AdminDataTables(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_data_table::AdminDataTable>,
);
impl TryFrom<Vec<crate::admin_data_table::AdminDataTable>> for AdminDataTables {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(vec: Vec<crate::admin_data_table::AdminDataTable>) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(vec).map(Self)
    }
}
impl AdminDataTables {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_data_table::AdminDataTable] {
        self.0.as_slice()
    }
}
