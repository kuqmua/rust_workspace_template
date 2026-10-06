#[test]
fn test_pagination_base_ranges_preserve_unchecked_values_and_saturate() {
    assert!(
        [
            (5i64, 7i64, 12i64),
            (1i64, i64::MAX, i64::MAX),
            (-1i64, i64::MIN, i64::MIN)
        ]
        .into_iter()
        .all(|(limit, offset, end)| {
            let pagination = crate::pagination_base::PaginationBase::new_unchecked(
                crate::pagination_limit::PaginationLimit::from(limit),
                crate::pagination_offset::PaginationOffset::from(offset),
            );
            pagination.start().get() == offset && pagination.end().get() == end
        })
    );
}

#[test]
fn test_pagination_base_query_numbers_limit_and_offset_and_preserves_overflow_progress() {
    let pagination = crate::pagination_base::PaginationBase::default();
    assert!([false, true].into_iter().all(|operator| {
        let mut increment = crate::query_part_increment::QueryPartIncrement::from(7u64);
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &pagination,
            &mut increment,
            crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
            crate::add_operator::AddOperator::from(operator),
        )
        .is_ok_and(|part| {
            part.to_string()
                == format!(
                    "{} $8 {} $9",
                    constants_str::LIMIT,
                    constants_str::OFFSET_ALT
                )
                && increment.get() == 9u64
        })
    }));
    assert!(
        [(u64::MAX, u64::MAX), (u64::MAX - 1u64, u64::MAX)]
            .into_iter()
            .all(|(initial, expected)| {
                let mut increment = crate::query_part_increment::QueryPartIncrement::from(initial);
                matches!(
                    crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                        &pagination,
                        &mut increment,
                        crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
                        crate::add_operator::AddOperator::from(false)
                    ),
                    Err(crate::query_part_error::QueryPartError::CheckedAdd { .. })
                ) && increment.get() == expected
            })
    );
}

#[test]
fn test_pagination_base_binding_appends_two_arguments_to_existing_query() {
    assert!([false, true].into_iter().all(|has_existing_argument| {
        let initial = sqlx::query(constants_str::EMPTY);
        let query = if has_existing_argument {
            initial.bind(17i64)
        } else {
            initial
        };
        let expected_initial = sqlx::query(constants_str::EMPTY);
        let expected_base = if has_existing_argument {
            expected_initial.bind(17i64)
        } else {
            expected_initial
        };
        let mut expected_query = expected_base.bind(i64::MAX).bind(i64::MIN);
        let expected_arguments_result = sqlx::Execute::take_arguments(&mut expected_query);
        let pagination = crate::pagination_base::PaginationBase::new_unchecked(
            crate::pagination_limit::PaginationLimit::from(i64::MAX),
            crate::pagination_offset::PaginationOffset::from(i64::MIN),
        );
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
                        && expected_arguments_result.as_ref().is_ok_and(
                            |expected_arguments_option| {
                                expected_arguments_option
                                    .as_ref()
                                    .is_some_and(|expected_values| {
                                        format!("{values:?}") == format!("{expected_values:?}")
                                    })
                            },
                        )
                })
            })
        })
    }));
}
