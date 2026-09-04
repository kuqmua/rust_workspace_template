#[allow(
    clippy::single_call_fn,
    reason = "data filter remains named because its complete filter contract is covered by focused unit tests"
)]
pub(crate) fn data_filter(
    admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
    admin_data_table_filter_query: &server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery,
) -> Result<Option<crate::data_flt::DataFlt>, crate::admin_repository_error::AdminRepositoryError> {
    let payload = (|| {
        let (Some(field), Some(operation)) = (
            admin_data_table_filter_query.field(),
            admin_data_table_filter_query.operation(),
        ) else {
            return if admin_data_table_filter_query.field().is_none()
                && admin_data_table_filter_query.operation().is_none()
                && admin_data_table_filter_query.value().is_none()
                && admin_data_table_filter_query.end().is_none()
            {
                Ok(None)
            } else {
                Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
            };
        };
        let fields =
            crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table)
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
                crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table)
                    .and_then(|generated| {
                        generated.filter_value(
                            frontend_contract::form_field_name_ref::FormFieldNameRef::from(
                                field.as_ref(),
                            ),
                            frontend_contract::form_value_ref::FormValueRef::from(value.as_ref()),
                        )
                    })
                    .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?;
            serde_json::from_str::<serde_json::Value>(wire_value.as_ref()).map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })
        };
        let mut body = serde_json::Map::new();
        let _body_operator_replaced = body.insert(
            constants_str::PG_CRUD_OPERATOR_FIELD.to_owned(),
            serde_json::Value::String(constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND.to_owned()),
        );
        match operation.value_shape() {
            frontend_contract::filter_value_shape::FilterValueShape::None => {
                if admin_data_table_filter_query.value().is_some()
                    || admin_data_table_filter_query.end().is_some()
                {
                    return Err(
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    );
                }
            }
            frontend_contract::filter_value_shape::FilterValueShape::Range => {
                let start = admin_data_table_filter_query
                    .value()
                    .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                    .and_then(parse_value)?;
                let end = admin_data_table_filter_query
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
                if admin_data_table_filter_query.end().is_some() {
                    return Err(
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    );
                }
                let values = admin_data_table_filter_query
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
                if admin_data_table_filter_query.end().is_some() {
                    return Err(
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    );
                }
                let value = admin_data_table_filter_query
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
                if admin_data_table_filter_query.end().is_some() {
                    return Err(
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    );
                }
                let value = admin_data_table_filter_query.value().ok_or(
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                )?;
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
                if admin_data_table_filter_query.end().is_some() {
                    return Err(
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    );
                }
                let value = admin_data_table_filter_query
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
        let json =
            serde_json::to_string(&serde_json::Value::Object(where_many)).map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })?;
        crate::data_flt_json::DataFltJson::try_from(json)
            .map(Some)
            .map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })
    })()?;
    let Some(payload_wrapper) = payload else {
        return Ok(None);
    };
    crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table)
        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?
        .parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            payload_wrapper.as_ref(),
        ))
        .map(Some)
}
