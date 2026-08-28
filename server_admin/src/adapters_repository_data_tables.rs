// The owner module retains lint-sensitive semantics from the original implementation.

#![allow(clippy::field_scoped_visibility_modifiers, clippy::wildcard_imports)] // split repository adapters expose private wrappers and vocabulary only through this facade

use crate::{
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
    pub use crate::data_flt::*;
}
mod data_flt_json {
    pub use crate::data_flt_json::*;
}
mod data_permissions_flt {
    pub use crate::data_permissions_flt::*;
}
mod data_role_permissions_flt {
    pub use crate::data_role_permissions_flt::*;
}
mod data_roles_flt {
    pub use crate::data_roles_flt::*;
}
mod data_system_settings_flt {
    pub use crate::data_system_settings_flt::*;
}
mod data_user_roles_flt {
    pub use crate::data_user_roles_flt::*;
}
mod data_users_flt {
    pub use crate::data_users_flt::*;
}
