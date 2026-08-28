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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminUserSummary>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminUserSummary, 10_000>)]
pub struct AdminUserSummaries(AdminBoundedVec<crate::domain_types::AdminUserSummary>);
impl TryFrom<Vec<crate::domain_types::AdminUserSummary>> for AdminUserSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminUserSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminUserSummaries {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminUserSummary] {
        self.0.as_slice()
    }
}
