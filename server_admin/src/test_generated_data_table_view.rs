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
        None,
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

#[test]
fn test_audit_log_cells_read_explicit_values() {
    let admin_data_table = server_admin_contract::admin_data_table::AdminDataTable::AuditLog;
    let columns = crate::admin_data_columns::admin_data_columns(
        admin_data_table,
        crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table),
    )
    .map_err(crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError::from)
    .and_then(|columns| {
        let fields = crate::admin_audit_log::AdminAuditLog::frontend_fields();
        server_admin_contract::admin_data_columns::AdminDataColumns::try_from(
            columns
                .as_slice()
                .iter()
                .filter(|column| {
                    fields.as_ref().iter().any(|field| {
                        field.name().as_ref() == column.name().as_ref()
                            && field.readable()
                                == frontend_contract::field_capability::FieldCapability::Enabled
                    })
                })
                .cloned()
                .collect::<Vec<_>>(),
        )
        .map_err(crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError::from)
    });
    assert!(columns.is_ok_and(|columns| {
        let item = serde_json::json!({
            (constants_str::SQL_NAMES_ID): pg_crud_common::explicit_value::ExplicitValue::new(1i64),
            (constants_str::USER_ID): pg_crud_common::explicit_value::ExplicitValue::new(Some(1i64)),
            (constants_str::USER_LOGIN): pg_crud_common::explicit_value::ExplicitValue::new(Some(constants_str::ADMIN)),
            (constants_str::ACTION): pg_crud_common::explicit_value::ExplicitValue::new(constants_str::READ),
            (constants_str::RESOURCE): pg_crud_common::explicit_value::ExplicitValue::new(constants_str::USER),
            (constants_str::RESOURCE_ID): pg_crud_common::explicit_value::ExplicitValue::new(Some(constants_str::VALUE_1)),
            (constants_str::TEST_JSON_REQUEST_ID): pg_crud_common::explicit_value::ExplicitValue::new(Some(constants_str::TEST_REFRESH_TOKEN_ID)),
            (constants_str::SUCCEEDED): pg_crud_common::explicit_value::ExplicitValue::new(true),
            (constants_str::CREATED_AT): pg_crud_common::explicit_value::ExplicitValue::new(constants_str::VALUE_BA5B49F1),
        });
        crate::generated_data_table_view::generated_data_table_view::<
            _,
            crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError,
        >(
            pg_crud_common::list_items::ListItems::from(vec![item]),
            pg_crud_common::list_total::ListTotal::from(1u32),
            admin_data_table,
            Some(columns),
        )
        .is_ok_and(|view| {
            view.items().len() == 1usize
                && view
                    .items()
                    .first()
                    .is_some_and(|row| row.values().len() == 9usize)
        })
    }));
}
