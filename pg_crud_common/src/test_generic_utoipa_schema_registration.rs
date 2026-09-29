fn assert_item_schema_registered<Wrapper, Item>()
where
    Wrapper: utoipa::ToSchema,
    Item: utoipa::ToSchema,
{
    let mut schemas = Vec::new();
    Wrapper::schemas(&mut schemas);
    assert!(
        schemas
            .iter()
            .any(|(name, _schema)| name == Item::name().as_ref())
    );
}

#[test]
fn test_explicit_value_registers_item_schema() {
    assert_item_schema_registered::<
        crate::explicit_value::ExplicitValue<crate::pagination_base::PaginationBase>,
        crate::pagination_base::PaginationBase,
    >();
}

#[test]
fn test_not_empty_unique_vec_registers_item_schema() {
    assert_item_schema_registered::<
        crate::not_empty_unique_vec::NotEmptyUniqueVec<crate::pagination_base::PaginationBase>,
        crate::pagination_base::PaginationBase,
    >();
}

#[test]
fn test_order_by_registers_column_schema() {
    assert_item_schema_registered::<
        crate::order_by::OrderBy<crate::pagination_base::PaginationBase>,
        crate::pagination_base::PaginationBase,
    >();
    assert_item_schema_registered::<
        crate::order_by::OrderBy<crate::pagination_base::PaginationBase>,
        crate::order::Order,
    >();
}

#[test]
fn test_pg_type_where_registers_item_schema() {
    assert_item_schema_registered::<
        crate::pg_type_where::PgTypeWhere<crate::pagination_base::PaginationBase>,
        crate::pagination_base::PaginationBase,
    >();
    assert_item_schema_registered::<
        crate::pg_type_where::PgTypeWhere<crate::pagination_base::PaginationBase>,
        crate::not_empty_unique_vec::NotEmptyUniqueVec<crate::pagination_base::PaginationBase>,
    >();
    assert_item_schema_registered::<
        crate::pg_type_where::PgTypeWhere<crate::pagination_base::PaginationBase>,
        crate::operator::Operator,
    >();
}
