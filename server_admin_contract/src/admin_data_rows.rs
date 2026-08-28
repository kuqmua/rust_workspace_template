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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminDataRow>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminDataRow, 10_000>)]
pub struct AdminDataRows(AdminBoundedVec<crate::domain_types::AdminDataRow>);
impl TryFrom<Vec<crate::domain_types::AdminDataRow>> for AdminDataRows {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminDataRow>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataRows {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminDataRow] {
        self.0.as_slice()
    }
}
