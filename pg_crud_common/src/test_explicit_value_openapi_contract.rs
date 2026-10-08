#[test]
fn test_explicit_value_schema_uses_serialized_field_name() {
    let schema_result = serde_json::to_value(
        <crate::explicit_value::ExplicitValue<u8> as utoipa::PartialSchema>::schema(),
    );
    assert!(schema_result.is_ok());
    if let Ok(schema) = schema_result {
        assert!(
            serde_json::to_value(<u8 as utoipa::PartialSchema>::schema()).is_ok_and(
                |item_schema| {
                    schema
                        == serde_json::json!({
                            (stringify!(type)): constants_str::OBJECT,
                            (constants_str::PROPERTIES): { (stringify!(value)): item_schema },
                            (constants_str::REQUIRED): [stringify!(value)],
                        })
                }
            )
        );
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
                schema,
                serde_json::json!({
                    (stringify!(type)): constants_str::OBJECT,
                    (constants_str::PROPERTIES): { (stringify!(value)): expected_null_schema },
                    (constants_str::REQUIRED): [stringify!(value)],
                })
            );
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

#[test]
fn test_non_primary_key_read_ids_preserve_schema_name_and_postgres_type_metadata() {
    assert_eq!(<crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds as utoipa::ToSchema>::name(), constants_str::NONPRIMARYKEYPGTYPEREADIDS);
    let type_info =
        <crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds as sqlx::Type<
            sqlx::Postgres,
        >>::type_info();
    assert_eq!(type_info, <sqlx::types::Json<crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds> as sqlx::Type<sqlx::Postgres>>::type_info());
    assert!(
        <crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(&type_info)
    );
    assert!(
        !<crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds as sqlx::Type<
            sqlx::Postgres,
        >>::compatible(&<i64 as sqlx::Type<sqlx::Postgres>>::type_info())
    );
}

#[test]
fn test_non_primary_key_read_ids_deserialization_preserves_null_and_rejects_invalid_values() {
    let value = crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds::default();
    let serialized_result = serde_json::to_value(&value);
    assert!(serialized_result.is_ok());
    let Ok(serialized) = serialized_result else {
        return;
    };
    assert_eq!(serde_json::from_value::<crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds>(serialized).ok(), Some(value));
    assert!(serde_json::from_value::<crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds>(serde_json::json!({ stringify!(value): false })).is_err_and(|error| error.is_data()));
    assert!(serde_json::from_value::<crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds>(serde_json::Value::Null).is_err_and(|error| error.is_data()));
}
