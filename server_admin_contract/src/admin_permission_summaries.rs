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
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_permission_summary::AdminPermissionSummary>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_permission_summary::AdminPermissionSummary, 10_000>)]
pub struct AdminPermissionSummaries(
    crate::admin_bounded_vec::AdminBoundedVec<
        crate::admin_permission_summary::AdminPermissionSummary,
    >,
);
impl TryFrom<Vec<crate::admin_permission_summary::AdminPermissionSummary>>
    for AdminPermissionSummaries
{
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_permission_summary::AdminPermissionSummary>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminPermissionSummaries {
    pub(crate) const fn as_slice(
        &self,
    ) -> &[crate::admin_permission_summary::AdminPermissionSummary] {
        self.0.as_slice()
    }
}
