pub use crate::admin_table_sort_field::AdminTableSortField;
pub use crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;
pub use crate::admin_table_sort_key_ref::AdminTableSortKeyRef;
use crate::admin_table_sort_values::AdminTableSortValues;

#[cfg(test)]
mod tests {
    #[test]
    fn user_login_sort_field_has_login_key() {
        assert_eq!(
            super::AdminTableSortField::UserLogin.key().as_ref(),
            constants_str::LOGIN,
        );
    }
}
