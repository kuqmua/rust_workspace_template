use super::AdminDataColumn;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(from = "crate::domain_types::collections::AdminBoundedVec<AdminDataColumn>")]
#[schema(value_type = crate::domain_types::collections::AdminOpenApiVec<AdminDataColumn, 10_000>)]
pub struct AdminDataColumns(crate::domain_types::collections::AdminBoundedVec<AdminDataColumn>);
impl TryFrom<Vec<AdminDataColumn>> for AdminDataColumns {
    type Error = crate::domain_types::AdminCollectionError;
    fn try_from(value: Vec<AdminDataColumn>) -> Result<Self, Self::Error> {
        crate::domain_types::collections::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataColumns {
    #[must_use]
    pub const fn as_slice(&self) -> &[AdminDataColumn] {
        self.0.as_slice()
    }
}
