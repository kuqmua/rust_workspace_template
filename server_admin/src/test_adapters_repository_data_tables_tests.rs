fn filter_query(
    str: &str,
    filter_operation: frontend_contract::filter_operation::FilterOperation,
    filter_value: Option<&str>,
    end: Option<&str>,
) -> server_admin_contract::admin_data_table_query::AdminDataTableQuery {
    server_admin_contract::admin_data_table_query::AdminDataTableQuery::new(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::admin_filter_field::AdminFilterField::try_from(
                    str.to_owned(),
                )
                .expect(constants_str::DIAGNOSTIC_A17498DC),
            ),
            Some(filter_operation),
            filter_value.map(|raw_filter_value| {
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                    raw_filter_value.to_owned(),
                )
                .expect(constants_str::DIAGNOSTIC_F064FCD7)
            }),
            end.map(|raw_filter_end| {
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                    raw_filter_end.to_owned(),
                )
                .expect(constants_str::DIAGNOSTIC_9B563F27)
            }),
        ),
        server_admin_contract::admin_table_query::AdminTableQuery::default(),
    )
}

fn filter_test_value(
    field_name: &str,
    input_kind: frontend_contract::input_kind::InputKind,
) -> &'static str {
    if field_name == stringify!(ip_address) {
        constants_str::VALUE_127_0_0_1
    } else {
        match input_kind {
            frontend_contract::input_kind::InputKind::Checkbox => stringify!(true),
            frontend_contract::input_kind::InputKind::Date
            | frontend_contract::input_kind::InputKind::DateTime
            | frontend_contract::input_kind::InputKind::Time => {
                constants_str::VALUE_2026_07_13T12_30_00
            }
            frontend_contract::input_kind::InputKind::Number => stringify!(2),
            frontend_contract::input_kind::InputKind::Text => constants_str::ADMIN_ALT,
            frontend_contract::input_kind::InputKind::Uuid => {
                constants_str::VALUE_550E8400_E29B_41D4_A716_446655440000
            }
        }
    }
}

fn filter_test_end_value(input_kind: frontend_contract::input_kind::InputKind) -> &'static str {
    match input_kind {
        frontend_contract::input_kind::InputKind::Number => stringify!(4),
        frontend_contract::input_kind::InputKind::Date
        | frontend_contract::input_kind::InputKind::DateTime
        | frontend_contract::input_kind::InputKind::Time => {
            constants_str::VALUE_2026_07_13T12_30_30
        }
        frontend_contract::input_kind::InputKind::Checkbox
        | frontend_contract::input_kind::InputKind::Text
        | frontend_contract::input_kind::InputKind::Uuid => constants_str::ADMIN,
    }
}

