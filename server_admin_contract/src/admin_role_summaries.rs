#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_role_summary::AdminRoleSummary>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_role_summary::AdminRoleSummary, 10_000>)]
pub struct AdminRoleSummaries(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_role_summary::AdminRoleSummary>,
);
impl TryFrom<Vec<crate::admin_role_summary::AdminRoleSummary>> for AdminRoleSummaries {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_role_summary::AdminRoleSummary>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleSummaries {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_role_summary::AdminRoleSummary] {
        self.0.as_slice()
    }
}
