#[test]
fn test_generated_serial_read_rejects_query_part_overflow() {
    let value =
        crate::generate_pg_types_mod::I16AsNonNullSmallSerialInitializationByPgRead::new(1i16);
    let column = char::from(120u8).to_string().repeat(1_048_576usize);
    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(0u64);
    assert!(matches!(
        pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &value,
            &mut increment,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(false),
        ),
        Err(pg_crud_common::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
}

#[test]
fn test_generated_int2_select_rejects_query_part_overflow() {
    let column = char::from(120u8).to_string().repeat(1_048_577usize);
    assert!(matches!(
        <crate::generate_pg_types_mod::I16AsNonNullInt2 as pg_crud_common::pg_type::PgType>::select_query_part(
            &crate::generate_pg_types_mod::I16AsNonNullInt2Select,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
        ),
        Err(pg_crud_common::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
}

#[test]
fn test_generated_int2_select_only_ids_rejects_query_part_overflow() {
    let column = char::from(120u8).to_string().repeat(1_048_576usize);
    assert!(matches!(
        <crate::generate_pg_types_mod::I16AsNonNullInt2 as pg_crud_common::pg_type::PgType>::select_only_ids_query_part(
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
        ),
        Err(pg_crud_common::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
}

#[test]
fn test_generated_int2_create_table_column_rejects_query_part_overflow() {
    let column = char::from(120u8).to_string().repeat(1_048_576usize);
    assert!(matches!(
        <crate::generate_pg_types_mod::I16AsNonNullInt2 as pg_crud_common::pg_type::PgType>::create_table_column_query_part(
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::pg_is_primary_key::PgIsPrimaryKey::from(false),
        ),
        Err(pg_crud_common::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
    let short_column = char::from(120u8).to_string();
    assert!(matches!(
        <crate::generate_pg_types_mod::I16AsNonNullInt2 as pg_crud_common::pg_type::PgType>::create_table_column_query_part(
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&short_column),
            pg_crud_common::pg_is_primary_key::PgIsPrimaryKey::from(false),
        ),
        Ok(fragment) if fragment.as_ref().starts_with(short_column.as_str())
    ));
}