#[test]
fn test_every_generated_read_filter_accepts_every_logical_operator_variation() {
    let filter_test_json_value =
        |admin_generated_table: crate::admin_generated_table::AdminGeneratedTable,
         field_name: &str,
         input_kind: frontend_contract::input_kind::InputKind,
         operation: frontend_contract::filter_operation::FilterOperation|
         -> Result<serde_json::Value, String> {
            let parse = |raw_value: &str| {
                let wire_value = admin_generated_table
                    .filter_value(
                        frontend_contract::form_field_name_ref::FormFieldNameRef::from(field_name),
                        frontend_contract::form_value_ref::FormValueRef::from(raw_value),
                    )
                    .ok_or_else(String::new)?
                    .map_err(|error| error.to_string())?;
                serde_json::from_str::<serde_json::Value>(wire_value.as_ref())
                    .map_err(|error| error.to_string())
            };
            let value = parse(filter_test_value(field_name, input_kind))?;
            match operation.value_shape() {
                frontend_contract::filter_value_shape::FilterValueShape::None => {
                    Ok(serde_json::Value::Null)
                }
                frontend_contract::filter_value_shape::FilterValueShape::Range => {
                    Ok(serde_json::json!({
                        (constants_str::PG_CRUD_START_FIELD): value,
                        (constants_str::PG_CRUD_END_FIELD): parse(filter_test_end_value(input_kind))?
                    }))
                }
                frontend_contract::filter_value_shape::FilterValueShape::List => {
                    Ok(serde_json::Value::Array(vec![
                        value,
                        parse(filter_test_end_value(input_kind))?,
                    ]))
                }
                frontend_contract::filter_value_shape::FilterValueShape::EncodedText => {
                    Ok(serde_json::json!({
                        (constants_str::SERVER_ADMIN_FILTER_ENCODE_FORMAT_FIELD): constants_str::SERVER_ADMIN_FILTER_ENCODE_BASE64,
                        (constants_str::SERVER_ADMIN_FILTER_ENCODED_VALUE_FIELD): constants_str::ADMIN_ALT,
                    }))
                }
                frontend_contract::filter_value_shape::FilterValueShape::Regex
                | frontend_contract::filter_value_shape::FilterValueShape::Scalar => Ok(value),
            }
        };
    [
        crate::admin_generated_table::AdminGeneratedTable::Roles,
        crate::admin_generated_table::AdminGeneratedTable::RolePermissions,
        crate::admin_generated_table::AdminGeneratedTable::UsersDatabaseRead,
        crate::admin_generated_table::AdminGeneratedTable::Permissions,
        crate::admin_generated_table::AdminGeneratedTable::SystemSettings,
        crate::admin_generated_table::AdminGeneratedTable::UserRoles,
    ]
    .into_iter()
    .for_each(|admin_generated_table| {
        admin_generated_table
            .field_contracts()
            .as_ref()
            .iter()
            .filter(|field_contract| {
                field_contract.readable()
                    == frontend_contract::field_capability::FieldCapability::Enabled
            })
            .for_each(|field_contract| {
                field_contract.filters().iter().copied().for_each(|operation| {
                    [
                        stringify!(And),
                        stringify!(AndNot),
                        stringify!(Or),
                        stringify!(OrNot),
                    ]
                    .into_iter()
                    .for_each(|field_operator| {
                        [
                            stringify!(And),
                            stringify!(AndNot),
                            stringify!(Or),
                            stringify!(OrNot),
                        ]
                        .into_iter()
                        .for_each(|predicate_operator| {
                            let mut predicate = serde_json::Map::new();
                            let _operator_replaced = predicate.insert(
                                constants_str::PG_CRUD_OPERATOR_FIELD.to_owned(),
                                serde_json::Value::String(predicate_operator.to_owned()),
                            );
                            if operation.value_shape()
                                != frontend_contract::filter_value_shape::FilterValueShape::None
                            {
                                let value = filter_test_json_value(
                                    admin_generated_table,
                                    field_contract.name().as_ref(),
                                    field_contract.type_contract().input_kind(),
                                    operation,
                                );
                                assert!(value.is_ok());
                                let Ok(value) = value else {
                                    return;
                                };
                                let _values_replaced = predicate.insert(
                                    constants_str::PG_CRUD_VALUES_FIELD.to_owned(),
                                    value,
                                );
                            }
                            if operation.value_shape()
                                == frontend_contract::filter_value_shape::FilterValueShape::Regex
                            {
                                let _regex_case_replaced = predicate.insert(
                                    constants_str::SERVER_ADMIN_FILTER_REGEX_CASE_FIELD.to_owned(),
                                    serde_json::Value::String(
                                        constants_str::SERVER_ADMIN_FILTER_REGEX_SENSITIVE
                                            .to_owned(),
                                    ),
                                );
                            }
                            let mut operation_entry = serde_json::Map::new();
                            let _operation_replaced = operation_entry.insert(
                                format!("{operation:?}"),
                                serde_json::Value::Object(predicate),
                            );
                            let filter = serde_json::json!({
                                (field_contract.name().as_ref()): {
                                    (constants_str::PG_CRUD_OPERATOR_FIELD): field_operator,
                                    (constants_str::PG_CRUD_VALUES_FIELD): [operation_entry]
                                }
                            });
                            let result = serde_json::to_string(&filter)
                                .map_err(|error| error.to_string())
                                .and_then(|filter_json| {
                                    admin_generated_table
                                        .parse_filter(
                                            server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                                                filter_json.as_str(),
                                            ),
                                        )
                                        .map_err(|error| error.to_string())
                                })
                                .and_then(|filter| {
                                    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
                                    filter
                                        .query_part(&mut increment)
                                        .map(|_fragment| increment.get())
                                        .map_err(|error| error.to_string())
                                });
                            let expected_increment = match operation.value_shape() {
                                frontend_contract::filter_value_shape::FilterValueShape::None => constants_u64::ZERO,
                                frontend_contract::filter_value_shape::FilterValueShape::Range
                                | frontend_contract::filter_value_shape::FilterValueShape::List => 2u64,
                                frontend_contract::filter_value_shape::FilterValueShape::EncodedText
                                | frontend_contract::filter_value_shape::FilterValueShape::Regex
                                | frontend_contract::filter_value_shape::FilterValueShape::Scalar => 1u64,
                            };
                            assert_eq!(
                                result,
                                Ok(expected_increment),
                                "{admin_generated_table:?}.{}.{operation:?}.{field_operator}.{predicate_operator}",
                                field_contract.name().as_ref(),
                            );
                        });
                    });
                });
            });
    });
}

