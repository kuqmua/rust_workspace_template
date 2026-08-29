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
#[serde(from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_role_name::AdminRoleName>")]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_role_name::AdminRoleName, 10_000>)]
pub struct AdminRoleNames(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_role_name::AdminRoleName>,
);
impl TryFrom<Vec<crate::admin_role_name::AdminRoleName>> for AdminRoleNames {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(value: Vec<crate::admin_role_name::AdminRoleName>) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleNames {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_role_name::AdminRoleName] {
        self.0.as_slice()
    }
}
