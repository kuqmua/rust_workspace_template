#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn data_tables_get(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_data_table::AdminDataTable,
    >,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(admin_auth_request.get_headers().as_ref()),
        *admin_auth_request.get_peer(),
        axum_admin_path.permission().as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    )
    .await?;
    let pool = crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
        admin_auth_request.get_state().as_ref().get_pool().as_ref(),
    );
    let view = async {
            let spec = axum_admin_path.spec();
            let columns = {
                let column_names = spec.columns();
                (|| {
                        let generated_fields =
                            crate::admin_generated_table::AdminGeneratedTable::for_data_table(*axum_admin_path)
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
            }?;
            let (base_count_sql, base_sql) = (|| {
                    let base_spec = axum_admin_path.spec();
                    let table_name = axum_admin_path.to_string();
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
                    Ok::<_, crate::admin_repository_error::AdminRepositoryError>((
                        server_admin_core::std_admin_string::StdAdminString::try_from(count)
                            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?,
                        server_admin_core::std_admin_string::StdAdminString::try_from(data)
                            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?,
                    ))
            })()?;
            let filter = {
                let query = axum_admin_query.filter();
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
                                    Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                                };
                            };
                            let fields =
                                crate::admin_generated_table::AdminGeneratedTable::for_data_table(*axum_admin_path)
                                    .map(crate::admin_generated_table::AdminGeneratedTable::field_contracts)
                                    .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                            let field_contract = fields
                                .as_ref()
                                .iter()
                                .find(|candidate| candidate.name().as_ref() == field.as_ref())
                                .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                            if !field_contract.filters().contains(&operation) {
                                return Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue);
                            }
                            let parse_value = |value: &server_admin_contract::admin_filter_value::AdminFilterValue| {
                                let wire_value =
                                    crate::admin_generated_table::AdminGeneratedTable::for_data_table(*axum_admin_path)
                                        .and_then(|generated| {
                                            generated.filter_value(
                                                frontend_contract::form_field_name_ref::FormFieldNameRef::from(field.as_ref()),
                                                frontend_contract::form_value_ref::FormValueRef::from(value.as_ref()),
                                            )
                                        })
                                        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?
                                        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                                serde_json::from_str::<serde_json::Value>(wire_value.as_ref())
                                    .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                            };
                            let mut body = serde_json::Map::new();
                            let _body_operator_replaced = body.insert(
                                constants_str::PG_CRUD_OPERATOR_FIELD.to_owned(),
                                serde_json::Value::String(constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND.to_owned()),
                            );
                            match operation.value_shape() {
                                frontend_contract::filter_value_shape::FilterValueShape::None => {
                                    if query.value().is_some() || query.end().is_some() {
                                        return Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue);
                                    }
                                }
                                frontend_contract::filter_value_shape::FilterValueShape::Range => {
                                    let start = query
                                        .value()
                                        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                                        .and_then(parse_value)?;
                                    let end = query
                                        .end()
                                        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                                        .and_then(parse_value)?;
                                    let mut range = serde_json::Map::new();
                                    let _range_start_replaced =
                                        range.insert(constants_str::PG_CRUD_START_FIELD.to_owned(), start);
                                    let _range_end_replaced =
                                        range.insert(constants_str::PG_CRUD_END_FIELD.to_owned(), end);
                                    let _body_range_replaced = body.insert(
                                        constants_str::PG_CRUD_VALUES_FIELD.to_owned(),
                                        serde_json::Value::Object(range),
                                    );
                                }
                                frontend_contract::filter_value_shape::FilterValueShape::List => {
                                    if query.end().is_some() {
                                        return Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let values = query
                                        .value()
                                        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?
                                        .as_ref()
                                        .split(constants_str::TEXT_ALT_7)
                                        .map(str::trim)
                                        .map(|raw_value| {
                                            let typed_value =
                                                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                                                    raw_value.to_owned(),
                                                )
                                                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                                            parse_value(&typed_value)
                                        })
                                        .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
                                    let _body_list_replaced = body.insert(
                                        constants_str::PG_CRUD_VALUES_FIELD.to_owned(),
                                        serde_json::Value::Array(values),
                                    );
                                }
                                frontend_contract::filter_value_shape::FilterValueShape::Regex => {
                                    if query.end().is_some() {
                                        return Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let value = query
                                        .value()
                                        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                                        .and_then(parse_value)?;
                                    let _body_regex_case_replaced = body.insert(
                                        constants_str::SERVER_ADMIN_FILTER_REGEX_CASE_FIELD.to_owned(),
                                        serde_json::Value::String(
                                            constants_str::SERVER_ADMIN_FILTER_REGEX_SENSITIVE.to_owned(),
                                        ),
                                    );
                                    let _body_regex_value_replaced =
                                        body.insert(constants_str::PG_CRUD_VALUES_FIELD.to_owned(), value);
                                }
                                frontend_contract::filter_value_shape::FilterValueShape::EncodedText => {
                                    if query.end().is_some() {
                                        return Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let value = query
                                        .value()
                                        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
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
                                frontend_contract::filter_value_shape::FilterValueShape::Scalar => {
                                    if query.end().is_some() {
                                        return Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue);
                                    }
                                    let value = query
                                        .value()
                                        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                                        .and_then(parse_value)?;
                                    let _body_scalar_replaced =
                                        body.insert(constants_str::PG_CRUD_VALUES_FIELD.to_owned(), value);
                                }
                            }
                            let mut operation_entry = serde_json::Map::new();
                            let _operation_replaced =
                                operation_entry.insert(format!("{operation:?}"), serde_json::Value::Object(body));
                            let mut field_filters = serde_json::Map::new();
                            let _field_values_replaced = field_filters.insert(
                                constants_str::PG_CRUD_VALUES_FIELD.to_owned(),
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
                                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                            crate::data_flt_json::DataFltJson::try_from(json)
                                .map(Some)
                                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                        })()?;
                        let Some(payload_wrapper) = payload else {
                            return Ok(None);
                        };
                        crate::admin_generated_table::AdminGeneratedTable::for_data_table(*axum_admin_path)
                            .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?
                            .parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                                payload_wrapper.as_ref(),
                            ))
                            .map(Some)
                })()
            }?;
            let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
            let fragment = filter
                .as_ref()
                .map(|value| value.query_part(&mut increment))
                .transpose()
                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
            let (count_sql, sql) = fragment.as_ref().map_or_else(
                || {
                    Ok::<_, crate::admin_repository_error::AdminRepositoryError>((
                        server_admin_core::std_admin_string::StdAdminString::try_from(base_count_sql.as_ref().to_owned())
                            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?,
                        server_admin_core::std_admin_string::StdAdminString::try_from(base_sql.as_ref().to_owned())
                            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?,
                    ))
                },
                |filter_fragment| {
                    let count_sql = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        base_count_sql.as_ref().as_str(),
                    );
                    let data_sql = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        base_sql.as_ref().as_str(),
                    );
                    (|| {
                            let mut filtered_count = count_sql.get().to_owned();
                            filtered_count.push(' ');
                            filtered_count.push_str(filter_fragment.as_ref());
                            let (data_prefix, ordered_suffix) = data_sql
                                .get()
                                .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
                                .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                            let order = ordered_suffix
                                .strip_suffix(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
                                .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                            let limit_index = increment.get().saturating_add(1u64);
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
                            let count = server_admin_core::std_admin_string::StdAdminString::try_from(filtered_count)
                                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                            let data = server_admin_core::std_admin_string::StdAdminString::try_from(filtered_data)
                                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                            Ok((count, data))
                    })()
                },
            )?;
            let unbound_count_query = sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str()));
            let bound_count_query = filter
                .clone()
                .map(|value| {
                    value.query_bind(pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                        unbound_count_query,
                    ))
                })
                .transpose()
                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?
                .map_or_else(
                    || sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str())),
                    pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::into_inner,
                );
            let count_row = bound_count_query
                .fetch_one(*pool)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
            let total = sqlx::Row::try_get::<i64, _>(&count_row, constants_usize::ZERO)
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
            let unbound_data_query = sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str()));
            let bound_data_query = filter
                .map(|value| {
                    value.query_bind(pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                        unbound_data_query,
                    ))
                })
                .transpose()
                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?
                .map_or_else(
                    || sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str())),
                    pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::into_inner,
                )
                .bind(i64::from(u16::from(axum_admin_query.page().limit())))
                .bind(i64::from(u32::from(axum_admin_query.page().offset())));
            let rows = bound_data_query
                .fetch_all(*pool)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?
                .into_iter()
                .map(|row| {
                    sqlx::Row::try_get::<Vec<Option<String>>, _>(&row, constants_usize::ZERO)
                        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                })
                .collect::<Result<Vec<_>, crate::sqlx_admin_error::SqlxAdminError>>()?;
            let items = rows
                .into_iter()
                .map(|row| {
                    let values = row
                        .into_iter()
                        .map(|value| {
                            server_admin_contract::admin_text::AdminText::try_from(
                                value.unwrap_or_else(|| constants_str::SERVER_ADMIN_DATA_NULL.to_owned()),
                            )
                            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                        })
                        .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
                    server_admin_contract::admin_texts::AdminTexts::try_from(values)
                        .map(server_admin_contract::admin_data_row::AdminDataRow::new)
                        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                })
                .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
            Ok(
                server_admin_contract::admin_data_table_view::AdminDataTableView::new(
                    columns,
                    server_admin_contract::admin_data_rows::AdminDataRows::try_from(items)
                        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?,
                    *axum_admin_path,
                    crate::repository_page_total::repository_page_total(crate::admin_page_total_count::AdminPageTotalCount::from(total))?,
                ),
            )
    }
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    Ok(crate::json_response::json_response(view))
}
