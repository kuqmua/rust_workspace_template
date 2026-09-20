#[test]
fn test_generated_table_cells_read_explicit_values() {
    let timestamp = constants_str::VALUE_BA5B49F1;
    let item = serde_json::json!({
        (constants_str::SQL_NAMES_ID): pg_crud_common::explicit_value::ExplicitValue::new(1i64),
        (constants_str::ROLE_ID): pg_crud_common::explicit_value::ExplicitValue::new(2i64),
        (constants_str::PERMISSION_ID): pg_crud_common::explicit_value::ExplicitValue::new(3i64),
        (constants_str::CREATED_AT): pg_crud_common::explicit_value::ExplicitValue::new(timestamp),
    });
    let view = crate::generated_data_table_view::generated_data_table_view::<
        _,
        crate::admin_role_permissions_read_page_error::AdminRolePermissionsReadPageError,
    >(
        pg_crud_common::list_items::ListItems::from(vec![item]),
        pg_crud_common::list_total::ListTotal::from(1u32),
        server_admin_contract::admin_data_table::AdminDataTable::RolePermissions,
    );
    assert!(view.is_ok_and(|view| {
        view.table() == server_admin_contract::admin_data_table::AdminDataTable::RolePermissions
            && u64::from(view.total()) == 1u64
            && view.items().len() == 1usize
            && view.items().first().is_some_and(|row| {
                row.values()
                    .iter()
                    .map(|admin_text| admin_text.as_ref().as_str())
                    .eq([
                        1i64.to_string().as_str(),
                        2i64.to_string().as_str(),
                        3i64.to_string().as_str(),
                        timestamp,
                    ])
            })
    }));
}
