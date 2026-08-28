pub use crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef;
pub use crate::admin_data_order_ref::AdminDataOrderRef;
pub use crate::admin_data_table::{AdminDataTable, AdminDataTableTryFromStrError};
pub use crate::admin_data_table_spec::AdminDataTableSpec;
pub use crate::admin_data_table_str_ref::AdminDataTableStrRef;
pub use crate::admin_permission::{AdminPermission, AdminPermissionTryFromStrError};
pub use crate::admin_permission_str_ref::AdminPermissionStrRef;
pub use crate::admin_permission_value::AdminPermissionValue;

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
