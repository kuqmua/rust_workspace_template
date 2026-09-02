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
#[serde(from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_role_id::AdminRoleId>")]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_role_id::AdminRoleId, 10_000>)]
pub struct AdminRoleIds(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_role_id::AdminRoleId>,
);
impl TryFrom<Vec<crate::admin_role_id::AdminRoleId>> for AdminRoleIds {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(vec: Vec<crate::admin_role_id::AdminRoleId>) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(vec).map(Self)
    }
}
impl AdminRoleIds {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_role_id::AdminRoleId] {
        self.0.as_slice()
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminRoleIds {
    fn default() -> Self {
        Self::from(crate::admin_empty_collection::AdminEmptyCollection)
    }
}
impl From<crate::admin_empty_collection::AdminEmptyCollection> for AdminRoleIds {
    fn from(admin_empty_collection: crate::admin_empty_collection::AdminEmptyCollection) -> Self {
        let _: crate::admin_empty_collection::AdminEmptyCollection = admin_empty_collection;
        Self(crate::admin_bounded_vec::AdminBoundedVec::from([]))
    }
}
