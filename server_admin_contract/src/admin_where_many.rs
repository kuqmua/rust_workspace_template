#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct AdminWhereMany(
    bounded_types::bounded_string::BoundedString<
        2usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);

impl TryFrom<String> for AdminWhereMany {
    type Error = crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = serde_json::from_str::<serde_json::Value>(&value).map_err(
            crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError::Json,
        )?;
        if !parsed.is_object() {
            return Err(crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError::NotObject);
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError::Length)
    }
}

impl serde::Serialize for AdminWhereMany {
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serde_json::from_str::<serde_json::Value>(self.as_ref())
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for AdminWhereMany {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Self::try_from(value.to_string()).map_err(serde::de::Error::custom)
    }
}

impl utoipa::PartialSchema for AdminWhereMany {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new().into()
    }
}

impl utoipa::ToSchema for AdminWhereMany {}

impl AdminWhereMany {
    pub fn try_from_identifier_filter(
        admin_data_table_filter_query: &crate::admin_data_table_filter_query::AdminDataTableFilterQuery,
    ) -> Result<Option<Self>, crate::admin_identifier_filter_error::AdminIdentifierFilterError>
    {
        Self::try_from_filter(
            admin_data_table_filter_query,
            frontend_contract::input_kind::InputKind::Number,
        )
    }

