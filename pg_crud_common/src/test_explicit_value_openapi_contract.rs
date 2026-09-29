#[test]
fn test_explicit_value_schema_uses_serialized_field_name() {
    let schema_result = serde_json::to_value(
        <crate::explicit_value::ExplicitValue<u8> as utoipa::PartialSchema>::schema(),
    );
    assert!(schema_result.is_ok());
    if let Ok(schema) = schema_result {
        assert!(
            schema
                .get(constants_str::PROPERTIES)
                .and_then(|properties| properties.get(stringify!(value)))
                .is_some()
        );
        assert!(
            schema
                .get(constants_str::PROPERTIES)
                .and_then(|properties| { properties.get(constants_str::PG_CRUD_VALUES_FIELD) })
                .is_none()
        );
    }
}

#[test]
fn test_non_primary_key_read_ids_schema_matches_serialized_null_value() {
    let serialized_result = serde_json::to_value(
        crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds::default(),
    );
    assert!(serialized_result.is_ok());
    if let Ok(serialized) = serialized_result {
        assert_eq!(
            serialized.get(stringify!(value)),
            Some(&serde_json::Value::Null)
        );
    }
    let schema_result = serde_json::to_value(
        <crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds as utoipa::PartialSchema>::schema(),
    );
    assert!(schema_result.is_ok());
    if let Ok(schema) = schema_result {
        let expected_null_schema_result = serde_json::to_value(
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::schema::Type::Null)
                .build(),
        );
        assert!(expected_null_schema_result.is_ok());
        if let Ok(expected_null_schema) = expected_null_schema_result {
            assert_eq!(
                schema
                    .get(constants_str::PROPERTIES)
                    .and_then(|properties| properties.get(stringify!(value))),
                Some(&expected_null_schema)
            );
        }
        assert!(
            schema
                .get(constants_str::PROPERTIES)
                .and_then(|properties| { properties.get(constants_str::PG_CRUD_VALUES_FIELD) })
                .is_none()
        );
    }
}
