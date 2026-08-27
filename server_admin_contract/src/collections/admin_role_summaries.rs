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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminRoleSummary>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminRoleSummary, 10_000>)]
pub struct AdminRoleSummaries(AdminBoundedVec<crate::domain_types::AdminRoleSummary>);
impl TryFrom<Vec<crate::domain_types::AdminRoleSummary>> for AdminRoleSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminRoleSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleSummaries {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminRoleSummary] {
        self.0.as_slice()
    }
}
