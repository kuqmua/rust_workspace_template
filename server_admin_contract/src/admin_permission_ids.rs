use super::{AdminBoundedVec, AdminCollectionError, AdminEmptyCollection, AdminOpenApiVec};

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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminPermissionId>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminPermissionId, 10_000>)]
pub struct AdminPermissionIds(AdminBoundedVec<crate::domain_types::AdminPermissionId>);
impl TryFrom<Vec<crate::domain_types::AdminPermissionId>> for AdminPermissionIds {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminPermissionId>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminPermissionIds {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminPermissionId] {
        self.0.as_slice()
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminPermissionIds {
    fn default() -> Self {
        Self::from(AdminEmptyCollection)
    }
}
impl From<AdminEmptyCollection> for AdminPermissionIds {
    fn from(_value: AdminEmptyCollection) -> Self {
        Self(AdminBoundedVec::from([]))
    }
}
