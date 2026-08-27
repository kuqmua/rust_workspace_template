#![allow(clippy::single_call_fn)] // one bounded query serves the read-only table inspection boundary

#[path = "adapters_repository_data_tables/base_sql.rs"]
mod base_sql;
#[path = "adapters_repository_data_tables/data_columns.rs"]
mod data_columns;
#[path = "adapters_repository_data_tables/data_filter.rs"]
mod data_filter;
#[path = "adapters_repository_data_tables/data_flt.rs"]
mod data_flt;
#[path = "adapters_repository_data_tables/data_flt_json.rs"]
mod data_flt_json;
#[path = "adapters_repository_data_tables/data_permissions_flt.rs"]
mod data_permissions_flt;
#[path = "adapters_repository_data_tables/data_role_permissions_flt.rs"]
mod data_role_permissions_flt;
#[path = "adapters_repository_data_tables/data_roles_flt.rs"]
mod data_roles_flt;
#[path = "adapters_repository_data_tables/data_system_settings_flt.rs"]
mod data_system_settings_flt;
#[path = "adapters_repository_data_tables/data_user_roles_flt.rs"]
mod data_user_roles_flt;
#[path = "adapters_repository_data_tables/data_users_flt.rs"]
mod data_users_flt;
#[path = "adapters_repository_data_tables/filtered_sql.rs"]
mod filtered_sql;
#[path = "adapters_repository_data_tables/read.rs"]
mod read;

use super::{AdminPageTotalCount, AdminRepositoryError, SqlxAdminRepositoryPoolRef, page_total};
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

#[cfg(test)]
#[path = "adapters_repository_data_tables_tests.rs"]
mod tests;
