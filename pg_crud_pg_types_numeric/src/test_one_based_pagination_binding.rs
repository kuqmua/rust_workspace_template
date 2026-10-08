#[test]
fn test_one_based_pagination_binding_preserves_existing_argument_and_value_order() {
    let pagination_result =
        pg_types_common::pagination_starts_with_one::PaginationStartsWithOne::try_new(2i64, 3i64);
    assert!(pagination_result.is_ok());
    let Ok(pagination) = pagination_result else {
        return;
    };
    let initial = sqlx::query(stringify!()).bind(17i64);
    let mut expected_query = sqlx::query(stringify!()).bind(17i64).bind(2i64).bind(3i64);
    let expected_arguments = sqlx::Execute::take_arguments(&mut expected_query);
    let bound = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
        pagination,
        pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(initial),
    );
    assert!(bound.is_ok_and(|query| {
        let mut actual_query = query.into_inner();
        sqlx::Execute::take_arguments(&mut actual_query).is_ok_and(|arguments| {
            arguments.is_some_and(|values| {
                sqlx::Arguments::len(&values) == 3usize
                    && expected_arguments.as_ref().is_ok_and(|expected| {
                        expected.as_ref().is_some_and(|expected_values| {
                            format!("{values:?}") == format!("{expected_values:?}")
                        })
                    })
            })
        })
    }));
}
