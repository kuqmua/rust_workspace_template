fn assert_names_are_distinct<First, Second>()
where
    First: utoipa::ToSchema,
    Second: utoipa::ToSchema,
{
    assert_ne!(First::name(), Second::name());
}

#[test]
fn test_explicit_value_schema_names_distinguish_item_types() {
    assert_names_are_distinct::<
        crate::explicit_value::ExplicitValue<crate::pagination_base::PaginationBase>,
        crate::explicit_value::ExplicitValue<crate::operator::Operator>,
    >();
}

#[test]
fn test_not_empty_unique_vec_schema_names_distinguish_item_types() {
    assert_names_are_distinct::<
        crate::not_empty_unique_vec::NotEmptyUniqueVec<crate::pagination_base::PaginationBase>,
        crate::not_empty_unique_vec::NotEmptyUniqueVec<crate::operator::Operator>,
    >();
}

#[test]
fn test_order_by_schema_names_distinguish_column_types() {
    assert_names_are_distinct::<
        crate::order_by::OrderBy<crate::pagination_base::PaginationBase>,
        crate::order_by::OrderBy<crate::operator::Operator>,
    >();
}

#[test]
fn test_pg_type_where_schema_names_distinguish_item_types() {
    assert_names_are_distinct::<
        crate::pg_type_where::PgTypeWhere<crate::pagination_base::PaginationBase>,
        crate::pg_type_where::PgTypeWhere<crate::operator::Operator>,
    >();
}

#[test]
fn test_single_or_multiple_schema_names_distinguish_item_types() {
    assert_names_are_distinct::<
        crate::single_or_multiple::SingleOrMultiple<crate::pagination_base::PaginationBase>,
        crate::single_or_multiple::SingleOrMultiple<crate::operator::Operator>,
    >();
}
