#[cfg(test)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::default_numeric_fallback, clippy::indexing_slicing)] // literal JSON assertions mirror the exact serialized OpenAPI wire values
mod tests {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct JsonContractValue {
        value: i32,
    }
    fn assert_schema_example_deserializes<T>()
    where
        T: utoipa::ToSchema + serde::Serialize + serde::de::DeserializeOwned,
    {
        fn first_example(schema: &serde_json::Value) -> Option<serde_json::Value> {
            if let Some(example) = schema
                .get(constants_str::VALUE_C590B3C9)
                .and_then(serde_json::Value::as_array)
                .and_then(|examples| examples.first())
            {
                return Some(example.clone());
            }
            match schema {
                serde_json::Value::Array(values) => values.iter().find_map(first_example),
                serde_json::Value::Object(values) => values.values().find_map(first_example),
                serde_json::Value::Bool(_)
                | serde_json::Value::Null
                | serde_json::Value::Number(_)
                | serde_json::Value::String(_) => None,
            }
        }
        let schema = <T as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_489F8964);
        let example = first_example(&schema_json).unwrap_or_else(|| {
            std::panic::panic_any(
                constants_str::PANIC_DFF79E9D
                    .replacen(
                        constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                        std::any::type_name::<T>(),
                        1usize,
                    )
                    .replacen(
                        constants_str::PANIC_PLACEHOLDER_SCHEMA_JSON,
                        schema_json.to_string().as_str(),
                        1usize,
                    ),
            )
        });
        let value = serde_json::from_value::<T>(example.clone()).unwrap_or_else(|error| {
            std::panic::panic_any(
                constants_str::PANIC_1E9E38EF
                    .replacen(
                        constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                        std::any::type_name::<T>(),
                        1usize,
                    )
                    .replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    )
                    .replacen(
                        constants_str::PANIC_PLACEHOLDER_EXAMPLE,
                        example.to_string().as_str(),
                        1usize,
                    ),
            )
        });
        assert_eq!(
            serde_json::to_value(value).expect(constants_str::DIAGNOSTIC_F126EFBE),
            example
        );
    }
    fn assert_wrapper_traits<T, Inner>()
    where
        T: From<Inner> + AsRef<Inner> + std::borrow::Borrow<Inner>,
    {
    }
    #[test]
    fn test_shared_json_contract_helper_round_trips_pg_type_fixture() {
        macro_helpers::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<
            JsonContractValue,
        >(macro_helpers::json_fixture_ref::JsonFixtureRef::from(
            constants_str::VALUE_7,
        ))
        .expect(constants_str::DIAGNOSTIC_13DF9134);
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "full type source generation is covered by native determinism tests and is prohibitively slow under interpretation"
    )]
    fn test_generated_output_is_deterministic() {
        let config = quote::quote! {{
            "pg_table_cols_write_into_file": "False",
            "whole_write_into_file": "False",
            "generate_secret_text": true,
            "variant": "All"
        }};
        let first = generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&config),
        );
        let second = generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&config),
        );
        assert_eq!(first.to_string(), second.to_string());
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "compiler subprocess validation is covered by the native Clippy gate"
    )]
    fn test_pg_types_generate_clippy() {
        macro_clippy_check_test_common::clippy_check(
            constants_str::GENERATE_PG_TYPES_TEST_CNT,
            constants_str::PG_CRUD_PG_TYPES,
            constants_str::DEPENDENCIES_NEWLINE_CHRONO_WORKSPACE_TRUE_NEWLINE_UUID_WORKSPACE_TRUE_NEWLINE_SQLX_WORKSPACE,
            &generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
                macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&quote::quote! {
                    {
                        "pg_table_cols_write_into_file": "False",
                        "whole_write_into_file": "False",
                        "generate_secret_text": true,
                        "variant": "All"
                    }
                }),
            )
            .to_string(),
        );
    }
    #[test]
    fn test_generated_integer_open_api_schema_has_format_bounds_and_example() {
        let schema = <pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_8AF67E13);
        assert_eq!(schema_json["type"], "integer");
        assert_eq!(schema_json["format"], "int32");
        assert_eq!(schema_json["minimum"], -32768);
        assert_eq!(schema_json["maximum"], 32767);
        assert_eq!(schema_json["examples"], serde_json::json!([42]));
    }
    #[test]
    fn test_generated_frontend_type_contract_matches_integer_wire_contract() {
        let contract =
            <pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2 as frontend_contract::has_type_contract::HasTypeContract>::type_contract();
        assert_eq!(
            contract.input_kind(),
            frontend_contract::input_kind::InputKind::Number
        );
        assert_eq!(
            contract.format(),
            frontend_contract::value_format::ValueFormat::Int16
        );
        assert_eq!(
            contract.nullability(),
            frontend_contract::nullability::Nullability::NonNullable
        );
        assert_eq!(
            contract.minimum(),
            frontend_contract::numeric_bound::NumericBound::Inclusive(
                frontend_contract::contract_i64::ContractI64::i16_min()
            )
        );
        assert_eq!(
            contract.maximum(),
            frontend_contract::numeric_bound::NumericBound::Inclusive(
                frontend_contract::contract_i64::ContractI64::i16_max()
            )
        );
    }
    #[test]
    fn test_generated_frontend_type_contract_preserves_nullable_uuid_semantics() {
        let contract = <pg_types_text_misc::generate_pg_types_mod::OptionalSqlxTypesUuidUuidAsNullableUuidInitializationByClient as frontend_contract::has_type_contract::HasTypeContract>::type_contract();
        assert_eq!(
            contract.input_kind(),
            frontend_contract::input_kind::InputKind::Uuid
        );
        assert_eq!(
            contract.format(),
            frontend_contract::value_format::ValueFormat::Uuid
        );
        assert_eq!(
            contract.nullability(),
            frontend_contract::nullability::Nullability::Nullable
        );
    }
    #[test]
    fn test_generated_form_value_contract_parses_and_formats_wire_values() {
        let integer = <pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin as frontend_contract::form_value_contract::FormValueContract>::parse_form_value(frontend_contract::form_value_ref::FormValueRef::from(constants_str::VALUE_42)).expect(constants_str::DIAGNOSTIC_0935C11D);
        assert_eq!(
            frontend_contract::form_value_contract::FormValueContract::format_form_value(&integer)
                .expect(constants_str::DIAGNOSTIC_144C7C4C)
                .as_ref(),
            "42"
        );
        let nullable = <pg_types_numeric::generate_pg_types_mod::OptionalI16AsNullableInt2Origin as frontend_contract::form_value_contract::FormValueContract>::parse_form_value(frontend_contract::form_value_ref::FormValueRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX)).expect(constants_str::DIAGNOSTIC_502918C1);
        assert_eq!(
            frontend_contract::form_value_contract::FormValueContract::format_form_value(&nullable)
                .expect(constants_str::DIAGNOSTIC_56531064)
                .as_ref(),
            ""
        );
        let uuid_value = constants_str::VALUE_7B93D4A1_6F28_4C70_9A51_2E8D3F640C12;
        let uuid = <pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin as frontend_contract::form_value_contract::FormValueContract>::parse_form_value(frontend_contract::form_value_ref::FormValueRef::from(uuid_value)).expect(constants_str::DIAGNOSTIC_804F13B2);
        assert_eq!(
            frontend_contract::form_value_contract::FormValueContract::format_form_value(&uuid)
                .expect(constants_str::DIAGNOSTIC_A17BCB42)
                .as_ref(),
            uuid_value
        );
        let timestamp = <pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoNaiveDateTimeAsNonNullTimestampOrigin as frontend_contract::form_value_contract::FormValueContract>::parse_form_value(frontend_contract::form_value_ref::FormValueRef::from(constants_str::VALUE_2026_07_13T12_30_00)).expect(constants_str::DIAGNOSTIC_AD1DE295);
        assert_eq!(
            frontend_contract::form_value_contract::FormValueContract::format_form_value(
                &timestamp
            )
            .expect(constants_str::DIAGNOSTIC_5A9F7D9C)
            .as_ref(),
            "2026-07-13T12:30:00"
        );
    }
    #[test]
    fn test_generated_filter_form_values_preserve_json_wire_types() {
        let integer = <pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2 as frontend_contract::filter_form_value_contract::FilterFormValueContract>::parse_filter_form_value(frontend_contract::form_value_ref::FormValueRef::from(constants_str::VALUE_42)).expect(constants_str::DIAGNOSTIC_12DF8CB5);
        assert_eq!(integer.as_ref(), "42");
        let timestamp = <pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoNaiveDateTimeAsNonNullTimestamp as frontend_contract::filter_form_value_contract::FilterFormValueContract>::parse_filter_form_value(frontend_contract::form_value_ref::FormValueRef::from(constants_str::VALUE_2026_07_13T12_30_00)).expect(constants_str::DIAGNOSTIC_98F3DF36);
        assert_eq!(
            timestamp.as_ref(),
            r#"{"date":"2026-07-13","time":{"hour":12,"min":30,"sec":0,"micro":0}}"#
        );
        let nullable = <pg_types_numeric::generate_pg_types_mod::OptionalI16AsNullableInt2 as frontend_contract::filter_form_value_contract::FilterFormValueContract>::parse_filter_form_value(frontend_contract::form_value_ref::FormValueRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX)).expect(constants_str::DIAGNOSTIC_B5939E08);
        assert_eq!(nullable.as_ref(), "null");
    }
    #[test]
    fn test_generated_nullable_open_api_schema_is_nullable() {
        let schema =
            <pg_types_numeric::generate_pg_types_mod::OptionalI16AsNullableInt2Origin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_F3B5A711);
        assert!(schema_json["oneOf"].as_array().is_some_and(|schemas| {
            schemas
                .iter()
                .any(|nullable_schema| nullable_schema["type"] == "null")
        }));
    }
    #[test]
    fn test_generated_uuid_open_api_schema_matches_wire_string() {
        let schema = <pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_80CB3EA4);
        assert_eq!(schema_json["type"], "string");
        assert_eq!(schema_json["format"], "uuid");
    }
    #[test]
    fn test_std_bound_wire_shape_is_stable_for_range_schemas() {
        assert_eq!(
            serde_json::to_value(std::ops::Bound::Included(1i32))
                .expect(constants_str::DIAGNOSTIC_90CDFBA3),
            serde_json::json!({"Included": 1})
        );
        assert_eq!(
            serde_json::to_value(std::ops::Bound::<i32>::Unbounded)
                .expect(constants_str::DIAGNOSTIC_2E7BD0DA),
            serde_json::json!("Unbounded")
        );
    }
    #[test]
    fn test_generated_time_open_api_properties_match_wire_object() {
        let time = pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin::try_new(
            chrono::NaiveTime::from_hms_micro_opt(12, 34, 56, 789).expect(constants_str::DIAGNOSTIC_C19F58A4),
        )
        .expect(constants_str::DIAGNOSTIC_68C0E12B);
        let wire = serde_json::to_value(time).expect(constants_str::DIAGNOSTIC_DE790942);
        let schema =
            <pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_DC191318);
        let wire_obj = wire.as_object().expect(constants_str::DIAGNOSTIC_E7150F4C);
        let schema_props = schema_json[constants_str::PROPERTIES]
            .as_object()
            .expect(constants_str::DIAGNOSTIC_85098DC5);
        assert!(wire_obj.keys().all(|key| schema_props.contains_key(key)));
        assert_eq!(schema_json["required"].as_array().map(Vec::len), Some(4));
    }
    #[test]
    fn test_generated_range_open_api_properties_match_wire_object() {
        let range = pg_types_numeric::generate_pg_types_mod::SqlxPgTypesPgRangeI32AsNonNullInt4RangeOrigin::try_new(
            sqlx::postgres::types::PgRange {
                start: std::ops::Bound::Included(1),
                end: std::ops::Bound::Excluded(3),
            },
        )
        .expect(constants_str::DIAGNOSTIC_760545B6);
        let wire = serde_json::to_value(range).expect(constants_str::DIAGNOSTIC_290B56BB);
        let schema =
            <pg_types_numeric::generate_pg_types_mod::SqlxPgTypesPgRangeI32AsNonNullInt4RangeOrigin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_72860BF4);
        let wire_obj = wire.as_object().expect(constants_str::DIAGNOSTIC_06A340B9);
        let schema_props = schema_json[constants_str::PROPERTIES]
            .as_object()
            .expect(constants_str::DIAGNOSTIC_3DC31CC6);
        assert!(wire_obj.keys().all(|key| schema_props.contains_key(key)));
        assert_eq!(
            schema_json["properties"]["start"]["oneOf"]
                .as_array()
                .map(Vec::len),
            Some(3)
        );
    }
    #[test]
    fn test_generated_filter_has_open_api_one_of_schema() {
        let schema = <pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Where as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_4BBD5367);
        assert!(
            schema_json["oneOf"]
                .as_array()
                .is_some_and(|variants| !variants.is_empty())
        );
    }
    #[test]
    fn test_generated_filters_follow_descriptor_capabilities() {
        let uuid_schema =
            <pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientWhere as utoipa::PartialSchema>::schema();
        let uuid_schema_json =
            serde_json::to_string(&uuid_schema).expect(constants_str::DIAGNOSTIC_C3AF72F5);
        assert!(uuid_schema_json.contains("In"));
        assert!(!uuid_schema_json.contains("Regex"));
        let string_schema =
            <pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextWhere as utoipa::PartialSchema>::schema();
        assert!(
            serde_json::to_string(&string_schema)
                .expect(constants_str::DIAGNOSTIC_2672B8C6)
                .contains("Regex")
        );
        let range_schema =
            <pg_types_numeric::generate_pg_types_mod::SqlxPgTypesPgRangeI32AsNonNullInt4RangeWhere as utoipa::PartialSchema>::schema();
        assert!(
            serde_json::to_string(&range_schema)
                .expect(constants_str::DIAGNOSTIC_C7954E5C)
                .contains("OverlapWithRange")
        );
    }
    #[test]
    fn test_generated_frontend_filters_follow_the_same_descriptor_matrix() {
        let number = <pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2 as frontend_contract::has_filter_contracts::HasFilterContracts>::filter_contracts();
        assert_eq!(
            number.as_ref().to_vec(),
            [
                frontend_contract::filter_operation::FilterOperation::Eq,
                frontend_contract::filter_operation::FilterOperation::GreaterThan,
                frontend_contract::filter_operation::FilterOperation::Between,
                frontend_contract::filter_operation::FilterOperation::In,
            ]
        );
        let text = <pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText as frontend_contract::has_filter_contracts::HasFilterContracts>::filter_contracts();
        assert_eq!(
            text.as_ref().to_vec(),
            [
                frontend_contract::filter_operation::FilterOperation::Eq,
                frontend_contract::filter_operation::FilterOperation::Regex,
            ]
        );
        assert_eq!(
            text.as_ref()
                .get(constants_usize::ONE)
                .map(|filter| filter.value_shape()),
            Some(frontend_contract::filter_value_shape::FilterValueShape::Regex)
        );
    }
    #[test]
    fn test_generated_schema_examples_deserialize_for_every_wire_kind() {
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::I32AsNonNullInt4Origin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8Origin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullSmallSerialInitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::I32AsNonNullSerialInitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::SqlxPgTypesPgMoneyAsNonNullMoneyOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::F32AsNonNullFloat4Origin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::F64AsNonNullFloat8Origin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBoolOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::generate_pg_types_mod::StdVecVecU8AsNonNullByteaOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::generate_pg_types_mod::SqlxTypesTimeTimeAsNonNullTimeOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::generate_pg_types_mod::SqlxPgTypesPgIntervalAsNonNullIntervalOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoNaiveDateAsNonNullDateOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoNaiveDateTimeAsNonNullTimestampOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTzOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::generate_pg_types_mod::SqlxTypesIpnetworkIpNetworkAsNonNullInetOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::generate_pg_types_mod::SqlxTypesMacAddressMacAddressAsNonNullMacAddrOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::SqlxPgTypesPgRangeI32AsNonNullInt4RangeOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::SqlxPgTypesPgRangeI64AsNonNullInt8RangeOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::generate_pg_types_mod::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsNonNullDateRangeOrigin,
        >();
        assert_schema_example_deserializes::<pg_types_chrono_net::generate_pg_types_mod::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNonNullTimestampRangeOrigin>();
        assert_schema_example_deserializes::<pg_types_chrono_net::generate_pg_types_mod::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTzRangeOrigin>();
        assert_schema_example_deserializes::<
            pg_types_numeric::generate_pg_types_mod::OptionalI16AsNullableInt2Origin,
        >();
        let _nullable_value = serde_json::from_value::<
            pg_types_numeric::generate_pg_types_mod::OptionalI16AsNullableInt2Origin,
        >(serde_json::Value::Null)
        .expect(constants_str::DIAGNOSTIC_4063A869);
    }
    #[test]
    fn test_generated_wire_contract_rejects_invalid_values() {
        drop(
            serde_json::from_value::<pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin>(serde_json::json!(
                32768
            ))
            .expect_err(constants_str::VALUE_18E07769),
        );
        drop(
            serde_json::from_value::<
                pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin,
            >(serde_json::json!("not-a-uuid"))
            .expect_err(constants_str::VALUE_4805266C),
        );
        drop(
            serde_json::from_value::<
                pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin,
            >(serde_json::json!({"hour": 24, "min": 0, "sec": 0, "micro": 0}))
            .expect_err(constants_str::VALUE_66B5606B),
        );
        drop(
            serde_json::from_value::<
                pg_types_chrono_net::generate_pg_types_mod::SqlxTypesMacAddressMacAddressAsNonNullMacAddrOrigin,
            >(serde_json::json!([0, 1, 2]))
            .expect_err(constants_str::CABD480A),
        );
    }
    #[test]
    fn test_generated_float8_rejects_non_finite_values() {
        let _finite =
            pg_types_numeric::generate_pg_types_mod::F64AsNonNullFloat8Origin::try_new(1.5f64)
                .expect(constants_str::DIAGNOSTIC_40483CD5);
        drop(
            pg_types_numeric::generate_pg_types_mod::F64AsNonNullFloat8Origin::try_new(f64::NAN)
                .expect_err(constants_str::VALUE_A3C9AE5D),
        );
        drop(
            pg_types_numeric::generate_pg_types_mod::F64AsNonNullFloat8Origin::try_new(
                f64::INFINITY,
            )
            .expect_err(constants_str::VALUE_CD23DFD9),
        );
        drop(
            <pg_types_numeric::generate_pg_types_mod::F64AsNonNullFloat8Origin as serde::Deserialize>::deserialize(
                serde::de::value::F64Deserializer::<serde::de::value::Error>::new(
                    f64::NEG_INFINITY,
                ),
            )
            .expect_err(constants_str::VALUE_D22548CF),
        );
    }
    #[test]
    fn test_generated_wrapper_roles_have_standard_conversions_and_borrows() {
        assert_wrapper_traits::<
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2TableType,
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin,
        >();
        assert_wrapper_traits::<
            pg_types_text_misc::generate_pg_types_mod::SqlxPgTypesPgIntervalAsNonNullIntervalCreate,
            pg_types_text_misc::generate_pg_types_mod::SqlxPgTypesPgIntervalAsNonNullIntervalOrigin,
        >();
        assert_wrapper_traits::<
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Read,
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin,
        >();
        assert_wrapper_traits::<
            pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPgReadIds,
            pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPgRead,
        >();
        assert_wrapper_traits::<
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Update,
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin,
        >();
        assert_wrapper_traits::<
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2UpdateForQuery,
            pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2Origin,
        >();
    }
    #[test]
    fn test_generated_secret_text_is_redacted_and_borrowable() {
        fn assert_traits<T>()
        where
            T: Clone
                + Eq
                + std::fmt::Debug
                + AsRef<str>
                + std::borrow::Borrow<str>
                + sqlx::Type<sqlx::Postgres>,
        {
        }
        assert_traits::<pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret>();
        let secret = pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret::from(
            constants_str::SECRET_VALUE.to_owned(),
        );
        assert_eq!(format!("{secret:?}"), "[REDACTED]");
        let borrowed =
            pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecretRef::from(&secret);
        assert_eq!(format!("{borrowed:?}"), "[REDACTED]");
        assert_eq!(borrowed.as_ref(), "secret-value");
    }
}
