#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataPermissionsFlt(
    pub(crate) crate::admin_permissions::StdOptionalOptionalAdminPermissionsWhereMany,
);
