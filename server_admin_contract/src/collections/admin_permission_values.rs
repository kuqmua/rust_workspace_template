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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminPermissionValue>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminPermissionValue, 10_000>)]
pub struct AdminPermissionValues(AdminBoundedVec<crate::domain_types::AdminPermissionValue>);
impl TryFrom<Vec<crate::domain_types::AdminPermissionValue>> for AdminPermissionValues {
    type Error = AdminCollectionError;
    fn try_from(
        value: Vec<crate::domain_types::AdminPermissionValue>,
    ) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
