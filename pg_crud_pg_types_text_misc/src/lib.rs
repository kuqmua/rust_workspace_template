proc_macro_generate_pg_types::generate_pg_types!({
    "pg_table_cols_write_into_file": "False",
    "whole_write_into_file": "False",
    "generate_secret_text": true,
    "variant": {
        "Subset": [
            "StringAsText",
            "StdVecVecU8AsBytea",
            "SqlxTypesUuidUuidAsUuidV4InitializationByPg",
            "SqlxTypesUuidUuidAsUuidInitializationByClient",
            "SqlxTypesTimeTimeAsTime",
            "SqlxPgTypesPgIntervalAsInterval"
        ]
    }
});

#[cfg(feature = "test-utils")]
#[cfg(test)]
mod tests {
    #[test]
    fn test_generated_non_null_text_create_cases_preserve_fixture_order_and_json() {
        let cases = <crate::generate_pg_types_mod::StringAsNonNullText as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::optional_vec_create();
        assert!(cases.is_some_and(|values| {
            let expected = pg_crud_common::string_test_cases_vec::string_test_cases_vec();
            values.len() == expected.len()
                && values.into_iter().zip(expected).all(|(create, text)| {
                    serde_json::to_value(create).is_ok_and(|json| {
                        json.as_str() == Some(text.as_str())
                            && serde_json::from_value::<<crate::generate_pg_types_mod::StringAsNonNullText as pg_crud_common::pg_type::PgType>::Create>(json.clone())
                                .is_ok_and(|decoded| serde_json::to_value(decoded).is_ok_and(|encoded| encoded == json))
                    })
                })
        }));
    }

    #[test]
    fn test_generated_nullable_text_create_cases_append_null_after_fixture_values() {
        let cases = <crate::generate_pg_types_mod::OptionalStringAsNullableText as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::optional_vec_create();
        assert!(cases.is_some_and(|values| {
            let expected = pg_crud_common::string_test_cases_vec::string_test_cases_vec();
            values.len() == expected.len() + 1usize
                && values.into_iter().zip(expected.into_iter().map(Some).chain(std::iter::once(None)))
                    .all(|(create, text)| {
                        serde_json::to_value(create).is_ok_and(|json| {
                            let expected_json = text.map_or(serde_json::Value::Null, serde_json::Value::String);
                            json == expected_json
                                && serde_json::from_value::<<crate::generate_pg_types_mod::OptionalStringAsNullableText as pg_crud_common::pg_type::PgType>::Create>(json.clone())
                                    .is_ok_and(|decoded| serde_json::to_value(decoded).is_ok_and(|encoded| encoded == json))
                        })
                    })
        }));
    }

    #[test]
    fn test_generated_uuid_create_cases_preserve_primary_key_fallback_and_nullable_values() {
        assert!(<crate::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::optional_vec_create().is_none());
        let expected = vec![serde_json::Value::String(
            uuid::Uuid::from_u128(1u128).to_string(),
        )];
        assert!(<crate::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidInitializationByClient as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::optional_vec_create().is_some_and(|values| {
            values.len() == expected.len()
                && values.into_iter().zip(&expected).all(|(create, json)| serde_json::to_value(create).is_ok_and(|encoded| &encoded == json))
        }));
        assert!(<crate::generate_pg_types_mod::OptionalSqlxTypesUuidUuidAsNullableUuidInitializationByClient as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::optional_vec_create().is_some_and(|values| {
            values.len() == expected.len() + 1usize
                && values.into_iter().zip(expected.into_iter().chain(std::iter::once(serde_json::Value::Null)))
                    .all(|(create, json)| serde_json::to_value(create).is_ok_and(|encoded| encoded == json))
        }));
    }

    #[test]
    fn test_generated_nullable_text_read_cases_preserve_values_as_singleton_rows() {
        let read_ids = <crate::generate_pg_types_mod::OptionalStringAsNullableText as pg_crud_common::pg_type::PgType>::ReadIds::from(pg_crud_common::explicit_value::ExplicitValue::new(None));
        let rows = <crate::generate_pg_types_mod::OptionalStringAsNullableText as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::read_ids_to_2_dimensions_vec_read_inner(&read_ids);
        let expected = pg_crud_common::string_test_cases_vec::string_test_cases_vec()
            .into_iter()
            .map(|value| serde_json::Value::Array(vec![serde_json::Value::String(value)]))
            .chain(std::iter::once(serde_json::Value::Array(vec![
                serde_json::Value::Null,
            ])))
            .collect::<Vec<_>>();
        assert!(rows.iter().all(|row| row.len() == 1usize));
        assert!(
            serde_json::to_value(rows).is_ok_and(|json| json == serde_json::Value::Array(expected))
        );
    }
}
