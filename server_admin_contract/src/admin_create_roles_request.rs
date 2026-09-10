#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_create_role_request::AdminCreateRoleRequest>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_create_role_request::AdminCreateRoleRequest, 10_000>)]
pub struct AdminCreateRolesRequest(
    crate::admin_bounded_vec::AdminBoundedVec<
        crate::admin_create_role_request::AdminCreateRoleRequest,
    >,
);
impl TryFrom<Vec<crate::admin_create_role_request::AdminCreateRoleRequest>>
    for AdminCreateRolesRequest
{
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_create_role_request::AdminCreateRoleRequest>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self::from)
    }
}
