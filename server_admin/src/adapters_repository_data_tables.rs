// The owner module retains lint-sensitive semantics from the original implementation.

use super::{
    AdminPageTotalCount, AdminRepositoryError, SqlxAdminRepositoryPoolRef, repository_page_total,
};
pub(crate) use data_flt::DataFlt;
use data_flt_json::*;
pub(crate) use data_permissions_flt::DataPermissionsFlt;
pub(crate) use data_role_permissions_flt::DataRolePermissionsFlt;
pub(crate) use data_roles_flt::DataRolesFlt;
pub(crate) use data_system_settings_flt::DataSystemSettingsFlt;
pub(crate) use data_user_roles_flt::DataUserRolesFlt;
pub(crate) use data_users_flt::DataUsersFlt;

// Root-owned module compatibility wrappers.
mod data_flt {
    pub use super::super::data_flt::*;
}
mod data_flt_json {
    pub use super::super::data_flt_json::*;
}
mod data_permissions_flt {
    pub use super::super::data_permissions_flt::*;
}
mod data_role_permissions_flt {
    pub use super::super::data_role_permissions_flt::*;
}
mod data_roles_flt {
    pub use super::super::data_roles_flt::*;
}
mod data_system_settings_flt {
    pub use super::super::data_system_settings_flt::*;
}
mod data_user_roles_flt {
    pub use super::super::data_user_roles_flt::*;
}
mod data_users_flt {
    pub use super::super::data_users_flt::*;
}
