#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(
    from = "bounded_types::bounded_vec::BoundedVec<crate::admin_optional_setting::AdminOptionalSetting, 0, 6>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_optional_setting::AdminOptionalSetting, 6>)]
pub struct AdminOptionalSettings(
    bounded_types::bounded_vec::BoundedVec<
        crate::admin_optional_setting::AdminOptionalSetting,
        0,
        6,
    >,
);
impl TryFrom<Vec<crate::admin_optional_setting::AdminOptionalSetting>> for AdminOptionalSettings {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_optional_setting::AdminOptionalSetting>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(crate::admin_collection_error::AdminCollectionError::TooLong)
    }
}
