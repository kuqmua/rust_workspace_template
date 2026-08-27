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
#[serde(from = "AdminBoundedVec<crate::domain_types::AdminOptionalSetting>")]
#[schema(value_type = AdminOpenApiVec<crate::domain_types::AdminOptionalSetting, 10_000>)]
pub struct AdminOptionalSettings(AdminBoundedVec<crate::domain_types::AdminOptionalSetting>);
impl TryFrom<Vec<crate::domain_types::AdminOptionalSetting>> for AdminOptionalSettings {
    type Error = AdminCollectionError;
    fn try_from(
        value: Vec<crate::domain_types::AdminOptionalSetting>,
    ) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
