#[test]
fn test_generated_table_cells_read_explicit_values() {
    let timestamp = constants_str::VALUE_BA5B49F1;
    let item = serde_json::json!({
        (constants_str::SQL_NAMES_ID): pg_crud_common::explicit_value::ExplicitValue::new(1i64),
        (constants_str::ROLE_ID): pg_crud_common::explicit_value::ExplicitValue::new(2i64),
        (constants_str::RULE_ID): pg_crud_common::explicit_value::ExplicitValue::new(3i64),
        (constants_str::CREATED_AT): pg_crud_common::explicit_value::ExplicitValue::new(timestamp),
    });
    let view = crate::generated_data_table_view::generated_data_table_view::<
        _,
        crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError,
    >(
        pg_crud_common::list_items::ListItems::from(vec![item]),
        pg_crud_common::list_total::ListTotal::from(1u32),
        server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
        None,
    );
    assert!(view.is_ok_and(|view| {
        view.table() == server_admin_contract::admin_data_table::AdminDataTable::RoleRules
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

fn test_generated_table_id_columns() -> Result<
    server_admin_contract::admin_data_columns::AdminDataColumns,
    server_runtime_http::serde_json_error::SerdeJsonError,
> {
    serde_json::from_value::<server_admin_contract::admin_data_columns::AdminDataColumns>(serde_json::json!([
        {(stringify!(filters)): [], (stringify!(label)): constants_str::X, (stringify!(name)): constants_str::SQL_NAMES_ID, (stringify!(input_kind)): frontend_contract::input_kind::InputKind::Text}
    ])).map_err(server_runtime_http::serde_json_error::SerdeJsonError::from)
}

#[test]
fn test_generated_table_formats_every_json_cell_kind_without_quoting_text() {
    [
        (
            serde_json::Value::Null,
            constants_str::SERVER_ADMIN_DATA_NULL.to_owned(),
        ),
        (
            serde_json::json!(constants_str::X),
            constants_str::X.to_owned(),
        ),
        (
            serde_json::json!(constants_str::TEST_TEXT_WITH_NUL),
            constants_str::TEST_TEXT_WITH_NUL.to_owned(),
        ),
        (serde_json::json!(true), constants_str::TRUE.to_owned()),
        (serde_json::json!(false), constants_str::FALSE.to_owned()),
        (serde_json::json!(1i64), 1i64.to_string()),
        (serde_json::json!(-1i64), (-1i64).to_string()),
        (serde_json::json!(u64::MAX), u64::MAX.to_string()),
        (
            serde_json::json!([1i64, true]),
            format!("[{},{}]", 1i64, constants_str::TRUE),
        ),
        (
            serde_json::json!({(constants_str::X): 1i64}),
            format!("{{\"{}\":{}}}", constants_str::X, 1i64),
        ),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        let columns_result = test_generated_table_id_columns();
        assert!(columns_result.as_ref().err().is_none());
        if let Ok(columns) = columns_result {
            let item =
                serde_json::json!({(constants_str::SQL_NAMES_ID): {(stringify!(value)): value}});
            let result = crate::generated_data_table_view::generated_data_table_view::<
                _,
                crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError,
            >(
                pg_crud_common::list_items::ListItems::from(vec![item]),
                pg_crud_common::list_total::ListTotal::from(7u32),
                server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
                Some(columns),
            );
            assert!(result.is_ok_and(|view| {
                view.table() == server_admin_contract::admin_data_table::AdminDataTable::RoleRules
                    && u64::from(view.total()) == 7u64
                    && view.columns().len() == 1usize
                    && view.items().len() == 1usize
                    && view.items().first().is_some_and(|row| {
                        row.values().len() == 1usize
                            && row
                                .values()
                                .first()
                                .is_some_and(|text| text.as_ref().as_str() == expected)
                    })
            }));
        }
    });
}

#[test]
fn test_generated_table_rejects_non_object_rows_and_malformed_value_envelopes() {
    [
        serde_json::Value::Null,
        serde_json::json!([]),
        serde_json::json!(1i64),
        serde_json::json!(true),
        serde_json::json!(constants_str::X),
        serde_json::json!({(constants_str::SQL_NAMES_ID): null}),
        serde_json::json!({(constants_str::SQL_NAMES_ID): constants_str::X}),
        serde_json::json!({(constants_str::SQL_NAMES_ID): []}),
        serde_json::json!({(constants_str::SQL_NAMES_ID): {}}),
        serde_json::json!({(constants_str::SQL_NAMES_ID): {(constants_str::X): 1i64}}),
    ].into_iter().fold((), |(), item| {
        [false, true].into_iter().fold((), |(), explicit_columns| {
            let columns_result = if explicit_columns { test_generated_table_id_columns().map(Some) } else { Ok(None) };
            assert!(columns_result.as_ref().err().is_none());
            if let Ok(columns) = columns_result {
                let result = crate::generated_data_table_view::generated_data_table_view::<_, crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError>(
                    pg_crud_common::list_items::ListItems::from(vec![&item]),
                    pg_crud_common::list_total::ListTotal::from(1u32),
                    server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
                    columns,
                );
                assert!(matches!(result, Err(crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError::StoredValue)));
            }
        });
    });
    let columns_result = test_generated_table_id_columns();
    assert!(columns_result.as_ref().err().is_none());
    if let Ok(columns) = columns_result {
        assert!(matches!(
            crate::generated_data_table_view::generated_data_table_view::<
                _,
                crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError,
            >(
                pg_crud_common::list_items::ListItems::from(vec![serde_json::json!({})]),
                pg_crud_common::list_total::ListTotal::from(1u32),
                server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
                Some(columns),
            ),
            Err(crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError::StoredValue)
        ));
    }
}

#[test]
fn test_generated_table_enforces_rendered_row_count_limit() {
    [10_000usize, 10_001usize].into_iter().fold((), |(), count| {
        let items = std::iter::repeat_with(|| serde_json::json!({})).take(count).collect::<Vec<_>>();
        let result = crate::generated_data_table_view::generated_data_table_view::<_, crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError>(
            pg_crud_common::list_items::ListItems::from(items),
            pg_crud_common::list_total::ListTotal::from(7u32),
            server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
            None,
        );
        if count == 10_000usize {
            assert!(result.is_ok_and(|view| view.columns().is_empty() && view.items().len() == count && view.items().iter().all(|row| row.values().is_empty()) && u64::from(view.total()) == 7u64));
        } else {
            assert!(matches!(result, Err(crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError::Collection(server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual, max }))) if actual.get() == count && max.get() == 10_000usize));
        }
    });
}

#[test]
fn test_generated_table_enforces_character_limit_without_truncating_text() {
    [(constants_str::X.repeat(8192usize), true), (constants_str::X.repeat(8193usize), false), (char::MAX.to_string().repeat(8192usize), true)].into_iter().fold((), |(), (text, accepted)| {
        let columns_result = test_generated_table_id_columns();
        assert!(columns_result.as_ref().err().is_none());
        if let Ok(columns) = columns_result {
            let item = serde_json::json!({(constants_str::SQL_NAMES_ID): {(stringify!(value)): &text}});
            let result = crate::generated_data_table_view::generated_data_table_view::<_, crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError>(
                pg_crud_common::list_items::ListItems::from(vec![item]),
                pg_crud_common::list_total::ListTotal::from(1u32),
                server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
                Some(columns),
            );
            if accepted {
                assert!(result.is_ok_and(|view| view.items().first().is_some_and(|row| row.values().first().is_some_and(|admin_text| admin_text.as_ref().as_str() == text))));
            } else {
                assert!(matches!(result, Err(crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError::Text(server_admin_contract::admin_text::AdminTextTryFromStringError::TooLong { len: 8193usize, max: 8192usize }))));
            }
        }
    });
}

#[test]
fn test_generated_table_preserves_serialization_failure_in_both_column_modes() {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct AdminTableRowSerializationFailure;
    impl serde::Serialize for AdminTableRowSerializationFailure {
        fn serialize<Serializer>(
            &self,
            serializer: Serializer,
        ) -> Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::Serializer,
        {
            drop(serializer);
            Err(serde::ser::Error::custom(constants_str::X))
        }
    }
    [false, true].into_iter().fold((), |(), explicit_columns| {
        let columns_result = if explicit_columns { test_generated_table_id_columns().map(Some) } else { Ok(None) };
        assert!(columns_result.as_ref().err().is_none());
        if let Ok(columns) = columns_result {
            let result = crate::generated_data_table_view::generated_data_table_view::<_, crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError>(
                pg_crud_common::list_items::ListItems::from(vec![AdminTableRowSerializationFailure]),
                pg_crud_common::list_total::ListTotal::from(1u32),
                server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
                columns,
            );
            assert!(matches!(result.as_ref(), Err(crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError::Serialization(_))));
            assert!(result.as_ref().err().is_some_and(|error| std::error::Error::source(error).is_some_and(|source| source.to_string() == constants_str::X)));
        }
    });
}

#[test]
fn test_generated_table_derives_columns_from_first_row_in_catalog_order() {
    let first = serde_json::json!({(constants_str::CREATED_AT): {(stringify!(value)): constants_str::X}, (constants_str::SQL_NAMES_ID): {(stringify!(value)): 1i64}});
    let second = serde_json::json!({(constants_str::ROLE_ID): {(stringify!(value)): 3i64}, (constants_str::CREATED_AT): {(stringify!(value)): constants_str::LOGIN}, (constants_str::SQL_NAMES_ID): {(stringify!(value)): 2i64}});
    let result = crate::generated_data_table_view::generated_data_table_view::<
        _,
        crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError,
    >(
        pg_crud_common::list_items::ListItems::from(vec![first, second]),
        pg_crud_common::list_total::ListTotal::from(7u32),
        server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
        None,
    );
    assert!(result.is_ok_and(|view| {
        view.columns()
            .iter()
            .map(|column| column.name().as_ref().as_str())
            .eq([constants_str::SQL_NAMES_ID, constants_str::CREATED_AT])
            && u64::from(view.total()) == 7u64
            && serde_json::to_value(view.items()).is_ok_and(|wire| {
                wire == serde_json::json!([
                    {(stringify!(values)): [1i64.to_string(), constants_str::X.to_owned()]},
                    {(stringify!(values)): [2i64.to_string(), constants_str::LOGIN.to_owned()]},
                ])
            })
    }));
    let empty = crate::generated_data_table_view::generated_data_table_view::<
        serde_json::Value,
        crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError,
    >(
        pg_crud_common::list_items::ListItems::from(Vec::new()),
        pg_crud_common::list_total::ListTotal::from(7u32),
        server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
        None,
    );
    assert!(empty.is_ok_and(|view| {
        view.items().is_empty()
            && u64::from(view.total()) == 7u64
            && view
                .columns()
                .iter()
                .map(|column| column.name().as_ref().as_str())
                .eq([
                    constants_str::SQL_NAMES_ID,
                    constants_str::ROLE_ID,
                    constants_str::RULE_ID,
                    constants_str::CREATED_AT,
                ])
    }));
}
