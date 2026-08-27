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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminDataTable>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminDataTable, 10_000>)]
pub struct AdminDataTables(AdminBoundedVec<crate::domain_types::AdminDataTable>);
impl TryFrom<Vec<crate::domain_types::AdminDataTable>> for AdminDataTables {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminDataTable>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataTables {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminDataTable] {
        self.0.as_slice()
    }
}
