#[test]
fn test_pagination_reports_start_and_end() {
    let pagination =
        crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(20i32, 5i32)
            .expect(constants_str::DIAGNOSTIC_5E74C1A9);
    assert_eq!(pagination.start().get(), 5i64);
    assert_eq!(pagination.end().get(), 25i64);
}

#[test]
fn test_pagination_rejects_invalid_bounds() {
    assert!(matches!(
        crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(
            constants_i32::ZERO,
            constants_i32::ZERO,
        ),
        Err(
            crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero { .. }
        )
    ));
    assert!(matches!(
        crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(1i32, -1i32),
        Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero { .. })
    ));
}

#[test]
fn test_pagination_accepts_exact_integer_sum_boundary_and_json_round_trip() {
    assert!([(i64::MAX, 0i64), (1i64, i64::MAX - 1)]
        .into_iter()
        .all(|(limit, offset)| {
            let pagination = crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(
                crate::pagination_limit::PaginationLimit::from(limit),
                crate::pagination_offset::PaginationOffset::from(offset),
            );
            matches!(pagination, Ok(value) if {
                let json = serde_json::json!({(stringify!(limit)): limit, (stringify!(offset)): offset});
                value.start().get() == offset
                    && value.end().get() == i64::MAX
                    && matches!(serde_json::to_value(value), Ok(serialized) if serialized == json)
                    && matches!(serde_json::from_value::<crate::pagination_starts_with_zero::PaginationStartsWithZero>(json), Ok(decoded) if decoded == value)
            })
        }));
}

#[test]
fn test_pagination_error_priority_and_json_preserve_integer_validation() {
    assert!([
        (0i64, -1i64),
        (i64::MIN, -1i64),
        (1i64, i64::MIN),
        (i64::MAX, 1i64),
        (1i64, i64::MAX),
    ]
    .into_iter()
    .all(|(limit, offset)| {
        let pagination_limit = crate::pagination_limit::PaginationLimit::from(limit);
        let pagination_offset = crate::pagination_offset::PaginationOffset::from(offset);
        let error_matches = match crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(
            pagination_limit,
            pagination_offset,
        ) {
            Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero { limit: rejected_limit, .. }) => {
                limit <= 0 && rejected_limit == pagination_limit
            }
            Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero { offset: rejected_offset, .. }) => {
                limit > 0 && offset < 0 && rejected_offset == pagination_offset
            }
            Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetPlusLimitIsIntOverflow { limit: rejected_limit, offset: rejected_offset, .. }) => {
                limit > 0 && offset >= 0 && rejected_limit == pagination_limit && rejected_offset == pagination_offset
            }
            Ok(_) => false,
        };
        error_matches && serde_json::from_value::<crate::pagination_starts_with_zero::PaginationStartsWithZero>(
            serde_json::json!({(stringify!(limit)): limit, (stringify!(offset)): offset}),
        ).is_err()
    }));
}

#[test]
fn test_pagination_defaults_preserve_policy_limit_and_maximum_page_size() {
    let standard = <crate::pagination_starts_with_zero::PaginationStartsWithZero as crate::default_some_one_element::DefaultSomeOneElement>::default_some_one_element();
    assert_eq!(
        standard,
        crate::pagination_starts_with_zero::PaginationStartsWithZero::default()
    );
    assert_eq!(standard.start().get(), 0i64);
    assert_eq!(standard.end().get(), 5i64);
    let maximum = <crate::pagination_starts_with_zero::PaginationStartsWithZero as crate::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize>::default_some_one_element_max_page_size();
    assert_eq!(maximum.start().get(), 0i64);
    assert_eq!(maximum.end().get(), i64::from(i32::MAX));
    assert!([standard, maximum].into_iter().all(|pagination| {
        serde_json::to_value(pagination).is_ok_and(|json| {
            json == serde_json::json!({(stringify!(limit)): pagination.end().get(), (stringify!(offset)): 0i64})
        })
    }));
}

