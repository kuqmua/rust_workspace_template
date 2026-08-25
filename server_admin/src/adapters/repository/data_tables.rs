#![allow(clippy::single_call_fn)] // one bounded query serves the read-only table inspection boundary

fn data_columns(
    table: server_admin_contract::domain_types::AdminDataTable,
    column_names: server_admin_contract::domain_types::AdminDataColumnsCsvRef<'_>,
) -> Result<server_admin_contract::domain_types::AdminDataColumns, super::AdminRepositoryError> {
    let generated_fields =
        crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
            .map(crate::domain_types::generated_tables::AdminGeneratedTable::field_contracts);
    let columns = column_names
        .get()
        .split(',')
        .map(|raw_name| {
            let generated_field = generated_fields.as_ref().and_then(|fields| {
                AsRef::<[frontend_contract::domain_types::FieldContract]>::as_ref(fields)
                    .iter()
                    .find(|field| field.name().as_ref() == raw_name)
            });
            let label_text = generated_field.map_or_else(
                || raw_name.to_owned(),
                |field| field.label().as_ref().to_owned(),
            );
            let input_kind = generated_field.map_or(
                server_admin_contract::domain_types::AdminDataInputKind::Text,
                |field| {
                    server_admin_contract::domain_types::AdminDataInputKind::from(
                        field.type_contract().input_kind(),
                    )
                },
            );
            let raw_filters = generated_field.map_or_else(Vec::new, |field| {
                field
                    .filters()
                    .iter()
                    .copied()
                    .map(server_admin_contract::domain_types::AdminDataFilter::from)
                    .collect::<Vec<_>>()
            });
            let filters =
                server_admin_contract::domain_types::AdminDataFilters::try_from(raw_filters)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
            let label = server_admin_contract::domain_types::AdminText::try_from(label_text)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
            let name =
                server_admin_contract::domain_types::AdminText::try_from(raw_name.to_owned())
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
            Ok(server_admin_contract::domain_types::AdminDataColumn::new(
                filters, input_kind, label, name,
            ))
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    server_admin_contract::domain_types::AdminDataColumns::try_from(columns)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
}

fn base_sql(
    table: server_admin_contract::domain_types::AdminDataTable,
) -> Result<
    (
        crate::domain_types::StdAdminString,
        crate::domain_types::StdAdminString,
    ),
    super::AdminRepositoryError,
> {
    let spec = table.spec();
    let table_name = table.to_string();
    let mut count = constants_str::SERVER_ADMIN_DATA_COUNT_PREFIX.to_owned();
    count.push_str(table_name.as_str());
    let mut data = spec.columns().get().split(',').enumerate().fold(
        constants_str::SERVER_ADMIN_DATA_SELECT_ARRAY_PREFIX.to_owned(),
        |mut sql, (index, column)| {
            if index > constants_usize::ZERO {
                sql.push_str(constants_str::TEXT_ALT_7);
            }
            sql.push_str(constants_str::SERVER_ADMIN_DATA_SELECT_COLUMN_PREFIX);
            sql.push_str(column);
            sql.push_str(constants_str::SERVER_ADMIN_DATA_SELECT_COLUMN_SUFFIX);
            sql
        },
    );
    data.push_str(constants_str::SERVER_ADMIN_DATA_SELECT_FROM);
    data.push_str(table_name.as_str());
    data.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
    data.push_str(spec.order().get());
    data.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR);
    Ok((
        crate::domain_types::StdAdminString::try_from(count)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        crate::domain_types::StdAdminString::try_from(data)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
    ))
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataPermissionsFlt(
    crate::domain_types::generated_tables::StdOptionalOptionalAdminPermissionsWhereMany,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataRolePermissionsFlt(
    crate::domain_types::generated_tables::StdOptionalOptionalAdminRolePermissionsWhereMany,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataRolesFlt(
    crate::domain_types::generated_tables::StdOptionalOptionalAdminRolesWhereMany,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataSystemSettingsFlt(
    crate::domain_types::generated_tables::StdOptionalOptionalAdminSystemSettingsWhereMany,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataUserRolesFlt(
    crate::domain_types::generated_tables::StdOptionalOptionalAdminUserRolesWhereMany,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataUsersFlt(
    crate::domain_types::generated_tables::StdOptionalOptionalAdminUsersWhereMany,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
struct DataFltJson(String);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone)]
pub(crate) enum DataFlt {
    Permissions(DataPermissionsFlt),
    RolePermissions(DataRolePermissionsFlt),
    Roles(DataRolesFlt),
    SystemSettings(DataSystemSettingsFlt),
    UserRoles(DataUserRolesFlt),
    Users(DataUsersFlt),
}
impl DataFlt {
    fn query_part(
        &self,
        increment: &mut pg_crud_common::domain_types::QueryPartIncrement,
    ) -> Result<
        pg_crud_common::domain_types::QueryPartFragment,
        pg_crud_common::domain_types::QueryPartError,
    > {
        let column = constants_str::PG_CRUD_EMPTY_SQL_SUFFIX;
        match self {
            Self::Permissions(value) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                    &value.0,
                    increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&column),
                    pg_crud_common::domain_types::AddOperator::from(false),
                )
            }
            Self::RolePermissions(value) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                    &value.0,
                    increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&column),
                    pg_crud_common::domain_types::AddOperator::from(false),
                )
            }
            Self::Roles(value) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                &value.0,
                increment,
                pg_crud_common::domain_types::SqlColumnRef::from(&column),
                pg_crud_common::domain_types::AddOperator::from(false),
            ),
            Self::SystemSettings(value) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                    &value.0,
                    increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&column),
                    pg_crud_common::domain_types::AddOperator::from(false),
                )
            }
            Self::UserRoles(value) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                &value.0,
                increment,
                pg_crud_common::domain_types::SqlColumnRef::from(&column),
                pg_crud_common::domain_types::AddOperator::from(false),
            ),
            Self::Users(value) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                &value.0,
                increment,
                pg_crud_common::domain_types::SqlColumnRef::from(&column),
                pg_crud_common::domain_types::AddOperator::from(false),
            ),
        }
    }
    fn query_bind(
        self,
        query: pg_crud_common::domain_types::SqlxPostgresQuery<'_>,
    ) -> Result<
        pg_crud_common::domain_types::SqlxPostgresQuery<'_>,
        pg_crud_common::domain_types::SqlxPostgresQueryBindError,
    > {
        match self {
            Self::Permissions(DataPermissionsFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::RolePermissions(DataRolePermissionsFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::Roles(DataRolesFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::SystemSettings(DataSystemSettingsFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::UserRoles(DataUserRolesFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::Users(DataUsersFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
        }
    }
}

fn filter_wire_value(
    table: server_admin_contract::domain_types::AdminDataTable,
    field: frontend_contract::domain_types::FormFieldNameRef<'_>,
    value: frontend_contract::domain_types::FormValueRef<'_>,
) -> Result<frontend_contract::domain_types::FilterWireJson, super::AdminRepositoryError> {
    let parsed = crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
        .and_then(|generated| generated.filter_value(field, value))
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    Ok(parsed)
}

fn filter_payload(
    table: server_admin_contract::domain_types::AdminDataTable,
    query: &server_admin_contract::domain_types::AdminDataTableFilterQuery,
) -> Result<Option<DataFltJson>, super::AdminRepositoryError> {
    let (Some(field), Some(operation)) = (query.field(), query.operation()) else {
        return if query.field().is_none()
            && query.operation().is_none()
            && query.value().is_none()
            && query.end().is_none()
        {
            Ok(None)
        } else {
            Err(super::AdminRepositoryError::InvalidStoredValue)
        };
    };
    let fields = crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
        .map(crate::domain_types::generated_tables::AdminGeneratedTable::field_contracts)
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?;
    let field_contract = fields
        .as_ref()
        .iter()
        .find(|candidate| candidate.name().as_ref() == field.as_ref())
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?;
    if !field_contract.filters().contains(&operation) {
        return Err(super::AdminRepositoryError::InvalidStoredValue);
    }
    let parse_value = |value: &server_admin_contract::domain_types::AdminFilterValue| {
        let wire_value = filter_wire_value(
            table,
            frontend_contract::domain_types::FormFieldNameRef::from(field.as_ref()),
            frontend_contract::domain_types::FormValueRef::from(value.as_ref()),
        )?;
        serde_json::from_str::<serde_json::Value>(wire_value.as_ref())
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
    };
    let mut body = serde_json::Map::new();
    let _body_operator_replaced = body.insert(
        constants_str::PG_CRUD_OPERATOR_FIELD.to_owned(),
        serde_json::Value::String(constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND.to_owned()),
    );
    match operation.value_shape() {
        frontend_contract::domain_types::FilterValueShape::None => {
            if query.value().is_some() || query.end().is_some() {
                return Err(super::AdminRepositoryError::InvalidStoredValue);
            }
        }
        frontend_contract::domain_types::FilterValueShape::Range => {
            let start = query
                .value()
                .ok_or(super::AdminRepositoryError::InvalidStoredValue)
                .and_then(parse_value)?;
            let end = query
                .end()
                .ok_or(super::AdminRepositoryError::InvalidStoredValue)
                .and_then(parse_value)?;
            let mut range = serde_json::Map::new();
            let _range_start_replaced =
                range.insert(constants_str::PG_CRUD_START_FIELD.to_owned(), start);
            let _range_end_replaced =
                range.insert(constants_str::PG_CRUD_END_FIELD.to_owned(), end);
            let _body_range_replaced = body.insert(
                constants_str::PG_CRUD_V_FIELD.to_owned(),
                serde_json::Value::Object(range),
            );
        }
        frontend_contract::domain_types::FilterValueShape::List => {
            if query.end().is_some() {
                return Err(super::AdminRepositoryError::InvalidStoredValue);
            }
            let values = query
                .value()
                .ok_or(super::AdminRepositoryError::InvalidStoredValue)?
                .as_ref()
                .split(constants_str::TEXT_ALT_7)
                .map(str::trim)
                .map(|raw_value| {
                    let typed_value =
                        server_admin_contract::domain_types::AdminFilterValue::try_from(
                            raw_value.to_owned(),
                        )
                        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
                    parse_value(&typed_value)
                })
                .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
            let _body_list_replaced = body.insert(
                constants_str::PG_CRUD_V_FIELD.to_owned(),
                serde_json::Value::Array(values),
            );
        }
        frontend_contract::domain_types::FilterValueShape::Regex => {
            if query.end().is_some() {
                return Err(super::AdminRepositoryError::InvalidStoredValue);
            }
            let value = query
                .value()
                .ok_or(super::AdminRepositoryError::InvalidStoredValue)
                .and_then(parse_value)?;
            let _body_regex_case_replaced = body.insert(
                constants_str::SERVER_ADMIN_FILTER_REGEX_CASE_FIELD.to_owned(),
                serde_json::Value::String(
                    constants_str::SERVER_ADMIN_FILTER_REGEX_SENSITIVE.to_owned(),
                ),
            );
            let _body_regex_value_replaced =
                body.insert(constants_str::PG_CRUD_V_FIELD.to_owned(), value);
        }
        frontend_contract::domain_types::FilterValueShape::EncodedText => {
            if query.end().is_some() {
                return Err(super::AdminRepositoryError::InvalidStoredValue);
            }
            let value = query
                .value()
                .ok_or(super::AdminRepositoryError::InvalidStoredValue)?;
            let _body_encode_format_replaced = body.insert(
                constants_str::SERVER_ADMIN_FILTER_ENCODE_FORMAT_FIELD.to_owned(),
                serde_json::Value::String(
                    constants_str::SERVER_ADMIN_FILTER_ENCODE_BASE64.to_owned(),
                ),
            );
            let _body_encoded_value_replaced = body.insert(
                constants_str::SERVER_ADMIN_FILTER_ENCODED_VALUE_FIELD.to_owned(),
                serde_json::Value::String(value.as_ref().to_owned()),
            );
        }
        frontend_contract::domain_types::FilterValueShape::Scalar => {
            if query.end().is_some() {
                return Err(super::AdminRepositoryError::InvalidStoredValue);
            }
            let value = query
                .value()
                .ok_or(super::AdminRepositoryError::InvalidStoredValue)
                .and_then(parse_value)?;
            let _body_scalar_replaced =
                body.insert(constants_str::PG_CRUD_V_FIELD.to_owned(), value);
        }
    }
    let mut operation_entry = serde_json::Map::new();
    let _operation_replaced =
        operation_entry.insert(format!("{operation:?}"), serde_json::Value::Object(body));
    let mut field_filters = serde_json::Map::new();
    let _field_values_replaced = field_filters.insert(
        constants_str::PG_CRUD_V_FIELD.to_owned(),
        serde_json::Value::Array(vec![serde_json::Value::Object(operation_entry)]),
    );
    let _field_operator_replaced = field_filters.insert(
        constants_str::PG_CRUD_OPERATOR_FIELD.to_owned(),
        serde_json::Value::String(constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND.to_owned()),
    );
    let mut where_many = serde_json::Map::new();
    let _field_replaced = where_many.insert(
        field.as_ref().to_owned(),
        serde_json::Value::Object(field_filters),
    );
    let json = serde_json::to_string(&serde_json::Value::Object(where_many))
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    DataFltJson::try_from(json)
        .map(Some)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
}

fn data_filter(
    table: server_admin_contract::domain_types::AdminDataTable,
    query: &server_admin_contract::domain_types::AdminDataTableFilterQuery,
) -> Result<Option<DataFlt>, super::AdminRepositoryError> {
    let Some(payload_wrapper) = filter_payload(table, query)? else {
        return Ok(None);
    };
    crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?
        .parse_filter(crate::domain_types::StdAdminStrRef::from(
            payload_wrapper.as_ref(),
        ))
        .map(Some)
}

fn filtered_sql(
    count_sql: crate::domain_types::StdAdminStrRef<'_>,
    data_sql: crate::domain_types::StdAdminStrRef<'_>,
    fragment: &pg_crud_common::domain_types::QueryPartFragment,
    bind_count: pg_crud_common::domain_types::QueryPartIncrement,
) -> Result<
    (
        crate::domain_types::StdAdminString,
        crate::domain_types::StdAdminString,
    ),
    super::AdminRepositoryError,
> {
    let mut filtered_count = count_sql.get().to_owned();
    filtered_count.push(' ');
    filtered_count.push_str(fragment.as_ref());
    let (data_prefix, ordered_suffix) = data_sql
        .get()
        .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?;
    let order = ordered_suffix
        .strip_suffix(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?;
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
    let count = crate::domain_types::StdAdminString::try_from(filtered_count)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    let data = crate::domain_types::StdAdminString::try_from(filtered_data)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    Ok((count, data))
}

pub(crate) async fn read(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    table: server_admin_contract::domain_types::AdminDataTable,
    query: &server_admin_contract::domain_types::AdminDataTableQuery,
) -> Result<server_admin_contract::domain_types::AdminDataTableView, super::AdminRepositoryError> {
    let spec = table.spec();
    let columns = data_columns(table, spec.columns())?;
    let (base_count_sql, base_sql) = base_sql(table)?;
    let filter = data_filter(table, query.filter())?;
    let mut increment = pg_crud_common::domain_types::QueryPartIncrement::from(constants_u64::ZERO);
    let fragment = filter
        .as_ref()
        .map(|value| value.query_part(&mut increment))
        .transpose()
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    let (count_sql, sql) = fragment.as_ref().map_or_else(
        || {
            Ok((
                crate::domain_types::StdAdminString::try_from(base_count_sql.as_ref().to_owned())
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                crate::domain_types::StdAdminString::try_from(base_sql.as_ref().to_owned())
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        },
        |filter_fragment| {
            filtered_sql(
                crate::domain_types::StdAdminStrRef::from(base_count_sql.as_ref().as_str()),
                crate::domain_types::StdAdminStrRef::from(base_sql.as_ref().as_str()),
                filter_fragment,
                increment,
            )
        },
    )?;
    let unbound_count_query = sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str()));
    let bound_count_query = filter
        .clone()
        .map(|value| {
            value.query_bind(pg_crud_common::domain_types::SqlxPostgresQuery::from(
                unbound_count_query,
            ))
        })
        .transpose()
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?
        .map_or_else(
            || sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str())),
            pg_crud_common::domain_types::SqlxPostgresQuery::into_inner,
        );
    let count_row = bound_count_query
        .fetch_one(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let total = sqlx::Row::try_get::<i64, _>(&count_row, constants_usize::ZERO)
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let unbound_data_query = sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str()));
    let bound_data_query = filter
        .map(|value| {
            value.query_bind(pg_crud_common::domain_types::SqlxPostgresQuery::from(
                unbound_data_query,
            ))
        })
        .transpose()
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?
        .map_or_else(
            || sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str())),
            pg_crud_common::domain_types::SqlxPostgresQuery::into_inner,
        )
        .bind(i64::from(u16::from(query.page().limit())))
        .bind(i64::from(u32::from(query.page().offset())));
    let rows = bound_data_query
        .fetch_all(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .into_iter()
        .map(|row| {
            sqlx::Row::try_get::<Vec<Option<String>>, _>(&row, constants_usize::ZERO)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
        .collect::<Result<Vec<_>, crate::domain_types::SqlxAdminError>>()?;
    let items = rows
        .into_iter()
        .map(|row| {
            let values = row
                .into_iter()
                .map(|value| {
                    server_admin_contract::domain_types::AdminText::try_from(
                        value.unwrap_or_else(|| constants_str::SERVER_ADMIN_DATA_NULL.to_owned()),
                    )
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
                })
                .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
            server_admin_contract::domain_types::AdminTexts::try_from(values)
                .map(server_admin_contract::domain_types::AdminDataRow::new)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok(
        server_admin_contract::domain_types::AdminDataTableView::new(
            columns,
            server_admin_contract::domain_types::AdminDataRows::try_from(items)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            table,
            super::page_total(super::AdminPageTotalCount::from(total))?,
        ),
    )
}

#[cfg(test)]
mod tests {
    fn filter_query(
        field: &str,
        operation: frontend_contract::domain_types::FilterOperation,
        filter_value: Option<&str>,
        end: Option<&str>,
    ) -> server_admin_contract::domain_types::AdminDataTableQuery {
        server_admin_contract::domain_types::AdminDataTableQuery::new(
            server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
                Some(
                    server_admin_contract::domain_types::AdminFilterField::try_from(
                        field.to_owned(),
                    )
                    .expect("a17498dc filter_query invariant must hold"),
                ),
                Some(operation),
                filter_value.map(|raw_filter_value| {
                    server_admin_contract::domain_types::AdminFilterValue::try_from(
                        raw_filter_value.to_owned(),
                    )
                    .expect("f064fcd7 filter_query invariant must hold")
                }),
                end.map(|raw_filter_end| {
                    server_admin_contract::domain_types::AdminFilterValue::try_from(
                        raw_filter_end.to_owned(),
                    )
                    .expect("9b563f27 filter_query invariant must hold")
                }),
            ),
            server_admin_contract::domain_types::AdminTableQuery::default(),
        )
    }

    #[test]
    fn generated_table_fields_supply_client_column_metadata() {
        let columns = super::data_columns(
            server_admin_contract::domain_types::AdminDataTable::Users,
            server_admin_contract::domain_types::AdminDataTable::Users
                .spec()
                .columns(),
        )
        .expect(
            "f3c897af generated_table_fields_supply_client_column_metadata invariant must hold",
        );
        let id = columns
            .as_slice()
            .iter()
            .find(|column| column.name().as_ref() == constants_str::SQL_NAMES_ID);
        assert!(id.is_some_and(|column| {
            column.input_kind() == server_admin_contract::domain_types::AdminDataInputKind::Number
        }));
        let login = columns
            .as_slice()
            .iter()
            .find(|column| column.name().as_ref() == constants_str::LOGIN)
            .expect(
                "7a340d1f generated_table_fields_supply_client_column_metadata invariant must hold",
            );
        assert_eq!(
            login
                .filters()
                .iter()
                .map(server_admin_contract::domain_types::AdminDataFilter::operation)
                .collect::<Vec<_>>(),
            vec![
                frontend_contract::domain_types::FilterOperation::Eq,
                frontend_contract::domain_types::FilterOperation::Regex,
            ]
        );
    }

    #[test]
    fn generated_where_filter_builds_typed_table_predicate() {
        let query = filter_query(
            constants_str::LOGIN,
            frontend_contract::domain_types::FilterOperation::Eq,
            Some("alice"),
            None,
        );
        let filter = super::data_filter(
            server_admin_contract::domain_types::AdminDataTable::Users,
            query.filter(),
        )
        .expect("4e779df0 generated_where_filter_builds_typed_table_predicate invariant must hold")
        .expect("d9c8cf39 generated_where_filter_builds_typed_table_predicate invariant must hold");
        let mut increment =
            pg_crud_common::domain_types::QueryPartIncrement::from(constants_u64::ZERO);

        let fragment = filter.query_part(&mut increment).expect(
            "a25fe142 generated_where_filter_builds_typed_table_predicate invariant must hold",
        );

        assert!(fragment.as_ref().contains(constants_str::LOGIN));
        assert!(fragment.as_ref().contains("$1"));
        assert_eq!(increment.get(), 1u64);
    }

    #[test]
    fn unsupported_field_operation_is_rejected() {
        let query = filter_query(
            constants_str::LOGIN,
            frontend_contract::domain_types::FilterOperation::Between,
            Some("alice"),
            Some("bob"),
        );

        assert!(
            super::data_filter(
                server_admin_contract::domain_types::AdminDataTable::Users,
                query.filter(),
            )
            .is_err()
        );
    }

    #[test]
    fn empty_filter_query_omits_the_predicate() {
        let query = server_admin_contract::domain_types::AdminDataTableFilterQuery::default();

        assert!(
            super::data_filter(
                server_admin_contract::domain_types::AdminDataTable::Users,
                &query
            )
            .expect("fd36a6f5 empty_filter_query_omits_the_predicate invariant must hold")
            .is_none()
        );
    }

    #[test]
    fn incomplete_filter_queries_are_rejected() {
        let field = server_admin_contract::domain_types::AdminFilterField::try_from(
            constants_str::LOGIN.to_owned(),
        )
        .expect("f1832a34 incomplete_filter_queries_are_rejected invariant must hold");
        let value =
            server_admin_contract::domain_types::AdminFilterValue::try_from(String::from("alice"))
                .expect("16849a06 incomplete_filter_queries_are_rejected invariant must hold");
        let queries = [
            server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
                Some(field),
                None,
                None,
                None,
            ),
            server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
                None,
                Some(frontend_contract::domain_types::FilterOperation::Eq),
                None,
                None,
            ),
            server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
                None,
                None,
                Some(value.clone()),
                None,
            ),
            server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
                None,
                None,
                None,
                Some(value),
            ),
        ];

        assert!(queries.iter().all(|query| {
            super::data_filter(
                server_admin_contract::domain_types::AdminDataTable::Users,
                query,
            )
            .is_err()
        }));
    }

    #[test]
    fn unknown_filter_field_is_rejected() {
        let query = filter_query(
            "unknown",
            frontend_contract::domain_types::FilterOperation::Eq,
            Some("alice"),
            None,
        );

        assert!(
            super::data_filter(
                server_admin_contract::domain_types::AdminDataTable::Users,
                query.filter()
            )
            .is_err()
        );
    }

    #[test]
    fn scalar_and_regex_filters_reject_range_end_values() {
        let operations = [
            frontend_contract::domain_types::FilterOperation::Eq,
            frontend_contract::domain_types::FilterOperation::Regex,
        ];
        assert!(operations.into_iter().all(|operation| {
            let query = filter_query(constants_str::LOGIN, operation, Some("alice"), Some("bob"));
            super::data_filter(
                server_admin_contract::domain_types::AdminDataTable::Users,
                query.filter(),
            )
            .is_err()
        }));
    }

    #[test]
    fn regex_filter_builds_a_typed_predicate() {
        let query = filter_query(
            constants_str::LOGIN,
            frontend_contract::domain_types::FilterOperation::Regex,
            Some("^alice"),
            None,
        );
        let filter = super::data_filter(
            server_admin_contract::domain_types::AdminDataTable::Users,
            query.filter(),
        )
        .expect("e0b1326d regex_filter_builds_a_typed_predicate invariant must hold")
        .expect("8a4e68fb regex_filter_builds_a_typed_predicate invariant must hold");
        let mut increment =
            pg_crud_common::domain_types::QueryPartIncrement::from(constants_u64::ZERO);

        let fragment = filter
            .query_part(&mut increment)
            .expect("9f5e101d regex_filter_builds_a_typed_predicate invariant must hold");

        assert!(fragment.as_ref().contains(constants_str::LOGIN));
        assert_eq!(increment.get(), 1u64);
    }

    #[test]
    fn filtered_sql_places_pagination_after_filter_binds() {
        let fragment = pg_crud_common::domain_types::QueryPartFragment::try_from(String::from(
            "where login = $1",
        ))
        .expect("45d292b8 filtered_sql_places_pagination_after_filter_binds invariant must hold");

        let (base_count, base_data) = super::base_sql(
            server_admin_contract::domain_types::AdminDataTable::Users,
        )
        .expect("44c43299 filtered_sql_places_pagination_after_filter_binds invariant must hold");
        let (_count, data) = super::filtered_sql(
            crate::domain_types::StdAdminStrRef::from(base_count.as_ref().as_str()),
            crate::domain_types::StdAdminStrRef::from(base_data.as_ref().as_str()),
            &fragment,
            pg_crud_common::domain_types::QueryPartIncrement::from(1u64),
        )
        .expect("c33365ba filtered_sql_places_pagination_after_filter_binds invariant must hold");

        assert!(data.as_ref().contains("where login = $1"));
        assert!(data.as_ref().contains("LIMIT $2 OFFSET $3"));
    }

    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn table_spec_generates_bounded_projection_and_count_sql_for_every_table() {
        server_admin_contract::domain_types::AdminDataTable::ALL
            .into_iter()
            .for_each(|table| {
                let (count, data) = super::base_sql(table).expect("5f714b28 table_spec_generates_bounded_projection_and_count_sql_for_every_table invariant must hold");
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
}
