#[test]
fn test_between_query_uses_consecutive_placeholders() {
    assert!(
        crate::between::Between::try_new(1i32, 2i32).is_ok_and(|between| {
            let mut increment =
                pg_crud_common::query_part_increment::QueryPartIncrement::from(4u64);
            let column = constants_str::TEST_DB_COLUMN_ID;
            let result = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                &between,
                &mut increment,
                pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                true.into(),
            );
            assert!(result.is_ok_and(|fragment| fragment.as_ref()
                == format!(
                    "{} ${}{}${}",
                    constants_str::ADMIN_FILTER_OPERATION_BETWEEN,
                    5u64,
                    constants_str::AND,
                    6u64
                )));
            assert_eq!(increment.get(), 6u64);
            true
        })
    );
}

#[test]
fn test_between_query_overflow_keeps_successful_counter_increment() {
    assert!(
        crate::between::Between::try_new(1i32, 2i32).is_ok_and(|between| {
            let mut increment =
                pg_crud_common::query_part_increment::QueryPartIncrement::from(u64::MAX - 1u64);
            let column = constants_str::TEST_DB_COLUMN_ID;
            let result = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                &between,
                &mut increment,
                pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                false.into(),
            );
            assert!(matches!(
                result,
                Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })
            ));
            assert_eq!(increment.get(), u64::MAX);
            let repeated = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                &between,
                &mut increment,
                pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                false.into(),
            );
            assert!(matches!(
                repeated,
                Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })
            ));
            assert_eq!(increment.get(), u64::MAX);
            true
        })
    );
}
