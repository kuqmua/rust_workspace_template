#[test]
fn test_pg_type_where_serializes_and_deserializes_with_full_field_name() {
    let filter = crate::pg_type_where::PgTypeWhere::try_new(
        crate::operator::Operator::default(),
        crate::duplicate_candidates::DuplicateCandidates::from(vec![7u8]),
    )
    .expect(constants_str::DIAGNOSTIC_F465D1AC);
    let serialized = serde_json::to_value(&filter).expect(constants_str::DIAGNOSTIC_529A2FFD);
    assert!(serialized.get(stringify!(values)).is_some());
    assert!(serialized.get(stringify!(v)).is_none());
    assert_eq!(
        serde_json::from_value::<crate::pg_type_where::PgTypeWhere<u8>>(serialized)
            .expect(constants_str::DIAGNOSTIC_28EC5ACA),
        filter
    );
}

#[test]
fn test_pg_type_where_sequence_deserialization_preserves_operator_and_value_order() {
    assert!(
        [
            crate::operator::Operator::And,
            crate::operator::Operator::AndNot,
            crate::operator::Operator::Or,
            crate::operator::Operator::OrNot,
        ]
        .into_iter()
        .all(|operator| {
            serde_json::to_value(operator).is_ok_and(|operator_json| {
                let sequence = serde_json::Value::Array(vec![
                    operator_json,
                    serde_json::Value::Array(vec![7u8.into(), 3u8.into()]),
                ]);
                serde_json::from_value::<crate::pg_type_where::PgTypeWhere<u8>>(sequence).is_ok_and(
                    |filter| {
                        filter.operator() == &operator && filter.values().as_slice() == [7u8, 3u8]
                    },
                )
            })
        })
    );
}

#[test]
fn test_pg_type_where_sequence_deserialization_rejects_invalid_shapes_and_values() {
    assert!(
        serde_json::to_value(crate::operator::Operator::And).is_ok_and(|operator_json| {
            [
                vec![],
                vec![operator_json.clone()],
                vec![operator_json.clone(), serde_json::Value::Array(vec![])],
                vec![
                    operator_json.clone(),
                    serde_json::Value::Array(vec![7u8.into(), 7u8.into()]),
                ],
                vec![operator_json.clone(), 7u8.into()],
                vec![7u8.into(), serde_json::Value::Array(vec![3u8.into()])],
                vec![
                    operator_json,
                    serde_json::Value::Array(vec![7u8.into()]),
                    3u8.into(),
                ],
            ]
            .into_iter()
            .all(|values| {
                matches!(
                    serde_json::from_value::<crate::pg_type_where::PgTypeWhere<u8>>(
                        serde_json::Value::Array(values)
                    ),
                    Err(error) if error.is_data()
                )
            })
        })
    );
}

#[test]
fn test_pg_type_where_object_deserialization_validates_fields_and_ignores_unknown_fields() {
    assert!(serde_json::to_value(crate::operator::Operator::And).is_ok_and(|operator_json| {
        let valid_fields = serde_json::Map::from_iter([
            (constants_str::PG_CRUD_OPERATOR_FIELD.to_owned(), operator_json),
            (constants_str::PG_CRUD_VALUES_FIELD.to_owned(), serde_json::Value::Array(vec![7u8.into(), 3u8.into()])),
        ]);
        let missing_fields_rejected = [constants_str::PG_CRUD_OPERATOR_FIELD, constants_str::PG_CRUD_VALUES_FIELD]
            .into_iter()
            .all(|field| {
                let mut fields = valid_fields.clone();
                assert!(fields.remove(field).is_some());
                matches!(serde_json::from_value::<crate::pg_type_where::PgTypeWhere<u8>>(serde_json::Value::Object(fields)), Err(error) if error.is_data())
            });
        let invalid_values_rejected = [vec![], vec![7u8.into(), 7u8.into()]]
            .into_iter()
            .all(|values| {
                let mut fields = valid_fields.clone();
                assert!(fields.insert(constants_str::PG_CRUD_VALUES_FIELD.to_owned(), serde_json::Value::Array(values)).is_some());
                matches!(serde_json::from_value::<crate::pg_type_where::PgTypeWhere<u8>>(serde_json::Value::Object(fields)), Err(error) if error.is_data())
            });
        let mut fields_with_unknown = valid_fields;
        assert!(fields_with_unknown.insert(constants_str::X.to_owned(), serde_json::Value::Array(vec![serde_json::Value::Null])).is_none());
        missing_fields_rejected && invalid_values_rejected
            && serde_json::from_value::<crate::pg_type_where::PgTypeWhere<u8>>(serde_json::Value::Object(fields_with_unknown))
                .is_ok_and(|filter| filter.operator() == &crate::operator::Operator::And && filter.values().as_slice() == [7u8, 3u8])
    }));
}