#[test]
fn test_every_read_table_filter_column_and_operation_builds_a_typed_predicate() {
    let table_field_contracts = |admin_data_table| match admin_data_table {
        server_admin_contract::admin_data_table::AdminDataTable::AccessSessions => {
            crate::admin_access_sessions::AdminAccessSessions::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::AuditLog => {
            crate::admin_audit_log::AdminAuditLog::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus => {
            crate::admin_cleanup_status::AdminCleanupStatus::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts => {
            crate::admin_login_attempts::AdminLoginAttempts::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::Permissions => {
            crate::admin_permissions::AdminPermissions::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::RateLimits => {
            crate::admin_rate_limits::AdminRateLimits::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens => {
            crate::admin_refresh_tokens::AdminRefreshTokens::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::RolePermissions => {
            crate::admin_role_permissions::AdminRolePermissions::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::Roles => {
            crate::admin_roles::AdminRoles::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::SystemSettings => {
            crate::admin_system_settings::AdminSystemSettings::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::UserRoles => {
            crate::admin_user_roles::AdminUserRoles::frontend_fields()
        }
        server_admin_contract::admin_data_table::AdminDataTable::Users => {
            crate::admin_users_database_read::AdminUsersDatabaseRead::frontend_fields()
        }
    };
    let table_filter_value = |admin_data_table,
                              field_name: &str,
                              raw_value: &str|
     -> Result<serde_json::Value, String> {
        let field_name_ref =
            frontend_contract::form_field_name_ref::FormFieldNameRef::from(field_name);
        let form_value_ref = frontend_contract::form_value_ref::FormValueRef::from(raw_value);
        let wire_value = match admin_data_table {
            server_admin_contract::admin_data_table::AdminDataTable::AccessSessions => {
                crate::admin_access_sessions::AdminAccessSessions::frontend_filter_value(
                    field_name_ref,
                    form_value_ref,
                )
            }
            server_admin_contract::admin_data_table::AdminDataTable::AuditLog => {
                crate::admin_audit_log::AdminAuditLog::frontend_filter_value(
                    field_name_ref,
                    form_value_ref,
                )
            }
            server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus => {
                crate::admin_cleanup_status::AdminCleanupStatus::frontend_filter_value(
                    field_name_ref,
                    form_value_ref,
                )
            }
            server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts => {
                crate::admin_login_attempts::AdminLoginAttempts::frontend_filter_value(
                    field_name_ref,
                    form_value_ref,
                )
            }
            server_admin_contract::admin_data_table::AdminDataTable::RateLimits => {
                crate::admin_rate_limits::AdminRateLimits::frontend_filter_value(
                    field_name_ref,
                    form_value_ref,
                )
            }
            server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens => {
                crate::admin_refresh_tokens::AdminRefreshTokens::frontend_filter_value(
                    field_name_ref,
                    form_value_ref,
                )
            }
            server_admin_contract::admin_data_table::AdminDataTable::Permissions
            | server_admin_contract::admin_data_table::AdminDataTable::RolePermissions
            | server_admin_contract::admin_data_table::AdminDataTable::Roles
            | server_admin_contract::admin_data_table::AdminDataTable::SystemSettings
            | server_admin_contract::admin_data_table::AdminDataTable::UserRoles
            | server_admin_contract::admin_data_table::AdminDataTable::Users => {
                crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table)
                    .and_then(|admin_generated_table| {
                        admin_generated_table.filter_value(field_name_ref, form_value_ref)
                    })
            }
        }
        .ok_or_else(String::new)?
        .map_err(|error| error.to_string())?;
        serde_json::from_str::<serde_json::Value>(wire_value.as_ref())
            .map_err(|error| error.to_string())
    };
    let assert_time_value = |wire_value: &serde_json::Value| {
        assert!(wire_value.as_object().is_some_and(|time| {
            time.get(stringify!(hour))
                .and_then(serde_json::Value::as_u64)
                == Some(12u64)
                && time
                    .get(constants_str::MIN)
                    .and_then(serde_json::Value::as_u64)
                    == Some(30u64)
                && time
                    .get(constants_str::SEC)
                    .and_then(serde_json::Value::as_u64)
                    == Some(constants_u64::ZERO)
        }));
    };
    let assert_wire_type_and_value =
        |value_format: frontend_contract::value_format::ValueFormat,
         raw_value: &str,
         wire_value: &serde_json::Value| {
            match value_format {
                frontend_contract::value_format::ValueFormat::Bool => {
                    assert_eq!(wire_value.as_bool(), raw_value.parse::<bool>().ok());
                }
                frontend_contract::value_format::ValueFormat::Int16
                | frontend_contract::value_format::ValueFormat::Int32
                | frontend_contract::value_format::ValueFormat::Int64 => {
                    assert_eq!(wire_value.as_i64(), raw_value.parse::<i64>().ok());
                }
                frontend_contract::value_format::ValueFormat::Inet => {
                    assert_eq!(wire_value.as_str(), Some(constants_str::VALUE_127_0_0_1_32));
                }
                frontend_contract::value_format::ValueFormat::Date
                | frontend_contract::value_format::ValueFormat::Mac
                | frontend_contract::value_format::ValueFormat::Text
                | frontend_contract::value_format::ValueFormat::Uuid => {
                    assert_eq!(wire_value.as_str(), Some(raw_value));
                }
                frontend_contract::value_format::ValueFormat::DateTime
                | frontend_contract::value_format::ValueFormat::Timestamp
                | frontend_contract::value_format::ValueFormat::TimestampTz => {
                    let object = wire_value.as_object();
                    assert!(object.is_some());
                    let Some(object) = object else {
                        return;
                    };
                    let expected_date = raw_value.split_once('T').map(|(date, _time)| date);
                    assert!(object
                        .values()
                        .any(|value| value.as_str() == expected_date));
                    let time = object
                        .get(stringify!(time))
                        .unwrap_or(&serde_json::Value::Null);
                    assert_time_value(time);
                }
                frontend_contract::value_format::ValueFormat::Float32
                | frontend_contract::value_format::ValueFormat::Float64 => {
                    assert_eq!(wire_value.as_f64(), raw_value.parse::<f64>().ok());
                }
                frontend_contract::value_format::ValueFormat::Time => {
                    assert_time_value(wire_value);
                }
                unsupported_value_format
                    @ (frontend_contract::value_format::ValueFormat::Bytes
                    | frontend_contract::value_format::ValueFormat::Interval
                    | frontend_contract::value_format::ValueFormat::Range) => {
                    assert_eq!(
                        unsupported_value_format,
                        frontend_contract::value_format::ValueFormat::Text,
                        "a94f6b2c unsupported filter value format reached the test matrix"
                    );
                }
            }
        };
    server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
        .into_iter()
        .for_each(|admin_data_table| {
            let field_contracts = table_field_contracts(admin_data_table);
            admin_data_table
                .spec()
                .columns()
                .get()
                .split(',')
                .for_each(|column| {
                    let field_contract = field_contracts
                        .as_ref()
                        .iter()
                        .find(|field_contract| field_contract.name().as_ref() == column);
                    assert!(field_contract.is_some());
                    let Some(field_contract) = field_contract else {
                        return;
                    };
                    if field_contract.readable()
                        == frontend_contract::field_capability::FieldCapability::Disabled
                    {
                        return;
                    }
                    assert!(!field_contract.filters().is_empty());
                    field_contract
                        .filters()
                        .iter()
                        .copied()
                        .for_each(|operation| {
                            let value = filter_test_value(
                                column,
                                field_contract.type_contract().input_kind(),
                            );
                            let wire_value = table_filter_value(
                                admin_data_table,
                                field_contract.name().as_ref(),
                                value,
                            );
                            assert!(wire_value.is_ok());
                            if let Ok(wire_value) = wire_value {
                                assert_wire_type_and_value(
                                    field_contract.type_contract().format(),
                                    value,
                                    &wire_value,
                                );
                            }
                            let (value, end) = match operation.value_shape() {
                            frontend_contract::filter_value_shape::FilterValueShape::None => {
                                (None, None)
                            }
                            frontend_contract::filter_value_shape::FilterValueShape::Range => {
                                (
                                    Some(value),
                                    Some(filter_test_end_value(
                                        field_contract.type_contract().input_kind(),
                                    )),
                                )
                            }
                            frontend_contract::filter_value_shape::FilterValueShape::List
                            | frontend_contract::filter_value_shape::FilterValueShape::EncodedText
                            | frontend_contract::filter_value_shape::FilterValueShape::Regex
                            | frontend_contract::filter_value_shape::FilterValueShape::Scalar => {
                                (Some(value), None)
                            }
                        };
                            let query = filter_query(column, operation, value, end);
                            let expected_increment = match operation.value_shape() {
                                frontend_contract::filter_value_shape::FilterValueShape::None => {
                                    constants_u64::ZERO
                                }
                                frontend_contract::filter_value_shape::FilterValueShape::Range => {
                                    2u64
                                }
                                frontend_contract::filter_value_shape::FilterValueShape::EncodedText
                                | frontend_contract::filter_value_shape::FilterValueShape::List
                                | frontend_contract::filter_value_shape::FilterValueShape::Regex
                                | frontend_contract::filter_value_shape::FilterValueShape::Scalar => {
                                    1u64
                                }
                            };
                            let result = (|| {
                                let filter = crate::data_filter::data_filter(
                                    admin_data_table,
                                    query.filter(),
                                )
                                .map_err(|error| error.to_string())?
                                .ok_or_else(String::new)?;
                                let mut increment =
                                    pg_crud_common::query_part_increment::QueryPartIncrement::from(
                                        constants_u64::ZERO,
                                    );
                                let fragment = filter
                                    .query_part(&mut increment)
                                    .map_err(|error| error.to_string())?;
                                if fragment.as_ref().contains(column)
                                    && increment.get() == expected_increment
                                {
                                    Ok(())
                                } else {
                                    Err(fragment.as_ref().to_owned())
                                }
                            })();
                            assert_eq!(
                                result,
                                Ok(()),
                                "{admin_data_table:?}.{column}.{operation:?}"
                            );
                        });
                });
        });
}

#[test]
fn test_generated_table_fields_supply_client_column_metadata() {
    let columns = (|| -> Result<
        server_admin_contract::admin_data_columns::AdminDataColumns,
        crate::admin_repository_error::AdminRepositoryError,
    > {
        let table = server_admin_contract::admin_data_table::AdminDataTable::Users;
        let column_names = table.spec().columns();
        let generated_fields =
            crate::admin_generated_table::AdminGeneratedTable::for_data_table(table)
                .map(crate::admin_generated_table::AdminGeneratedTable::field_contracts);
        let columns = column_names
            .get()
            .split(',')
            .map(|raw_name| {
                let generated_field = generated_fields.as_ref().and_then(|fields| {
                    AsRef::<[frontend_contract::field_contract::FieldContract]>::as_ref(fields)
                        .iter()
                        .find(|field| field.name().as_ref() == raw_name)
                });
                let label_text = generated_field.map_or_else(
                    || raw_name.to_owned(),
                    |field| field.label().as_ref().to_owned(),
                );
                let input_kind = generated_field.map_or(frontend_contract::input_kind::InputKind::Text, |field| {
                    field.type_contract().input_kind()
                });
                let raw_filters = generated_field.map_or_else(Vec::new, |field| {
                    field
                        .filters()
                        .iter()
                        .copied()
                        .map(server_admin_contract::admin_data_filter::AdminDataFilter::from)
                        .collect::<Vec<_>>()
                });
                let filters =
                    server_admin_contract::admin_data_filters::AdminDataFilters::try_from(raw_filters)
                        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                let label = server_admin_contract::admin_text::AdminText::try_from(label_text)
                    .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                let name =
                    server_admin_contract::admin_text::AdminText::try_from(raw_name.to_owned())
                        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                Ok(server_admin_contract::admin_data_column::AdminDataColumn::new(
                    filters, input_kind, label, name,
                ))
            })
            .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
        server_admin_contract::admin_data_columns::AdminDataColumns::try_from(columns)
            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
    })()
    .expect(constants_str::DIAGNOSTIC_F3C897AF);
    let id = columns
        .as_slice()
        .iter()
        .find(|column| column.name().as_ref() == constants_str::SQL_NAMES_ID);
    assert!(id.is_some_and(|column| {
        column.input_kind() == frontend_contract::input_kind::InputKind::Number
    }));
    let login = columns
        .as_slice()
        .iter()
        .find(|column| column.name().as_ref() == constants_str::LOGIN)
        .expect(constants_str::DIAGNOSTIC_7A340D1F);
    assert_eq!(
        login
            .filters()
            .iter()
            .map(server_admin_contract::admin_data_filter::AdminDataFilter::operation)
            .collect::<Vec<_>>(),
        [
            frontend_contract::filter_operation::FilterOperation::Eq,
            frontend_contract::filter_operation::FilterOperation::In,
            frontend_contract::filter_operation::FilterOperation::Regex,
        ]
    );
}

#[test]
fn test_generated_where_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::VALUE_2BD806C9),
        None,
    );
    let filter = crate::data_filter::data_filter(
        server_admin_contract::admin_data_table::AdminDataTable::Users,
        query.filter(),
    )
    .expect(constants_str::DIAGNOSTIC_F4EEDE2B)
    .expect(constants_str::DIAGNOSTIC_ED970B26);
    let mut increment =
        pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);

    let fragment = filter
        .query_part(&mut increment)
        .expect(constants_str::DIAGNOSTIC_FE02D3C8);

    assert!(fragment.as_ref().contains(constants_str::LOGIN));
    assert!(fragment.as_ref().contains(constants_str::DOLLAR_1_ALT));
    assert_eq!(increment.get(), 1u64);
}

#[test]
fn test_session_tables_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::USER_ID,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::VALUE_42),
        None,
    );
    [
        server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
        server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens,
    ]
    .into_iter()
    .for_each(|table| {
        let result = (|| {
            let filter = crate::data_filter::data_filter(table, query.filter())
                .map_err(|error| error.to_string())?
                .ok_or_else(String::new)?;
            let mut increment =
                pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
            let fragment = filter
                .query_part(&mut increment)
                .map_err(|error| error.to_string())?;
            if fragment.as_ref().contains(constants_str::USER_ID)
                && fragment.as_ref().contains(constants_str::DOLLAR_1_ALT)
            {
                Ok(increment.get())
            } else {
                Err(String::new())
            }
        })();
        assert_eq!(result, Ok(1u64));
    });
}

#[test]
fn test_active_sessions_filter_appends_typed_predicate_after_owner_parameter() {
    let query = filter_query(
        constants_str::SQL_NAMES_ID,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::VALUE_550E8400_E29B_41D4_A716_446655440000),
        None,
    );
    let filter = crate::data_filter::data_filter(
        server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
        query.filter(),
    )
    .expect(constants_str::DIAGNOSTIC_4E779DF0)
    .expect(constants_str::DIAGNOSTIC_D9C8CF39);
    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(1u64);

    let fragment = filter
        .query_part(&mut increment)
        .expect(constants_str::DIAGNOSTIC_A25FE142);

    assert!(
        fragment
            .as_ref()
            .starts_with(constants_str::WHERE_ALT.trim_end())
    );
    assert!(fragment.as_ref().contains(constants_str::SQL_NAMES_ID));
    assert!(fragment.as_ref().contains(constants_str::DOLLAR_2));
    assert_eq!(increment.get(), 2u64);
}

#[test]
fn test_session_tables_columns_supply_filter_metadata() {
    [
        (
            server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
            crate::admin_access_sessions::AdminAccessSessions::frontend_fields(),
        ),
        (
            server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens,
            crate::admin_refresh_tokens::AdminRefreshTokens::frontend_fields(),
        ),
    ]
    .into_iter()
    .for_each(|(table, field_contracts)| {
        assert!(table.spec().columns().get().split(',').all(|column| {
            field_contracts
                .as_ref()
                .iter()
                .find(|field| field.name().as_ref() == column)
                .is_some_and(|field| !field.filters().is_empty())
        }));
    });
}

#[test]
fn test_login_attempts_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::ADMIN),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::LOGIN)
            && fragment.as_ref().contains(constants_str::DOLLAR_1_ALT)
        {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();

    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_login_attempts_columns_supply_filter_metadata() {
    let field_contracts = crate::admin_login_attempts::AdminLoginAttempts::frontend_fields();

    assert!(
        server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts
            .spec()
            .columns()
            .get()
            .split(',')
            .all(|column| {
                field_contracts
                    .as_ref()
                    .iter()
                    .find(|field| field.name().as_ref() == column)
                    .is_some_and(|field| !field.filters().is_empty())
            })
    );
}

#[test]
fn test_cleanup_status_filter_builds_typed_table_predicate() {
    let identifier = constants_str::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS
        .split(',')
        .next()
        .unwrap_or(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
    let query = filter_query(
        identifier,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::VALUE_42),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(identifier)
            && fragment.as_ref().contains(constants_str::DOLLAR_1_ALT)
        {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();

    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_cleanup_status_columns_supply_filter_metadata() {
    let field_contracts = crate::admin_cleanup_status::AdminCleanupStatus::frontend_fields();

    assert!(
        server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus
            .spec()
            .columns()
            .get()
            .split(',')
            .all(|column| {
                field_contracts
                    .as_ref()
                    .iter()
                    .find(|field| field.name().as_ref() == column)
                    .is_some_and(|field| !field.filters().is_empty())
            })
    );
}

#[test]
fn test_system_settings_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::VALUE_7C6A6719,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::ADMIN),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::SystemSettings,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::VALUE_7C6A6719)
            && fragment.as_ref().contains(constants_str::DOLLAR_1_ALT)
        {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();

    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_system_settings_columns_supply_filter_metadata() {
    let field_contracts = crate::admin_system_settings::AdminSystemSettings::frontend_fields();

    assert!(
        server_admin_contract::admin_data_table::AdminDataTable::SystemSettings
            .spec()
            .columns()
            .get()
            .split(',')
            .all(|column| {
                field_contracts
                    .as_ref()
                    .iter()
                    .find(|field| field.name().as_ref() == column)
                    .is_some_and(|field| !field.filters().is_empty())
            })
    );
}

#[test]
fn test_rate_limits_filter_builds_typed_table_predicate() {
    let scope = constants_str::SCOPE;
    let query = filter_query(
        scope,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::ADMIN),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::RateLimits,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(scope)
            && fragment.as_ref().contains(constants_str::DOLLAR_1_ALT)
        {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();

    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_rate_limits_columns_supply_filter_metadata() {
    let field_contracts = crate::admin_rate_limits::AdminRateLimits::frontend_fields();

    assert!(
        server_admin_contract::admin_data_table::AdminDataTable::RateLimits
            .spec()
            .columns()
            .get()
            .split(',')
            .all(|column| {
                field_contracts
                    .as_ref()
                    .iter()
                    .find(|field| field.name().as_ref() == column)
                    .is_some_and(|field| !field.filters().is_empty())
            })
    );
}

#[test]
fn test_audit_log_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::ACTION,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::UPDATE),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::AuditLog,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::ACTION)
            && fragment.as_ref().contains(constants_str::DOLLAR_1_ALT)
        {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();

    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_audit_log_columns_supply_filter_metadata() {
    let field_contracts = crate::admin_audit_log::AdminAuditLog::frontend_fields();

    assert_eq!(field_contracts.as_ref().len(), 10usize);
    assert!(
        field_contracts
            .as_ref()
            .iter()
            .filter(|field| {
                field.readable() == frontend_contract::field_capability::FieldCapability::Enabled
            })
            .all(|field| {
                !field.filters().is_empty()
                    && server_admin_contract::admin_data_table::AdminDataTable::AuditLog
                        .spec()
                        .columns()
                        .get()
                        .split(',')
                        .any(|column| field.name().as_ref() == column)
            })
    );
    assert_eq!(
        field_contracts
            .as_ref()
            .iter()
            .filter(|field| {
                field.readable() == frontend_contract::field_capability::FieldCapability::Disabled
            })
            .count(),
        1usize
    );
}

#[test]
fn test_generated_text_membership_filter_builds_typed_table_predicate() {
    let values = [constants_str::ADMIN, constants_str::ADMIN_ALT].join(constants_str::TEXT_ALT_7);
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::filter_operation::FilterOperation::In,
        Some(values.as_str()),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::LOGIN) {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();
    assert_eq!(result, Ok(2u64));
}

#[test]
fn test_generated_boolean_filter_reuses_shared_where_many_builder() {
    let query = filter_query(
        constants_str::IS_BANNED,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::TRUE),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::IS_BANNED) {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();
    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_generated_role_system_equality_filter_reuses_shared_where_many_builder() {
    let query = filter_query(
        constants_str::IS_SYSTEM,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(stringify!(true)),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Roles,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::IS_SYSTEM) {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();
    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_generated_role_name_membership_filter_builds_typed_table_predicate() {
    let values = [constants_str::ADMIN, constants_str::ADMIN_ALT].join(constants_str::TEXT_ALT_7);
    let query = filter_query(
        constants_str::NAME,
        frontend_contract::filter_operation::FilterOperation::In,
        Some(values.as_str()),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Roles,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::NAME) {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();
    assert_eq!(result, Ok(2u64));
}

#[test]
fn test_generated_user_display_name_equality_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::DISPLAY_NAME,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::ADMIN),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::DISPLAY_NAME) {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();
    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_generated_user_display_name_membership_filter_builds_typed_table_predicate() {
    let values = [constants_str::ADMIN, constants_str::ADMIN_ALT].join(constants_str::TEXT_ALT_7);
    let query = filter_query(
        constants_str::DISPLAY_NAME,
        frontend_contract::filter_operation::FilterOperation::In,
        Some(values.as_str()),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::DISPLAY_NAME) {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();
    assert_eq!(result, Ok(2u64));
}

#[test]
fn test_generated_role_name_equality_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::NAME,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::ADMIN),
        None,
    );
    let result = (|| {
        let filter = crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Roles,
            query.filter(),
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(String::new)?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .query_part(&mut increment)
            .map_err(|error| error.to_string())?;
        if fragment.as_ref().contains(constants_str::NAME) {
            Ok(increment.get())
        } else {
            Err(String::new())
        }
    })();
    assert_eq!(result, Ok(1u64));
}

#[test]
fn test_unsupported_field_operation_is_rejected() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::filter_operation::FilterOperation::Between,
        Some(constants_str::VALUE_2BD806C9),
        Some(constants_str::VALUE_81B637D8),
    );

    assert!(
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .is_err()
    );
}

#[test]
fn test_empty_filter_query_omits_the_predicate() {
    let query =
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::default();

    assert!(
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            &query
        )
        .expect(constants_str::DIAGNOSTIC_FD36A6F5)
        .is_none()
    );
}

#[test]
fn test_incomplete_filter_queries_are_rejected() {
    let field = server_admin_contract::admin_filter_field::AdminFilterField::try_from(
        constants_str::LOGIN.to_owned(),
    )
    .expect(constants_str::DIAGNOSTIC_F1832A34);
    let value = server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
        String::from(constants_str::VALUE_2BD806C9),
    )
    .expect(constants_str::DIAGNOSTIC_16849A06);
    let queries = [
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(field),
            None,
            None,
            None,
        ),
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            None,
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            None,
            None,
        ),
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            None,
            None,
            Some(value.clone()),
            None,
        ),
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            None,
            None,
            None,
            Some(value),
        ),
    ];

    assert!(queries.iter().all(|query| {
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query,
        )
        .is_err()
    }));
}

