use super::AdminDataFilter;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(from = "crate::domain_types::collections::AdminBoundedVec<AdminDataFilter>")]
#[schema(value_type = crate::domain_types::collections::AdminOpenApiVec<AdminDataFilter, 100>)]
pub struct AdminDataFilters(crate::domain_types::collections::AdminBoundedVec<AdminDataFilter>);
impl TryFrom<Vec<AdminDataFilter>> for AdminDataFilters {
    type Error = crate::domain_types::AdminCollectionError;
    fn try_from(value: Vec<AdminDataFilter>) -> Result<Self, Self::Error> {
        crate::domain_types::collections::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataFilters {
    #[must_use]
    pub const fn as_slice(&self) -> &[AdminDataFilter] {
        self.0.as_slice()
    }
}
