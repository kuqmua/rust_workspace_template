#[path = "admin_data_columns_csv_ref.rs"]
mod admin_data_columns_csv_ref;
#[path = "admin_data_order_ref.rs"]
mod admin_data_order_ref;
#[path = "admin_data_table.rs"]
mod admin_data_table;
#[path = "admin_data_table_spec.rs"]
mod admin_data_table_spec;
#[path = "admin_data_table_str_ref.rs"]
mod admin_data_table_str_ref;
#[path = "admin_permission.rs"]
mod admin_permission;
#[path = "admin_permission_str_ref.rs"]
mod admin_permission_str_ref;
#[path = "admin_permission_value.rs"]
mod admin_permission_value;

pub use admin_data_columns_csv_ref::AdminDataColumnsCsvRef;
pub use admin_data_order_ref::AdminDataOrderRef;
pub use admin_data_table::{AdminDataTable, AdminDataTableTryFromStrError};
pub use admin_data_table_spec::AdminDataTableSpec;
pub use admin_data_table_str_ref::AdminDataTableStrRef;
pub use admin_permission::{AdminPermission, AdminPermissionTryFromStrError};
pub use admin_permission_str_ref::AdminPermissionStrRef;
pub use admin_permission_value::AdminPermissionValue;

#[cfg(test)]
mod tests {
    #[test]
    fn user_table_requires_user_read_permission() {
        assert_eq!(
            super::AdminDataTable::Users.permission(),
            super::AdminPermission::UsersRead,
        );
    }
}