#[test]
fn test_validated_pagination_query_preserves_placeholder_progress_and_overflow() {
    let pagination = crate::pagination_starts_with_zero::PaginationStartsWithZero::default();
    assert!([false, true].into_iter().all(|operator| {
        [0u64, 7u64, u64::MAX - 2u64, u64::MAX - 1u64, u64::MAX]
            .into_iter()
            .all(|initial| {
                let mut increment = crate::query_part_increment::QueryPartIncrement::from(initial);
                let result = crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &pagination,
                    &mut increment,
                    crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
                    crate::add_operator::AddOperator::from(operator),
                );
                if let Some(expected) = initial.checked_add(2u64) {
                    result.is_ok_and(|part| {
                        part.as_ref()
                            == format!(
                                "{} ${} {} ${expected}",
                                constants_str::LIMIT,
                                initial + 1u64,
                                constants_str::OFFSET_ALT
                            )
                    }) && increment.get() == expected
                } else {
                    matches!(
                        result,
                        Err(crate::query_part_error::QueryPartError::CheckedAdd { .. })
                    ) && increment.get() == u64::MAX
                }
            })
    }));
}

#[test]
fn test_validated_pagination_binding_preserves_existing_query_arguments() {
    assert!([false, true].into_iter().all(|has_existing_argument| {
        let initial = sqlx::query(constants_str::EMPTY);
        let query = if has_existing_argument {
            initial.bind(17i64)
        } else {
            initial
        };
        let pagination = crate::pagination_starts_with_zero::PaginationStartsWithZero::default();
        crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(
            pagination,
            crate::sqlx_postgres_query::SqlxPostgresQuery::from(query),
        )
        .is_ok_and(|bound| {
            let mut sqlx_query = bound.into_inner();
            sqlx::Execute::take_arguments(&mut sqlx_query).is_ok_and(|arguments| {
                arguments.is_some_and(|values| {
                    sqlx::Arguments::len(&values)
                        == if has_existing_argument {
                            3usize
                        } else {
                            2usize
                        }
                })
            })
        })
    }));
}

#[test]
fn test_pagination_leaf_conversions_preserve_full_signed_integer_range() {
    assert_eq!(
        crate::pagination_limit::PaginationLimit::default().get(),
        0i64
    );
    assert_eq!(
        crate::pagination_offset::PaginationOffset::default().get(),
        0i64
    );
    assert!(
        [i32::MIN, -1i32, 0i32, 1i32, i32::MAX]
            .into_iter()
            .all(|value| {
                let expanded = i64::from(value);
                crate::pagination_limit::PaginationLimit::from(value).get() == expanded
                    && crate::pagination_offset::PaginationOffset::from(value).get() == expanded
            })
    );
    assert!(
        [
            i64::MIN,
            i64::from(i32::MIN),
            -1i64,
            0i64,
            1i64,
            i64::from(i32::MAX),
            i64::MAX
        ]
        .into_iter()
        .all(|value| {
            let limit = crate::pagination_limit::PaginationLimit::from(value);
            let offset = crate::pagination_offset::PaginationOffset::from(value);
            let wire_value = serde_json::json!(value);
            limit.get() == value
                && offset.get() == value
                && limit.to_string() == value.to_string()
                && offset.to_string() == value.to_string()
                && serde_json::to_value(limit).is_ok_and(|serialized| serialized == wire_value)
                && serde_json::to_value(offset).is_ok_and(|serialized| serialized == wire_value)
                && serde_json::from_value::<crate::pagination_limit::PaginationLimit>(
                    wire_value.clone(),
                )
                .is_ok_and(|decoded| decoded == limit)
                && serde_json::from_value::<crate::pagination_offset::PaginationOffset>(wire_value)
                    .is_ok_and(|decoded| decoded == offset)
        })
    );
    assert!(
        [
            serde_json::Value::Null,
            serde_json::json!(false),
            serde_json::json!(1.5f64),
            serde_json::json!(constants_str::X),
            serde_json::json!({}),
            serde_json::json!([]),
        ]
        .into_iter()
        .all(|wire_value| {
            serde_json::from_value::<crate::pagination_limit::PaginationLimit>(wire_value.clone())
                .is_err_and(|error| error.is_data())
                && serde_json::from_value::<crate::pagination_offset::PaginationOffset>(wire_value)
                    .is_err_and(|error| error.is_data())
        })
    );
}