    pub fn try_from_filter(
        admin_data_table_filter_query: &crate::admin_data_table_filter_query::AdminDataTableFilterQuery,
        input_kind: frontend_contract::input_kind::InputKind,
    ) -> Result<Option<Self>, crate::admin_identifier_filter_error::AdminIdentifierFilterError>
    {
        let (optional_field, optional_operation, value, end) = (
            admin_data_table_filter_query.field(),
            admin_data_table_filter_query.operation(),
            admin_data_table_filter_query.value(),
            admin_data_table_filter_query.end(),
        );
        let (Some(identifier_field), Some(operation)) = (optional_field, optional_operation) else {
            return if optional_field.is_none()
                && optional_operation.is_none()
                && value.is_none()
                && end.is_none()
            {
                Ok(None)
            } else {
                Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete)
            };
        };
        let parse_identifier = |filter_value: &crate::admin_filter_value::AdminFilterValue| {
            filter_value
                .as_ref()
                .parse::<i64>()
                .ok()
                .filter(|identifier| identifier.is_positive())
                .map(serde_json::Value::from)
                .ok_or(
                    crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
                )
        };
        let identifier_values = || {
            Ok(match operation {
            frontend_contract::filter_operation::FilterOperation::Eq
            | frontend_contract::filter_operation::FilterOperation::GreaterThan => {
                if end.is_some() {
                    return Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd);
                }
                parse_identifier(value.ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete)?)?
            }
            frontend_contract::filter_operation::FilterOperation::Between => {
                let start = parse_identifier(value.ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete)?)?;
                let parsed_end = end
                    .ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete)?
                    .as_ref()
                    .parse::<i64>()
                    .ok()
                    .filter(|identifier| identifier.is_positive())
                    .map(serde_json::Value::from)
                    .ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidEnd)?;
                serde_json::json!({
                    (constants_str::PG_CRUD_START_FIELD): start,
                    (constants_str::PG_CRUD_END_FIELD): parsed_end
                })
            }
            frontend_contract::filter_operation::FilterOperation::In => {
                if end.is_some() {
                    return Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd);
                }
                serde_json::Value::Array(
                    value
                        .ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete)?
                        .as_ref()
                        .split(constants_str::TEXT_ALT_7)
                        .map(str::trim)
                        .map(|raw_value| {
                            raw_value
                                .parse::<i64>()
                                .ok()
                                .filter(|identifier| identifier.is_positive())
                                .map(serde_json::Value::from)
                                .ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue)
                        })
                        .collect::<Result<Vec<_>, crate::admin_identifier_filter_error::AdminIdentifierFilterError>>()?,
                )
            }
            frontend_contract::filter_operation::FilterOperation::AdjacentWithRange
            | frontend_contract::filter_operation::FilterOperation::Before
            | frontend_contract::filter_operation::FilterOperation::CurrentDate
            | frontend_contract::filter_operation::FilterOperation::CurrentTime
            | frontend_contract::filter_operation::FilterOperation::CurrentTimestamp
            | frontend_contract::filter_operation::FilterOperation::EqToEncodedStringRepresentation
            | frontend_contract::filter_operation::FilterOperation::ExcludedUpperBound
            | frontend_contract::filter_operation::FilterOperation::FindRangesThatFullyContainTheGivenRange
            | frontend_contract::filter_operation::FilterOperation::FindRangesWithinGivenRange
            | frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentDate
            | frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentTime
            | frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentTimestamp
            | frontend_contract::filter_operation::FilterOperation::GreaterThanExcludedUpperBound
            | frontend_contract::filter_operation::FilterOperation::GreaterThanIncludedLowerBound
            | frontend_contract::filter_operation::FilterOperation::IncludedLowerBound
            | frontend_contract::filter_operation::FilterOperation::OverlapWithRange
            | frontend_contract::filter_operation::FilterOperation::RangeLen
            | frontend_contract::filter_operation::FilterOperation::Regex
            | frontend_contract::filter_operation::FilterOperation::StrictlyToLeftOfRange
            | frontend_contract::filter_operation::FilterOperation::StrictlyToRightOfRange => {
                return Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation);
            }
            })
        };
        let boolean_values = || {
            if end.is_some() {
                return Err(
                    crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
                );
            }
            let parse_boolean =
                |admin_filter_value: &crate::admin_filter_value::AdminFilterValue| {
                    admin_filter_value
                        .as_ref()
                        .parse::<bool>()
                        .ok()
                        .map(serde_json::Value::from)
                        .ok_or(
                            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
                        )
                };
            let filter_value = value.ok_or(
                crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
            )?;
            match operation {
                frontend_contract::filter_operation::FilterOperation::Eq => {
                    parse_boolean(filter_value)
                }
                frontend_contract::filter_operation::FilterOperation::In => Ok(
                    serde_json::Value::Array(
                        filter_value
                            .as_ref()
                            .split(constants_str::TEXT_ALT_7)
                            .map(str::trim)
                            .map(str::to_owned)
                            .map(crate::admin_filter_value::AdminFilterValue::try_from)
                            .map(|result| {
                                let admin_filter_value = result
                                    .ok()
                                    .ok_or(
                                        crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
                                    )?;
                                parse_boolean(&admin_filter_value)
                            })
                            .collect::<Result<Vec<_>, crate::admin_identifier_filter_error::AdminIdentifierFilterError>>()?,
                    ),
                ),
                frontend_contract::filter_operation::FilterOperation::AdjacentWithRange
                | frontend_contract::filter_operation::FilterOperation::Before
                | frontend_contract::filter_operation::FilterOperation::Between
                | frontend_contract::filter_operation::FilterOperation::CurrentDate
                | frontend_contract::filter_operation::FilterOperation::CurrentTime
                | frontend_contract::filter_operation::FilterOperation::CurrentTimestamp
                | frontend_contract::filter_operation::FilterOperation::EqToEncodedStringRepresentation
                | frontend_contract::filter_operation::FilterOperation::ExcludedUpperBound
                | frontend_contract::filter_operation::FilterOperation::FindRangesThatFullyContainTheGivenRange
                | frontend_contract::filter_operation::FilterOperation::FindRangesWithinGivenRange
                | frontend_contract::filter_operation::FilterOperation::GreaterThan
                | frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentDate
                | frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentTime
                | frontend_contract::filter_operation::FilterOperation::GreaterThanCurrentTimestamp
                | frontend_contract::filter_operation::FilterOperation::GreaterThanExcludedUpperBound
                | frontend_contract::filter_operation::FilterOperation::GreaterThanIncludedLowerBound
                | frontend_contract::filter_operation::FilterOperation::IncludedLowerBound
                | frontend_contract::filter_operation::FilterOperation::OverlapWithRange
                | frontend_contract::filter_operation::FilterOperation::RangeLen
                | frontend_contract::filter_operation::FilterOperation::Regex
                | frontend_contract::filter_operation::FilterOperation::StrictlyToLeftOfRange
                | frontend_contract::filter_operation::FilterOperation::StrictlyToRightOfRange => Err(
                    crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation,
                ),
            }
        };
        let values = match input_kind {
            frontend_contract::input_kind::InputKind::Number
                if identifier_field.as_ref() == constants_str::SQL_NAMES_ID =>
            {
                identifier_values()?
            }
            frontend_contract::input_kind::InputKind::Text => {
                if end.is_some() {
                    return Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd);
                }
                let filter_value = value.ok_or(
                    crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
                )?;
                if operation == frontend_contract::filter_operation::FilterOperation::Eq {
                    serde_json::Value::String(filter_value.as_ref().to_owned())
                } else if operation == frontend_contract::filter_operation::FilterOperation::In {
                    serde_json::Value::Array(
                        filter_value
                            .as_ref()
                            .split(constants_str::TEXT_ALT_7)
                            .map(str::trim)
                            .map(str::to_owned)
                            .map(serde_json::Value::String)
                            .collect::<Vec<_>>(),
                    )
                } else {
                    return Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation);
                }
            }
            frontend_contract::input_kind::InputKind::Checkbox => boolean_values()?,
            frontend_contract::input_kind::InputKind::Date
            | frontend_contract::input_kind::InputKind::DateTime
            | frontend_contract::input_kind::InputKind::Number
            | frontend_contract::input_kind::InputKind::Time
            | frontend_contract::input_kind::InputKind::Uuid => return Err(
                crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedField,
            ),
        };
        let payload = serde_json::json!({
            (identifier_field.as_ref()): {
                (constants_str::PG_CRUD_OPERATOR_FIELD): constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND,
                (constants_str::PG_CRUD_VALUES_FIELD): [{
                    (format!("{operation:?}")): {
                        (constants_str::PG_CRUD_OPERATOR_FIELD): constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND,
                        (constants_str::PG_CRUD_VALUES_FIELD): values
                    }
                }]
            }
        });
        Self::try_from(payload.to_string()).map(Some).map_err(
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Representation,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_text_filter_rejects_unsupported_operation() {
        let query = crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            crate::admin_filter_field::AdminFilterField::try_from(constants_str::LOGIN.to_owned())
                .ok(),
            Some(frontend_contract::filter_operation::FilterOperation::Regex),
            crate::admin_filter_value::AdminFilterValue::try_from(constants_str::ADMIN.to_owned())
                .ok(),
            None,
        );

        assert!(matches!(
            super::AdminWhereMany::try_from_filter(
                &query,
                frontend_contract::input_kind::InputKind::Text,
            ),
            Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation)
        ));
    }

    #[test]
    fn test_text_filter_rejects_range_end() {
        let query = crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            crate::admin_filter_field::AdminFilterField::try_from(
                constants_str::DISPLAY_NAME.to_owned(),
            )
            .ok(),
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            crate::admin_filter_value::AdminFilterValue::try_from(constants_str::ADMIN.to_owned())
                .ok(),
            crate::admin_filter_value::AdminFilterValue::try_from(
                constants_str::ADMIN_ALT.to_owned(),
            )
            .ok(),
        );

        assert!(matches!(
            super::AdminWhereMany::try_from_filter(
                &query,
                frontend_contract::input_kind::InputKind::Text,
            ),
            Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd)
        ));
    }

    #[test]
    fn test_boolean_filter_builds_equality() {
        let query = crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            crate::admin_filter_field::AdminFilterField::try_from(
                constants_str::IS_BANNED.to_owned(),
            )
            .ok(),
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            crate::admin_filter_value::AdminFilterValue::try_from(constants_str::TRUE.to_owned())
                .ok(),
            None,
        );

        let result = super::AdminWhereMany::try_from_filter(
            &query,
            frontend_contract::input_kind::InputKind::Checkbox,
        )
        .and_then(|where_many| {
            where_many
                .ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete)
        });

        assert!(result.is_ok_and(|where_many| {
            serde_json::from_str::<serde_json::Value>(where_many.as_ref()).is_ok_and(|value| {
                value
                    .get(constants_str::IS_BANNED)
                    .and_then(|field| field.get(constants_str::PG_CRUD_VALUES_FIELD))
                    .and_then(|predicates| predicates.get(0usize))
                    .and_then(|predicate| predicate.get(stringify!(Eq)))
                    .and_then(|body| body.get(constants_str::PG_CRUD_VALUES_FIELD))
                    == Some(&serde_json::Value::Bool(true))
            })
        }));
    }

    #[test]
    fn test_boolean_filter_builds_membership() {
        let query = crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            crate::admin_filter_field::AdminFilterField::try_from(
                constants_str::IS_BANNED.to_owned(),
            )
            .ok(),
            Some(frontend_contract::filter_operation::FilterOperation::In),
            crate::admin_filter_value::AdminFilterValue::try_from(
                [constants_str::TRUE, constants_str::FALSE].join(constants_str::TEXT_ALT_7),
            )
            .ok(),
            None,
        );

        let result = super::AdminWhereMany::try_from_filter(
            &query,
            frontend_contract::input_kind::InputKind::Checkbox,
        )
        .and_then(|where_many| {
            where_many
                .ok_or(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete)
        });

        assert!(result.is_ok_and(|where_many| {
            serde_json::from_str::<serde_json::Value>(where_many.as_ref()).is_ok_and(|value| {
                value
                    .get(constants_str::IS_BANNED)
                    .and_then(|field| field.get(constants_str::PG_CRUD_VALUES_FIELD))
                    .and_then(|predicates| predicates.get(0usize))
                    .and_then(|predicate| predicate.get(stringify!(In)))
                    .and_then(|body| body.get(constants_str::PG_CRUD_VALUES_FIELD))
                    == Some(&serde_json::json!([true, false]))
            })
        }));
    }

    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository policy requires iterator operations instead of for loops"
    )]
    fn test_text_filter_builds_equality_and_membership_operations() {
        [
            (
                frontend_contract::filter_operation::FilterOperation::Eq,
                constants_str::ADMIN_ALT.to_owned(),
                serde_json::json!(constants_str::ADMIN_ALT),
            ),
            (
                frontend_contract::filter_operation::FilterOperation::In,
                [constants_str::ADMIN, constants_str::ADMIN_ALT].join(constants_str::TEXT_ALT_7),
                serde_json::json!([constants_str::ADMIN, constants_str::ADMIN_ALT]),
            ),
        ]
        .into_iter()
        .for_each(|(operation, raw_value, expected_values)| {
            let result = (|| {
                let field = crate::admin_filter_field::AdminFilterField::try_from(
                    constants_str::LOGIN.to_owned(),
                )
                .map_err(|error| error.to_string())?;
                let filter_value = crate::admin_filter_value::AdminFilterValue::try_from(raw_value)
                    .map_err(|error| error.to_string())?;
                let query = crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
                    Some(field),
                    Some(operation),
                    Some(filter_value),
                    None,
                );
                let where_many = super::AdminWhereMany::try_from_filter(
                    &query,
                    frontend_contract::input_kind::InputKind::Text,
                )
                .map_err(|error| error.to_string())?
                .ok_or_else(String::new)?;
                let parsed = serde_json::from_str::<serde_json::Value>(where_many.as_ref())
                    .map_err(|error| error.to_string())?;
                parsed
                    .get(constants_str::LOGIN)
                    .and_then(|entry| entry.get(constants_str::PG_CRUD_VALUES_FIELD))
                    .and_then(serde_json::Value::as_array)
                    .and_then(|predicates| predicates.first())
                    .and_then(|predicate| predicate.get(format!("{operation:?}")))
                    .and_then(|body| body.get(constants_str::PG_CRUD_VALUES_FIELD))
                    .cloned()
                    .ok_or_else(String::new)
            })();
            assert_eq!(result, Ok(expected_values));
        });
    }

    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository policy requires iterator operations instead of for loops"
    )]
    fn test_identifier_filter_builds_every_supported_operation() {
        [
            (
                frontend_contract::filter_operation::FilterOperation::Eq,
                stringify!(2),
                None,
                serde_json::json!(2i64),
            ),
            (
                frontend_contract::filter_operation::FilterOperation::GreaterThan,
                stringify!(2),
                None,
                serde_json::json!(2i64),
            ),
            (
                frontend_contract::filter_operation::FilterOperation::Between,
                stringify!(2),
                Some(stringify!(4)),
                serde_json::json!({
                    (constants_str::PG_CRUD_START_FIELD): 2i64,
                    (constants_str::PG_CRUD_END_FIELD): 4i64
                }),
            ),
            (
                frontend_contract::filter_operation::FilterOperation::In,
                concat!(stringify!(2), ",", stringify!(4)),
                None,
                serde_json::json!([2i64, 4i64]),
            ),
        ]
        .into_iter()
        .for_each(|(operation, raw_value, raw_end, expected_values)| {
            let result = (|| {
                let field = crate::admin_filter_field::AdminFilterField::try_from(
                    constants_str::SQL_NAMES_ID.to_owned(),
                )
                .map_err(|error| error.to_string())?;
                let filter_value =
                    crate::admin_filter_value::AdminFilterValue::try_from(raw_value.to_owned())
                        .map_err(|error| error.to_string())?;
                let filter_end = raw_end
                    .map(str::to_owned)
                    .map(crate::admin_filter_value::AdminFilterValue::try_from)
                    .transpose()
                    .map_err(|error| error.to_string())?;
                let query = crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
                    Some(field),
                    Some(operation),
                    Some(filter_value),
                    filter_end,
                );
                let where_many = super::AdminWhereMany::try_from_identifier_filter(&query)
                    .map_err(|error| error.to_string())?
                    .ok_or_else(String::new)?;
                let parsed = serde_json::from_str::<serde_json::Value>(where_many.as_ref())
                    .map_err(|error| error.to_string())?;
                let operation_name = format!("{operation:?}");
                parsed
                    .get(constants_str::SQL_NAMES_ID)
                    .and_then(|identifier_entry| {
                        identifier_entry.get(constants_str::PG_CRUD_VALUES_FIELD)
                    })
                    .and_then(serde_json::Value::as_array)
                    .and_then(|predicates| predicates.first())
                    .and_then(|predicate| predicate.get(operation_name))
                    .and_then(|body| body.get(constants_str::PG_CRUD_VALUES_FIELD))
                    .cloned()
                    .ok_or_else(String::new)
            })();
            assert_eq!(result, Ok(expected_values));
        });
    }

    #[test]
    fn test_where_many_round_trips_as_a_json_object() {
        let input = serde_json::json!({
            (constants_str::SQL_NAMES_ID): {
                (constants_str::PG_CRUD_OPERATOR_FIELD): constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND,
                (constants_str::PG_CRUD_VALUES_FIELD): [{
                    (stringify!(GreaterThan)): {
                        (constants_str::PG_CRUD_OPERATOR_FIELD): constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND,
                        (constants_str::PG_CRUD_VALUES_FIELD): 1i64
                    }
                }]
            }
        });
        let parsed = serde_json::from_value::<super::AdminWhereMany>(input.clone());
        assert!(
            parsed
                .is_ok_and(|value| serde_json::to_value(value).is_ok_and(|output| output == input))
        );
    }
}
