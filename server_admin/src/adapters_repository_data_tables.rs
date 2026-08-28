// The owner module retains lint-sensitive semantics from the original implementation.

#![allow(clippy::field_scoped_visibility_modifiers, clippy::wildcard_imports)] // split repository adapters expose private wrappers and vocabulary only through this facade

use crate::{
    AdminPageTotalCount, AdminRepositoryError, SqlxAdminRepositoryPoolRef, repository_page_total,
};
use base_sql::base_sql;
use data_columns::data_columns;
use data_filter::data_filter;
pub(crate) use data_flt::DataFlt;
use data_flt_json::*;
pub(crate) use data_permissions_flt::DataPermissionsFlt;
pub(crate) use data_role_permissions_flt::DataRolePermissionsFlt;
pub(crate) use data_roles_flt::DataRolesFlt;
pub(crate) use data_system_settings_flt::DataSystemSettingsFlt;
pub(crate) use data_user_roles_flt::DataUserRolesFlt;
pub(crate) use data_users_flt::DataUsersFlt;
use filtered_sql::filtered_sql;
pub(crate) use read::read;

// Root-owned module compatibility wrappers.
mod base_sql {
    pub use crate::base_sql::*;
}
mod data_columns {
    pub use crate::data_columns::*;
}
mod data_filter {
    pub use crate::data_filter::*;
}
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
mod filtered_sql {
    pub use crate::filtered_sql::*;
}
mod read {
    pub use crate::read::*;
}
