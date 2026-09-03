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
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_permission_value::AdminPermissionValue>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_permission_value::AdminPermissionValue, 10_000>)]
pub struct AdminPermissionValues(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_permission_value::AdminPermissionValue>,
);
impl TryFrom<Vec<crate::admin_permission_value::AdminPermissionValue>> for AdminPermissionValues {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        vec: Vec<crate::admin_permission_value::AdminPermissionValue>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(vec).map(Self)
    }
}
