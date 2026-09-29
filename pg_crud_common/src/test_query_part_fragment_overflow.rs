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