#[test]
fn test_pg_type_where_object_deserialization_rejects_duplicate_known_fields() {
    assert!(serde_json::to_string(&crate::operator::Operator::And).is_ok_and(|operator_json| {
        [
            format!(r#"{{"{0}":{1},"{0}":{1},"{2}":[7]}}"#, constants_str::PG_CRUD_OPERATOR_FIELD, operator_json, constants_str::PG_CRUD_VALUES_FIELD),
            format!(r#"{{"{0}":{1},"{2}":[7],"{2}":[3]}}"#, constants_str::PG_CRUD_OPERATOR_FIELD, operator_json, constants_str::PG_CRUD_VALUES_FIELD),
        ]
        .into_iter()
        .all(|json| matches!(serde_json::from_str::<crate::pg_type_where::PgTypeWhere<u8>>(&json), Err(error) if error.is_data()))
    }));
}

#[test]
fn test_pg_type_where_schema_preserves_required_operator_and_validated_values() {
    let schema = <crate::pg_type_where::PgTypeWhere<crate::operator::Operator> as utoipa::PartialSchema>::schema();
    assert!(matches!(
        &schema,
        utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(_))
    ));
    if let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema {
        assert_eq!(object.properties.len(), 2usize);
        assert!(object.required.iter().map(String::as_str).eq([
            constants_str::PG_CRUD_VALUES_FIELD,
            constants_str::PG_CRUD_OPERATOR_FIELD,
        ]));
        assert!(
            object.properties.get(constants_str::PG_CRUD_VALUES_FIELD)
                == Some(&<crate::not_empty_unique_vec::NotEmptyUniqueVec<
                    crate::operator::Operator,
                > as utoipa::PartialSchema>::schema())
        );
        assert!(
            object.properties.get(constants_str::PG_CRUD_OPERATOR_FIELD)
                == Some(&<crate::operator::Operator as utoipa::PartialSchema>::schema())
        );
    }
}

#[test]
fn test_pg_type_where_nested_queries_preserve_grouping_numbering_and_bound_arguments() {
    assert!([false, true].into_iter().all(|add_operator| {
        [
            (
                crate::operator::Operator::And,
                constants_str::EMPTY,
                constants_str::AND_ALT,
            ),
            (
                crate::operator::Operator::AndNot,
                constants_str::NOT,
                constants_str::AND_NOT,
            ),
            (
                crate::operator::Operator::Or,
                constants_str::EMPTY,
                constants_str::OR,
            ),
            (
                crate::operator::Operator::OrNot,
                constants_str::NOT,
                constants_str::OR_NOT,
            ),
        ]
        .into_iter()
        .all(|(parent_operator, standalone, joined)| {
            let build_child = |operator: crate::operator::Operator| {
                crate::pg_type_where::PgTypeWhere::try_new(
                    operator,
                    crate::duplicate_candidates::DuplicateCandidates::from(vec![
                        crate::pagination_base::PaginationBase::default(),
                    ]),
                )
            };
            let first = build_child(crate::operator::Operator::Or);
            let second = build_child(crate::operator::Operator::AndNot);
            match (first, second) {
                (Ok(first_filter), Ok(second_filter)) => {
                    crate::pg_type_where::PgTypeWhere::try_new(
                        parent_operator,
                        crate::duplicate_candidates::DuplicateCandidates::from(vec![
                            first_filter,
                            second_filter,
                        ]),
                    )
                    .is_ok_and(|filter| {
                        let mut increment =
                            crate::query_part_increment::QueryPartIncrement::from(7u64);
                        let rendered = crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                            &filter,
                            &mut increment,
                            crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
                            crate::add_operator::AddOperator::from(add_operator),
                        )
                        .is_ok_and(|fragment| {
                            fragment.as_ref()
                                == format!(
                                    "{}(({} $8 {} $9) {}({} $10 {} $11))",
                                    if add_operator { joined } else { standalone },
                                    constants_str::LIMIT,
                                    constants_str::OFFSET_ALT,
                                    constants_str::AND_NOT,
                                    constants_str::LIMIT,
                                    constants_str::OFFSET_ALT,
                                )
                        });
                        let query = sqlx::query(constants_str::EMPTY).bind(17i64);
                        let bound = crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                            filter,
                            crate::sqlx_postgres_query::SqlxPostgresQuery::from(query),
                        )
                        .is_ok_and(|bound_query| {
                            let mut sqlx_query = bound_query.into_inner();
                            sqlx::Execute::take_arguments(&mut sqlx_query).is_ok_and(|arguments| {
                                arguments
                                    .is_some_and(|values| sqlx::Arguments::len(&values) == 5usize)
                            })
                        });
                        rendered && increment.get() == 11u64 && bound
                    })
                }
                _ => false,
            }
        })
    }));
}

#[test]
fn test_pg_type_where_stops_after_child_query_overflow_and_preserves_increment_progress() {
    let filter = crate::pg_type_where::PgTypeWhere::try_new(
        crate::operator::Operator::And,
        crate::duplicate_candidates::DuplicateCandidates::from(vec![
            crate::pagination_base::PaginationBase::default(),
            crate::pagination_base::PaginationBase::new_unchecked(
                crate::pagination_limit::PaginationLimit::from(1i64),
                crate::pagination_offset::PaginationOffset::from(1i64),
            ),
        ]),
    );
    assert!(filter.is_ok_and(|validated| {
        [u64::MAX, u64::MAX - 1u64, u64::MAX - 2u64, u64::MAX - 3u64]
            .into_iter()
            .all(|initial| {
                let mut increment = crate::query_part_increment::QueryPartIncrement::from(initial);
                matches!(
                    crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                        &validated,
                        &mut increment,
                        crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
                        crate::add_operator::AddOperator::from(false),
                    ),
                    Err(crate::query_part_error::QueryPartError::CheckedAdd { .. })
                ) && increment.get() == u64::MAX
            })
    }));
}