#[test]
fn test_unknown_filter_field_is_rejected() {
    let query = filter_query(
        constants_str::UNKNOWN_ALT,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::VALUE_2BD806C9),
        None,
    );

    assert!(
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter()
        )
        .is_err()
    );
}

#[test]
fn test_scalar_and_regex_filters_reject_range_end_values() {
    let operations = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::Regex,
    ];
    assert!(operations.into_iter().all(|operation| {
        let query = filter_query(
            constants_str::LOGIN,
            operation,
            Some(constants_str::VALUE_2BD806C9),
            Some(constants_str::VALUE_81B637D8),
        );
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .is_err()
    }));
}

#[test]
fn test_regex_filter_builds_a_typed_predicate() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::filter_operation::FilterOperation::Regex,
        Some(constants_str::VALUE_78C40633),
        None,
    );
    let filter = crate::data_filter::data_filter(
        server_admin_contract::admin_data_table::AdminDataTable::Users,
        query.filter(),
    )
    .expect(constants_str::DIAGNOSTIC_E0B1326D)
    .expect(constants_str::DIAGNOSTIC_8A4E68FB);
    let mut increment =
        pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);

    let fragment = filter
        .query_part(&mut increment)
        .expect(constants_str::DIAGNOSTIC_9F5E101D);

    assert!(fragment.as_ref().contains(constants_str::LOGIN));
    assert_eq!(increment.get(), 1u64);
}

