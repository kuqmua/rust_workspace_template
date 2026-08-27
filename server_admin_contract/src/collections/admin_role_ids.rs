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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminRoleId>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminRoleId, 10_000>)]
pub struct AdminRoleIds(AdminBoundedVec<crate::domain_types::AdminRoleId>);
impl TryFrom<Vec<crate::domain_types::AdminRoleId>> for AdminRoleIds {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminRoleId>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleIds {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminRoleId] {
        self.0.as_slice()
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminRoleIds {
    fn default() -> Self {
        Self::from(AdminEmptyCollection)
    }
}
impl From<AdminEmptyCollection> for AdminRoleIds {
    fn from(_value: AdminEmptyCollection) -> Self {
        Self(AdminBoundedVec::from([]))
    }
}
