#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn data_tables_get(
    auth: crate::AdminAuthReq,
    crate::AxumAdminPath(table): crate::AxumAdminPath<
        server_admin_contract::domain_types::AdminDataTable,
    >,
    crate::AxumAdminQuery(data_query): crate::AxumAdminQuery<
        server_admin_contract::domain_types::AdminDataTableQuery,
    >,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        table.permission().as_str(),
        crate::StdAdminBool::from(false),
    )
    .await?;
    let pool =
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref());
    let view = async {
            let spec = table.spec();
            let columns = {
                let column_names = spec.columns();
                (|| {
                        let generated_fields =
                            crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
                                .map(crate::domain_types::generated_tables::AdminGeneratedTable::field_contracts);
                        let columns = column_names
                            .get()
                            .split(',')
                            .map(|raw_name| {
                                let generated_field = generated_fields.as_ref().and_then(|fields| {
                                    AsRef::<[frontend_contract::FieldContract]>::as_ref(fields)
                                        .iter()
                                        .find(|field| field.name().as_ref() == raw_name)
                                });
                                let label_text = generated_field.map_or_else(
                                    || raw_name.to_owned(),
                                    |field| field.label().as_ref().to_owned(),
                                );
                                let input_kind = generated_field.map_or(frontend_contract::InputKind::Text, |field| {
                                    field.type_contract().input_kind()
                                });
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
                                        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                                let label = server_admin_contract::domain_types::AdminText::try_from(label_text)
                                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                                let name =
                                    server_admin_contract::domain_types::AdminText::try_from(raw_name.to_owned())
                                        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                                Ok(server_admin_contract::domain_types::AdminDataColumn::new(
                                    filters, input_kind, label, name,
                                ))
                            })
                            .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
                        server_admin_contract::domain_types::AdminDataColumns::try_from(columns)
                            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
                })()
            }?;
            let (base_count_sql, base_sql) = (|| {
                    let base_spec = table.spec();
                    let table_name = table.to_string();
                    let mut count = constants_str::SERVER_ADMIN_DATA_COUNT_PREFIX.to_owned();
                    count.push_str(table_name.as_str());
                    let mut data = base_spec.columns().get().split(',').enumerate().fold(
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
                    data.push_str(base_spec.order().get());
                    data.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR);
                    Ok::<_, crate::AdminRepositoryError>((
                        crate::domain_types::StdAdminString::try_from(count)
                            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                        crate::domain_types::StdAdminString::try_from(data)
                            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                    ))
            })()?;
            let filter = {
                let query = data_query.filter();
                (|| {
                        let payload = (|| {
                            let (Some(field), Some(operation)) = (query.field(), query.operation()) else {
                                return if query.field().is_none()
                                    && query.operation().is_none()
                                    && query.value().is_none()
                                    && query.end().is_none()
                                {
                                    Ok(None)
                                } else {
                                    Err(crate::AdminRepositoryError::InvalidStoredValue)
                                };
                            };
                            let fields =
                                crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
                                    .map(crate::domain_types::generated_tables::AdminGeneratedTable::field_contracts)
                                    .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?;
                            let field_contract = fields
                                .as_ref()
                                .iter()
                                .find(|candidate| candidate.name().as_ref() == field.as_ref())
                                .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?;
                            if !field_contract.filters().contains(&operation) {
                                return Err(crate::AdminRepositoryError::InvalidStoredValue);
                            }
                            let parse_value = |value: &server_admin_contract::domain_types::AdminFilterValue| {
                                let wire_value =
                                    crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
                                        .and_then(|generated| {
                                            generated.filter_value(
                                                frontend_contract::FormFieldNameRef::from(field.as_ref()),
                                                frontend_contract::FormValueRef::from(value.as_ref()),
                                            )
                                        })
                                        .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?
                                        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                                serde_json::from_str::<serde_json::Value>(wire_value.as_ref())
                                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
                            };
                            let mut body = serde_json::Map::new();
                            let _body_operator_replaced = body.insert(
                                constants_str::PG_CRUD_OPERATOR_FIELD.to_owned(),
                                serde_json::Value::String(constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND.to_owned()),
                            );
                            match operation.value_shape() {
                                frontend_contract::FilterValueShape::None => {
                                    if query.value().is_some() || query.end().is_some() {
                                        return Err(crate::AdminRepositoryError::InvalidStoredValue);
                                    }
                                }
                                frontend_contract::FilterValueShape::Range => {
                                    let start = query
                                        .value()
                                        .ok_or(crate::AdminRepositoryError::InvalidStoredValue)
                                        .and_then(parse_value)?;
                                    let end = query
                                        .end()
                                        .ok_or(crate::AdminRepositoryError::InvalidStoredValue)
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
                                frontend_contract::FilterValueShape::List => {
                                    if query.end().is_some() {
                                        return Err(crate::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let values = query
                                        .value()
                                        .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?
                                        .as_ref()
                                        .split(constants_str::TEXT_ALT_7)
                                        .map(str::trim)
                                        .map(|raw_value| {
                                            let typed_value =
                                                server_admin_contract::domain_types::AdminFilterValue::try_from(
                                                    raw_value.to_owned(),
                                                )
                                                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                                            parse_value(&typed_value)
                                        })
                                        .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
                                    let _body_list_replaced = body.insert(
                                        constants_str::PG_CRUD_V_FIELD.to_owned(),
                                        serde_json::Value::Array(values),
                                    );
                                }
                                frontend_contract::FilterValueShape::Regex => {
                                    if query.end().is_some() {
                                        return Err(crate::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let value = query
                                        .value()
                                        .ok_or(crate::AdminRepositoryError::InvalidStoredValue)
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
                                frontend_contract::FilterValueShape::EncodedText => {
                                    if query.end().is_some() {
                                        return Err(crate::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let value = query
                                        .value()
                                        .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?;
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
                                frontend_contract::FilterValueShape::Scalar => {
                                    if query.end().is_some() {
                                        return Err(crate::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let value = query
                                        .value()
                                        .ok_or(crate::AdminRepositoryError::InvalidStoredValue)
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
                                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                            crate::DataFltJson::try_from(json)
                                .map(Some)
                                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
                        })()?;
                        let Some(payload_wrapper) = payload else {
                            return Ok(None);
                        };
                        crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
                            .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?
                            .parse_filter(crate::domain_types::StdAdminStrRef::from(
                                payload_wrapper.as_ref(),
                            ))
                            .map(Some)
                })()
            }?;
            let mut increment = pg_crud_common::domain_types::QueryPartIncrement::from(constants_u64::ZERO);
            let fragment = filter
                .as_ref()
                .map(|value| value.query_part(&mut increment))
                .transpose()
                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
            let (count_sql, sql) = fragment.as_ref().map_or_else(
                || {
                    Ok::<_, crate::AdminRepositoryError>((
                        crate::domain_types::StdAdminString::try_from(base_count_sql.as_ref().to_owned())
                            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                        crate::domain_types::StdAdminString::try_from(base_sql.as_ref().to_owned())
                            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                    ))
                },
                |filter_fragment| {
                    let count_sql = crate::domain_types::StdAdminStrRef::from(
                        base_count_sql.as_ref().as_str(),
                    );
                    let data_sql = crate::domain_types::StdAdminStrRef::from(
                        base_sql.as_ref().as_str(),
                    );
                    let bind_count = increment;
                    (|| {
                            let mut filtered_count = count_sql.get().to_owned();
                            filtered_count.push(' ');
                            filtered_count.push_str(filter_fragment.as_ref());
                            let (data_prefix, ordered_suffix) = data_sql
                                .get()
                                .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
                                .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?;
                            let order = ordered_suffix
                                .strip_suffix(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
                                .ok_or(crate::AdminRepositoryError::InvalidStoredValue)?;
                            let limit_index = bind_count.get().saturating_add(1u64);
                            let offset_index = limit_index.saturating_add(1u64);
                            let mut filtered_data = data_prefix.to_owned();
                            filtered_data.push(' ');
                            filtered_data.push_str(filter_fragment.as_ref());
                            filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
                            filtered_data.push_str(order);
                            filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_PREFIX);
                            filtered_data.push_str(limit_index.to_string().as_str());
                            filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_OFFSET_PREFIX);
                            filtered_data.push_str(offset_index.to_string().as_str());
                            let count = crate::domain_types::StdAdminString::try_from(filtered_count)
                                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                            let data = crate::domain_types::StdAdminString::try_from(filtered_data)
                                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
                            Ok((count, data))
                    })()
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
                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?
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
                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?
                .map_or_else(
                    || sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str())),
                    pg_crud_common::domain_types::SqlxPostgresQuery::into_inner,
                )
                .bind(i64::from(u16::from(data_query.page().limit())))
                .bind(i64::from(u32::from(data_query.page().offset())));
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
                            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
                        })
                        .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
                    server_admin_contract::domain_types::AdminTexts::try_from(values)
                        .map(server_admin_contract::domain_types::AdminDataRow::new)
                        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
                })
                .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
            Ok(
                server_admin_contract::domain_types::AdminDataTableView::new(
                    columns,
                    server_admin_contract::domain_types::AdminDataRows::try_from(items)
                        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                    table,
                    crate::repository_page_total(crate::AdminPageTotalCount::from(total))?,
                ),
            )
    }
    .await
    .map_err(crate::shared::map_repository_error::map_repository_error)?;
    Ok(crate::shared::json_response::json_response(view))
}
