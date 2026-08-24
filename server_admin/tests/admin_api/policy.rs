#[test]
fn generated_admin_users_descriptor_keeps_sensitive_and_server_owned_fields_excluded() {
    let read_excluded = <server_admin::generated_tables::AdminUsers as pg_crud_common::DbTableSchema>::read_excluded_columns();
    assert!(
        read_excluded
            .iter()
            .any(|field| field.as_ref() == constants_str::PASSWORD_HASH)
    );
    let create_excluded = <server_admin::generated_tables::AdminUsers as pg_crud_common::DbTableSchema>::create_excluded_columns();
    assert!(
        create_excluded
            .iter()
            .any(|field| field.as_ref() == constants_str::PASSWORD_HASH)
    );
}
