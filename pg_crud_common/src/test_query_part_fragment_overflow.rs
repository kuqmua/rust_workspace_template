#[derive(Debug, Clone, PartialEq, Eq, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement);

impl<'query_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt>
    for TestOversizedWhereValue
{
    fn query_bind(
        self,
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        Ok(sqlx_postgres_query)
    }

    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    > {
        let _: (
            &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
            crate::sql_column_ref::SqlColumnRef<'_>,
            crate::add_operator::AddOperator,
        ) = (increment, sql_column_ref, add_operator);
        Ok(crate::query_part_fragment::QueryPartFragment::try_from(
            constants_str::X.repeat(600_000usize),
        )?)
    }
}

impl crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement
    for TestOversizedWhereValue
{
    fn all_variants_default_some_one_element() -> crate::all_enum_variants::AllEnumVariants<Self> {
        vec![Self(crate::query_part_increment::QueryPartIncrement::from(1u64))].into()
    }
}

#[test]
fn test_pg_type_where_rejects_combined_fragment_overflow() {
    let filter_result = crate::pg_type_where::PgTypeWhere::try_new(
        crate::operator::Operator::Or,
        crate::duplicate_candidates::DuplicateCandidates::from(vec![
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(1u64)),
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(2u64)),
        ]),
    );
    assert!(filter_result.is_ok());
    if let Ok(filter) = filter_result {
        let mut increment = crate::query_part_increment::QueryPartIncrement::from(0u64);
        assert!(matches!(
            crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                &filter,
                &mut increment,
                crate::sql_column_ref::SqlColumnRef::from(&constants_str::X),
                crate::add_operator::AddOperator::from(false),
            ),
            Err(crate::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
        ));
    }
}

#[test]
fn test_nullable_json_obj_filter_rejects_fragment_overflow() {
    let filter = crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::<
        TestOversizedWhereValue,
    >::from(None);
    let column = constants_str::X
        .repeat(crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN);
    let mut increment = crate::query_part_increment::QueryPartIncrement::from(0u64);
    assert!(matches!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &filter,
            &mut increment,
            crate::sql_column_ref::SqlColumnRef::from(&column),
            crate::add_operator::AddOperator::from(false),
        ),
        Err(crate::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
}

#[test]
fn test_query_part_error_formats_as_error_text() {
    let error = crate::query_part_error::QueryPartError::CheckedAdd {
        location: proc_macro_location_bang::location!(),
    };
    let error_text = to_err_string::to_err_string::ToErrString::to_err_string(&error);
    assert_eq!(error_text.as_ref(), error.to_string());
}

#[test]
fn test_nullable_json_obj_null_filter_preserves_counter_and_existing_arguments() {
    let filter = crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::<
        TestOversizedWhereValue,
    >::from(None);
    assert!([false, true].into_iter().all(|operator| {
        let mut increment = crate::query_part_increment::QueryPartIncrement::from(u64::MAX);
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &filter,
            &mut increment,
            crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
            crate::add_operator::AddOperator::from(operator),
        )
        .is_ok_and(|part| {
            part.to_string() == format!("{} = '{}'", constants_str::SQL_NAMES_ID, stringify!(null))
                && increment.get() == u64::MAX
        })
    }));
    assert!(filter.as_ref().is_none());
    let query = crate::sqlx_postgres_query::SqlxPostgresQuery::from(
        sqlx::query(constants_str::EMPTY).bind(17i64),
    );
    assert!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(filter, query).is_ok_and(
            |bound| {
                let mut sqlx_query = bound.into_inner();
                sqlx::Execute::take_arguments(&mut sqlx_query).is_ok_and(|arguments| {
                    arguments.is_some_and(|bound_arguments| {
                        sqlx::Arguments::len(&bound_arguments) == 1usize
                    })
                })
            }
        )
    );
}

#[test]
fn test_nullable_json_obj_non_null_filter_preserves_values_and_query_errors() {
    let Ok(values) = crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(
        crate::duplicate_candidates::DuplicateCandidates::from(vec![
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(1u64)),
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(2u64)),
        ]),
    ) else {
        std::panic::panic_any(constants_str::PANIC_EDC94D17);
    };
    let filter =
        crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::from(
            Some(values.clone()),
        );
    assert_eq!(filter.as_ref(), Some(&values));
    let mut increment = crate::query_part_increment::QueryPartIncrement::from(7u64);
    assert!(matches!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &filter,
            &mut increment,
            crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
            crate::add_operator::AddOperator::from(false)
        ),
        Err(crate::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
    assert_eq!(increment.get(), 7u64);
    assert_eq!(filter.into_option(), Some(values));
}

#[test]
fn test_nullable_json_obj_non_null_filter_delegates_argument_binding() {
    let values: crate::not_empty_unique_vec::NotEmptyUniqueVec<TestOversizedWhereValue> =
        crate::default_some_one_element::DefaultSomeOneElement::default_some_one_element();
    let filter =
        crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::from(
            Some(values),
        );
    let query = crate::sqlx_postgres_query::SqlxPostgresQuery::from(
        sqlx::query(constants_str::EMPTY).bind(17i64),
    );
    assert!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(filter, query).is_ok_and(
            |bound| {
                let mut sqlx_query = bound.into_inner();
                sqlx::Execute::take_arguments(&mut sqlx_query).is_ok_and(|arguments| {
                    arguments.is_some_and(|bound_arguments| {
                        sqlx::Arguments::len(&bound_arguments) == 1usize
                    })
                })
            }
        )
    );
}