#[test]
fn test_filtered_sql_places_pagination_after_filter_binds() {
    let fragment = pg_crud_common::query_part_fragment::QueryPartFragment::try_from(String::from(
        constants_str::VALUE_F7A09FE1,
    ))
    .expect(constants_str::DIAGNOSTIC_45D292B8);

    let (base_count, base_data) =
        crate::base_sql::base_sql(server_admin_contract::admin_data_table::AdminDataTable::Users)
            .expect(constants_str::DIAGNOSTIC_44C43299);
    let (_count, data) = (|| -> Result<
        (
            server_admin_core::std_admin_string::StdAdminString,
            server_admin_core::std_admin_string::StdAdminString,
        ),
        crate::admin_repository_error::AdminRepositoryError,
    > {
        let count_sql = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(base_count.as_ref().as_str());
        let data_sql = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(base_data.as_ref().as_str());
        let bind_count = pg_crud_common::query_part_increment::QueryPartIncrement::from(1u64);
        let mut filtered_count = count_sql.get().to_owned();
        filtered_count.push(' ');
        filtered_count.push_str(fragment.as_ref());
        let (data_prefix, ordered_suffix) = data_sql
            .get()
            .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
            .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let order = ordered_suffix
            .strip_suffix(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
            .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let limit_index = bind_count.get().saturating_add(1u64);
        let offset_index = limit_index.saturating_add(1u64);
        let mut filtered_data = data_prefix.to_owned();
        filtered_data.push(' ');
        filtered_data.push_str(fragment.as_ref());
        filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
        filtered_data.push_str(order);
        filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_PREFIX);
        filtered_data.push_str(limit_index.to_string().as_str());
        filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_OFFSET_PREFIX);
        filtered_data.push_str(offset_index.to_string().as_str());
        let count = server_admin_core::std_admin_string::StdAdminString::try_from(filtered_count)
            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let data = server_admin_core::std_admin_string::StdAdminString::try_from(filtered_data)
            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        Ok((count, data))
    })()
    .expect(constants_str::DIAGNOSTIC_C33365BA);

    assert!(data.as_ref().contains(constants_str::VALUE_F7A09FE1));
    assert!(data.as_ref().contains(constants_str::VALUE_51920234));
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "test adapters repository data tables tests uses iterator traversal to comply with the workspace no-for-loop policy"
)]
fn test_table_spec_generates_bounded_projection_and_count_sql_for_every_table() {
    server_admin_contract::admin_data_table::AdminDataTable::ALL
        .into_iter()
        .for_each(|table| {
            let (count, data) =
                crate::base_sql::base_sql(table).expect(constants_str::DIAGNOSTIC_5F714B28);
            let table_name = table.to_string();
            assert!(count.as_ref().contains(table_name.as_str()));
            assert!(data.as_ref().contains(table_name.as_str()));
            table.spec().columns().get().split(',').for_each(|column| {
                assert!(data.as_ref().contains(column));
            });
            assert!(
                data.as_ref()
                    .contains(constants_str::SERVER_ADMIN_DATA_SELECT_COLUMN_SUFFIX)
            );
            assert!(
                data.as_ref()
                    .ends_with(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
            );
        });
}
