use super::{AdminBoundedVec, AdminCollectionError, AdminOpenApiVec};

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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminRoleName>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminRoleName, 10_000>)]
pub struct AdminRoleNames(AdminBoundedVec<crate::domain_types::AdminRoleName>);
impl TryFrom<Vec<crate::domain_types::AdminRoleName>> for AdminRoleNames {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminRoleName>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleNames {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminRoleName] {
        self.0.as_slice()
    }
}
