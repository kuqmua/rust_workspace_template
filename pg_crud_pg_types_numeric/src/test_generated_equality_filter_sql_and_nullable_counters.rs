#[test]
fn test_generated_equality_filter_sql_and_nullable_counters() {
    let column = char::from(120u8).to_string();
    assert!([0u64, 4u64, u64::MAX].into_iter().all(|initial_index| {
        [pg_crud_common::operator::Operator::And, pg_crud_common::operator::Operator::AndNot, pg_crud_common::operator::Operator::Or, pg_crud_common::operator::Operator::OrNot].into_iter().all(|operator| {
            [false, true].into_iter().all(|add_operator| {
                let prefix = operator.to_query_part(pg_crud_common::add_operator::AddOperator::from(add_operator));
        let mut non_null_increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
        let mut some_increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
        let mut null_increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
        let cases = [
            (
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &where_filters::domain_types::PgTypeWhereEq::new(
                        operator,
                        crate::generate_pg_types_mod::I32AsNonNullInt4TableType::new(1i32),
                    ),
                    &mut non_null_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(add_operator),
                ),
                non_null_increment,
                false,
            ),
            (
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &where_filters::domain_types::PgTypeWhereEq::new(
                        operator,
                        crate::generate_pg_types_mod::OptionalI32AsNullableInt4TableType::new(Some(1i32)),
                    ),
                    &mut some_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(add_operator),
                ),
                some_increment,
                false,
            ),
            (
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &where_filters::domain_types::PgTypeWhereEq::new(
                        operator,
                        crate::generate_pg_types_mod::OptionalI32AsNullableInt4TableType::new(None),
                    ),
                    &mut null_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(add_operator),
                ),
                null_increment,
                true,
            ),
        ];
        cases.into_iter().all(|(result, increment, is_null)| {
            if is_null {
                increment.get() == initial_index && result.is_ok_and(|fragment| {
                    fragment.as_ref().strip_prefix(prefix.as_ref()).and_then(|text| text.strip_prefix('(')).and_then(|text| text.strip_prefix(column.as_str()))
                        .and_then(|text| text.strip_suffix(')')).is_some_and(|text| text.trim_start() == pg_crud_common::eq_operator::EqOperator::IsNull.to_query_str().as_ref())
                })
            } else {
                match initial_index.checked_add(1u64) {
                    Some(expected_index) => increment.get() == expected_index && result.is_ok_and(|fragment| {
                        let Some(text) = fragment.as_ref().strip_prefix(prefix.as_ref()) else { return false; };
                        let mut parts = text.split_whitespace();
                        parts.next().and_then(|part| part.strip_prefix('(')) == Some(column.as_str())
                            && parts.next() == Some(pg_crud_common::eq_operator::EqOperator::Eq.to_query_str().as_ref())
                            && parts.next().and_then(|part| part.strip_suffix(')')).and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == Some(expected_index)
                            && parts.next().is_none()
                    }),
                    None => increment.get() == initial_index && matches!(result, Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })),
                }
            }
        })
            })
        })
    }));
}
