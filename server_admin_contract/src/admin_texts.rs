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
#[serde(from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_text::AdminText>")]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_text::AdminText, 10_000>)]
pub struct AdminTexts(crate::admin_bounded_vec::AdminBoundedVec<crate::admin_text::AdminText>);
impl TryFrom<Vec<crate::admin_text::AdminText>> for AdminTexts {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(value: Vec<crate::admin_text::AdminText>) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminTexts {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_text::AdminText] {
        self.0.as_slice()
    }
}
