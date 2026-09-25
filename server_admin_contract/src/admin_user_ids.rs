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
#[serde(from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_user_id::AdminUserId>")]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_user_id::AdminUserId, 10_000>)]
pub struct AdminUserIds(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_user_id::AdminUserId>,
);

impl TryFrom<Vec<crate::admin_user_id::AdminUserId>> for AdminUserIds {
    type Error = crate::admin_collection_error::AdminCollectionError;

    fn try_from(value: Vec<crate::admin_user_id::AdminUserId>) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self::from)
    }
}
