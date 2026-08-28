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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminText>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminText, 10_000>)]
pub struct AdminTexts(AdminBoundedVec<crate::domain_types::AdminText>);
impl TryFrom<Vec<crate::domain_types::AdminText>> for AdminTexts {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<crate::domain_types::AdminText>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminTexts {
    pub(crate) const fn as_slice(&self) -> &[crate::domain_types::AdminText] {
        self.0.as_slice()
    }
}
