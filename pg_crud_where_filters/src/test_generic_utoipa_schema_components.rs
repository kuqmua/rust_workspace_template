#[test]
fn test_between_schema_names_distinguish_item_types() {
    assert_ne!(
        <crate::between::Between<i32> as utoipa::ToSchema>::name(),
        <crate::between::Between<i64> as utoipa::ToSchema>::name()
    );
}

#[test]
fn test_between_schema_registers_item_component() {
    let mut schemas = Vec::new();
    <crate::between::Between<i32> as utoipa::ToSchema>::schemas(&mut schemas);
    assert!(
        schemas
            .iter()
            .any(|(name, _schema)| { name == <i32 as utoipa::ToSchema>::name().as_ref() })
    );
}

#[test]
fn test_pg_type_not_empty_unique_vec_schema_names_distinguish_item_types() {
    assert_ne!(
        <crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec<u8> as utoipa::ToSchema>::name(),
        <crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec<u16> as utoipa::ToSchema>::name()
    );
}

#[test]
fn test_pg_type_not_empty_unique_vec_schema_registers_item_component() {
    let mut schemas = Vec::new();
    <crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec<u8> as utoipa::ToSchema>::schemas(
        &mut schemas,
    );
    assert!(
        schemas
            .iter()
            .any(|(name, _schema)| { name == <u8 as utoipa::ToSchema>::name().as_ref() })
    );
}

#[test]
fn test_between_openapi_schema_requires_exactly_both_typed_bounds() {
    let schema = <crate::between::Between<i32> as utoipa::PartialSchema>::schema();
    assert!(matches!(schema,
        utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object))
            if object.schema_type == utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Object)
                && object.properties.len() == 2usize
                && object.properties.get(constants_str::PG_CRUD_START_FIELD) == Some(&<i32 as utoipa::PartialSchema>::schema())
                && object.properties.get(constants_str::PG_CRUD_END_FIELD) == Some(&<i32 as utoipa::PartialSchema>::schema())
                && object.required.iter().map(String::as_str).eq([
                    constants_str::PG_CRUD_START_FIELD, constants_str::PG_CRUD_END_FIELD,
                ])
    ));
}
