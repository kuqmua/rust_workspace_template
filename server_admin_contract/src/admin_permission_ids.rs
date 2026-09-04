#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_permission_id::AdminPermissionId>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_permission_id::AdminPermissionId, 10_000>)]
pub struct AdminPermissionIds(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_permission_id::AdminPermissionId>,
);
impl TryFrom<Vec<crate::admin_permission_id::AdminPermissionId>> for AdminPermissionIds {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_permission_id::AdminPermissionId>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminPermissionIds {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_permission_id::AdminPermissionId] {
        self.0.as_slice()
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminPermissionIds {
    fn default() -> Self {
        Self::from(crate::admin_empty_collection::AdminEmptyCollection)
    }
}
impl From<crate::admin_empty_collection::AdminEmptyCollection> for AdminPermissionIds {
    fn from(value: crate::admin_empty_collection::AdminEmptyCollection) -> Self {
        let _: crate::admin_empty_collection::AdminEmptyCollection = value;
        Self(crate::admin_bounded_vec::AdminBoundedVec::from([]))
    }
}
