use crate::{DataFlt, DataFltJson};

pub(crate) fn data_filter(
    table: server_admin_contract::domain_types::AdminDataTable,
    query: &server_admin_contract::domain_types::AdminDataTableFilterQuery,
) -> Result<Option<DataFlt>, crate::AdminRepositoryError> {
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
        DataFltJson::try_from(json)
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
}
