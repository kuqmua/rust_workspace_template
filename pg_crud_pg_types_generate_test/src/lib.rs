#[cfg(test)]
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
        let schema_json =
            serde_json::to_value(schema).expect("489f8964 first_example invariant must hold");
        let example = first_example(&schema_json).unwrap_or_else(|| {
            panic!(
                "dff79e9d schema for {} has no examples array: {schema_json}",
                std::any::type_name::<T>(),
            )
        });
        let value = serde_json::from_value::<T>(example.clone()).unwrap_or_else(|error| {
            panic!(
                "1e9e38ef schema example for {} failed to deserialize: {error}; example: {example}",
                std::any::type_name::<T>(),
            )
        });
        assert_eq!(
            serde_json::to_value(value).expect("f126efbe first_example invariant must hold"),
            example
        );
    }
    fn assert_wrapper_traits<T, Inner>()
    where
        T: From<Inner> + AsRef<Inner> + std::borrow::Borrow<Inner>,
    {
    }
    #[test]
    fn shared_json_contract_helper_round_trips_pg_type_fixture() {
        macro_helpers::domain_types::json_contract::ensure_json_contract_round_trip::<
            JsonContractValue,
        >(
            macro_helpers::domain_types::json_contract::JsonFixtureRef::from(
                constants_str::VALUE_7,
            ),
        )
        .expect(
            "13df9134 shared_json_contract_helper_round_trips_pg_type_fixture invariant must hold",
        );
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "full type source generation is covered by native determinism tests and is prohibitively slow under interpretation"
    )]
    fn generated_output_is_deterministic() {
        let config = quote::quote! {{
            "pg_table_cols_write_into_file": "False",
            "whole_write_into_file": "False",
            "generate_secret_text": true,
            "variant": "All"
        }};
        let first = generate_pg_types_src::domain_types::source::generate_pg_types(
            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&config),
        );
        let second = generate_pg_types_src::domain_types::source::generate_pg_types(
            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&config),
        );
        assert_eq!(first.to_string(), second.to_string());
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "compiler subprocess validation is covered by the native Clippy gate"
    )]
    fn clippy() {
        macro_clippy_check_common::clippy_check(
            constants_str::GENERATE_PG_TYPES_TEST_CNT,
            constants_str::PG_CRUD_PG_TYPES,
            constants_str::DEPENDENCIES_NEWLINE_CHRONO_WORKSPACE_TRUE_NEWLINE_UUID_WORKSPACE_TRUE_NEWLINE_SQLX_WORKSPACE,
            &generate_pg_types_src::domain_types::source::generate_pg_types(
                macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&quote::quote! {
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
    fn generated_integer_open_api_schema_has_format_bounds_and_example() {
        let schema = <pg_types_numeric::I16AsNonNullInt2Origin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect("8af67e13 generated_integer_open_api_schema_has_format_bounds_and_example invariant must hold");
        assert_eq!(schema_json["type"], "integer");
        assert_eq!(schema_json["format"], "int32");
        assert_eq!(schema_json["minimum"], -32768);
        assert_eq!(schema_json["maximum"], 32767);
        assert_eq!(schema_json["examples"], serde_json::json!([42]));
    }
    #[test]
    fn generated_frontend_type_contract_matches_integer_wire_contract() {
        let contract =
            <pg_types_numeric::I16AsNonNullInt2 as frontend_contract::domain_types::HasTypeContract>::type_contract();
        assert_eq!(
            contract.input_kind(),
            frontend_contract::domain_types::InputKind::Number
        );
        assert_eq!(
            contract.format(),
            frontend_contract::domain_types::ValueFormat::Int16
        );
        assert_eq!(
            contract.nullability(),
            frontend_contract::domain_types::Nullability::NonNullable
        );
        assert_eq!(
            contract.minimum(),
            frontend_contract::domain_types::NumericBound::Inclusive(
                frontend_contract::domain_types::ContractI64::i16_min()
            )
        );
        assert_eq!(
            contract.maximum(),
            frontend_contract::domain_types::NumericBound::Inclusive(
                frontend_contract::domain_types::ContractI64::i16_max()
            )
        );
    }
    #[test]
    fn generated_frontend_type_contract_preserves_nullable_uuid_semantics() {
        let contract = <pg_types_text_misc::OptionalSqlxTypesUuidUuidAsNullableUuidInitializationByClient as frontend_contract::domain_types::HasTypeContract>::type_contract();
        assert_eq!(
            contract.input_kind(),
            frontend_contract::domain_types::InputKind::Uuid
        );
        assert_eq!(
            contract.format(),
            frontend_contract::domain_types::ValueFormat::Uuid
        );
        assert_eq!(
            contract.nullability(),
            frontend_contract::domain_types::Nullability::Nullable
        );
    }
    #[test]
    fn generated_form_value_contract_parses_and_formats_wire_values() {
        let integer = <pg_types_numeric::I16AsNonNullInt2Origin as frontend_contract::domain_types::FormValueContract>::parse_form_value(frontend_contract::domain_types::FormValueRef::from(constants_str::VALUE_42)).expect("0935c11d generated_form_value_contract_parses_and_formats_wire_values invariant must hold");
        assert_eq!(
            frontend_contract::domain_types::FormValueContract::format_form_value(&integer)
                .expect("144c7c4c generated_form_value_contract_parses_and_formats_wire_values invariant must hold")
                .as_ref(),
            "42"
        );
        let nullable = <pg_types_numeric::OptionalI16AsNullableInt2Origin as frontend_contract::domain_types::FormValueContract>::parse_form_value(frontend_contract::domain_types::FormValueRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX)).expect("502918c1 generated_form_value_contract_parses_and_formats_wire_values invariant must hold");
        assert_eq!(
            frontend_contract::domain_types::FormValueContract::format_form_value(&nullable)
                .expect("56531064 generated_form_value_contract_parses_and_formats_wire_values invariant must hold")
                .as_ref(),
            ""
        );
        let uuid_value = constants_str::VALUE_7B93D4A1_6F28_4C70_9A51_2E8D3F640C12;
        let uuid = <pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin as frontend_contract::domain_types::FormValueContract>::parse_form_value(frontend_contract::domain_types::FormValueRef::from(uuid_value)).expect("804f13b2 generated_form_value_contract_parses_and_formats_wire_values invariant must hold");
        assert_eq!(
            frontend_contract::domain_types::FormValueContract::format_form_value(&uuid)
                .expect("a17bcb42 generated_form_value_contract_parses_and_formats_wire_values invariant must hold")
                .as_ref(),
            uuid_value
        );
        let timestamp = <pg_types_chrono_net::SqlxTypesChronoNaiveDateTimeAsNonNullTimestampOrigin as frontend_contract::domain_types::FormValueContract>::parse_form_value(frontend_contract::domain_types::FormValueRef::from(constants_str::VALUE_2026_07_13T12_30_00)).expect("ad1de295 generated_form_value_contract_parses_and_formats_wire_values invariant must hold");
        assert_eq!(
            frontend_contract::domain_types::FormValueContract::format_form_value(&timestamp)
                .expect("5a9f7d9c generated_form_value_contract_parses_and_formats_wire_values invariant must hold")
                .as_ref(),
            "2026-07-13T12:30:00"
        );
    }
    #[test]
    fn generated_filter_form_values_preserve_json_wire_types() {
        let integer = <pg_types_numeric::I16AsNonNullInt2 as frontend_contract::domain_types::FilterFormValueContract>::parse_filter_form_value(frontend_contract::domain_types::FormValueRef::from(constants_str::VALUE_42)).expect("12df8cb5 generated_filter_form_values_preserve_json_wire_types invariant must hold");
        assert_eq!(integer.as_ref(), "42");
        let timestamp = <pg_types_chrono_net::SqlxTypesChronoNaiveDateTimeAsNonNullTimestamp as frontend_contract::domain_types::FilterFormValueContract>::parse_filter_form_value(frontend_contract::domain_types::FormValueRef::from(constants_str::VALUE_2026_07_13T12_30_00)).expect("98f3df36 generated_filter_form_values_preserve_json_wire_types invariant must hold");
        assert_eq!(
            timestamp.as_ref(),
            r#"{"date":"2026-07-13","time":{"hour":12,"min":30,"sec":0,"micro":0}}"#
        );
        let nullable = <pg_types_numeric::OptionalI16AsNullableInt2 as frontend_contract::domain_types::FilterFormValueContract>::parse_filter_form_value(frontend_contract::domain_types::FormValueRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX)).expect("b5939e08 generated_filter_form_values_preserve_json_wire_types invariant must hold");
        assert_eq!(nullable.as_ref(), "null");
    }
    #[test]
    fn generated_nullable_open_api_schema_is_nullable() {
        let schema =
            <pg_types_numeric::OptionalI16AsNullableInt2Origin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema)
            .expect("f3b5a711 generated_nullable_open_api_schema_is_nullable invariant must hold");
        assert!(schema_json["oneOf"].as_array().is_some_and(|schemas| {
            schemas
                .iter()
                .any(|nullable_schema| nullable_schema["type"] == "null")
        }));
    }
    #[test]
    fn generated_uuid_open_api_schema_matches_wire_string() {
        let schema = <pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(
            "80cb3ea4 generated_uuid_open_api_schema_matches_wire_string invariant must hold",
        );
        assert_eq!(schema_json["type"], "string");
        assert_eq!(schema_json["format"], "uuid");
    }
    #[test]
    fn std_bound_wire_shape_is_stable_for_range_schemas() {
        assert_eq!(
            serde_json::to_value(std::ops::Bound::Included(1i32)).expect(
                "90cdfba3 std_bound_wire_shape_is_stable_for_range_schemas invariant must hold"
            ),
            serde_json::json!({"Included": 1})
        );
        assert_eq!(
            serde_json::to_value(std::ops::Bound::<i32>::Unbounded).expect(
                "2e7bd0da std_bound_wire_shape_is_stable_for_range_schemas invariant must hold"
            ),
            serde_json::json!("Unbounded")
        );
    }
    #[test]
    fn generated_time_open_api_properties_match_wire_object() {
        let time = pg_types_chrono_net::SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin::try_new(
            chrono::NaiveTime::from_hms_micro_opt(12, 34, 56, 789).expect(
                "c19f58a4 generated_time_open_api_properties_match_wire_object invariant must hold",
            ),
        )
        .expect(
            "68c0e12b generated_time_open_api_properties_match_wire_object invariant must hold",
        );
        let wire = serde_json::to_value(time).expect(
            "de790942 generated_time_open_api_properties_match_wire_object invariant must hold",
        );
        let schema =
            <pg_types_chrono_net::SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(
            "dc191318 generated_time_open_api_properties_match_wire_object invariant must hold",
        );
        let wire_obj = wire.as_object().expect(
            "e7150f4c generated_time_open_api_properties_match_wire_object invariant must hold",
        );
        let schema_props = schema_json[constants_str::PROPERTIES].as_object().expect(
            "85098dc5 generated_time_open_api_properties_match_wire_object invariant must hold",
        );
        assert!(wire_obj.keys().all(|key| schema_props.contains_key(key)));
        assert_eq!(schema_json["required"].as_array().map(Vec::len), Some(4));
    }
    #[test]
    fn generated_range_open_api_properties_match_wire_object() {
        let range = pg_types_numeric::SqlxPgTypesPgRangeI32AsNonNullInt4RangeOrigin::try_new(
            sqlx::postgres::types::PgRange {
                start: std::ops::Bound::Included(1),
                end: std::ops::Bound::Excluded(3),
            },
        )
        .expect(
            "760545b6 generated_range_open_api_properties_match_wire_object invariant must hold",
        );
        let wire = serde_json::to_value(range).expect(
            "290b56bb generated_range_open_api_properties_match_wire_object invariant must hold",
        );
        let schema =
            <pg_types_numeric::SqlxPgTypesPgRangeI32AsNonNullInt4RangeOrigin as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect(
            "72860bf4 generated_range_open_api_properties_match_wire_object invariant must hold",
        );
        let wire_obj = wire.as_object().expect(
            "06a340b9 generated_range_open_api_properties_match_wire_object invariant must hold",
        );
        let schema_props = schema_json[constants_str::PROPERTIES].as_object().expect(
            "3dc31cc6 generated_range_open_api_properties_match_wire_object invariant must hold",
        );
        assert!(wire_obj.keys().all(|key| schema_props.contains_key(key)));
        assert_eq!(
            schema_json["properties"]["start"]["oneOf"]
                .as_array()
                .map(Vec::len),
            Some(3)
        );
    }
    #[test]
    fn generated_filter_has_open_api_one_of_schema() {
        let schema = <pg_types_numeric::I16AsNonNullInt2Where as utoipa::PartialSchema>::schema();
        let schema_json = serde_json::to_value(schema)
            .expect("4bbd5367 generated_filter_has_open_api_one_of_schema invariant must hold");
        assert!(
            schema_json["oneOf"]
                .as_array()
                .is_some_and(|variants| !variants.is_empty())
        );
    }
    #[test]
    fn generated_filters_follow_descriptor_capabilities() {
        let uuid_schema =
            <pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientWhere as utoipa::PartialSchema>::schema();
        let uuid_schema_json = serde_json::to_string(&uuid_schema).expect(
            "c3af72f5 generated_filters_follow_descriptor_capabilities invariant must hold",
        );
        assert!(uuid_schema_json.contains("In"));
        assert!(!uuid_schema_json.contains("Regex"));
        let string_schema =
            <pg_types_text_misc::StringAsNonNullTextWhere as utoipa::PartialSchema>::schema();
        assert!(
            serde_json::to_string(&string_schema)
                .expect(
                    "2672b8c6 generated_filters_follow_descriptor_capabilities invariant must hold"
                )
                .contains("Regex")
        );
        let range_schema =
            <pg_types_numeric::SqlxPgTypesPgRangeI32AsNonNullInt4RangeWhere as utoipa::PartialSchema>::schema();
        assert!(
            serde_json::to_string(&range_schema)
                .expect(
                    "c7954e5c generated_filters_follow_descriptor_capabilities invariant must hold"
                )
                .contains("OverlapWithRange")
        );
    }
    #[test]
    fn generated_frontend_filters_follow_the_same_descriptor_matrix() {
        let number = <pg_types_numeric::I16AsNonNullInt2 as frontend_contract::domain_types::HasFilterContracts>::filter_contracts();
        assert_eq!(
            number.as_ref().to_vec(),
            vec![
                frontend_contract::domain_types::FilterOperation::Eq,
                frontend_contract::domain_types::FilterOperation::GreaterThan,
                frontend_contract::domain_types::FilterOperation::Between,
                frontend_contract::domain_types::FilterOperation::In,
            ]
        );
        let text = <pg_types_text_misc::StringAsNonNullText as frontend_contract::domain_types::HasFilterContracts>::filter_contracts();
        assert_eq!(
            text.as_ref().to_vec(),
            vec![
                frontend_contract::domain_types::FilterOperation::Eq,
                frontend_contract::domain_types::FilterOperation::Regex,
            ]
        );
        assert_eq!(
            text.as_ref()
                .get(constants_usize::ONE)
                .map(|filter| filter.value_shape()),
            Some(frontend_contract::domain_types::FilterValueShape::Regex)
        );
    }
    #[test]
    fn generated_schema_examples_deserialize_for_every_wire_kind() {
        assert_schema_example_deserializes::<pg_types_numeric::I16AsNonNullInt2Origin>();
        assert_schema_example_deserializes::<pg_types_numeric::I32AsNonNullInt4Origin>();
        assert_schema_example_deserializes::<pg_types_numeric::I64AsNonNullInt8Origin>();
        assert_schema_example_deserializes::<
            pg_types_numeric::I16AsNonNullSmallSerialInitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::I32AsNonNullSerialInitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::I64AsNonNullBigSerialInitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::SqlxPgTypesPgMoneyAsNonNullMoneyOrigin,
        >();
        assert_schema_example_deserializes::<pg_types_numeric::F32AsNonNullFloat4Origin>();
        assert_schema_example_deserializes::<pg_types_numeric::F64AsNonNullFloat8Origin>();
        assert_schema_example_deserializes::<pg_types_numeric::BoolAsNonNullBoolOrigin>();
        assert_schema_example_deserializes::<pg_types_text_misc::StringAsNonNullTextOrigin>();
        assert_schema_example_deserializes::<pg_types_text_misc::StdVecVecU8AsNonNullByteaOrigin>();
        assert_schema_example_deserializes::<
            pg_types_text_misc::SqlxTypesTimeTimeAsNonNullTimeOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::SqlxPgTypesPgIntervalAsNonNullIntervalOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::SqlxTypesChronoNaiveDateAsNonNullDateOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::SqlxTypesChronoNaiveDateTimeAsNonNullTimestampOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTzOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPgOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::SqlxTypesIpnetworkIpNetworkAsNonNullInetOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::SqlxTypesMacAddressMacAddressAsNonNullMacAddrOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::SqlxPgTypesPgRangeI32AsNonNullInt4RangeOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_numeric::SqlxPgTypesPgRangeI64AsNonNullInt8RangeOrigin,
        >();
        assert_schema_example_deserializes::<
            pg_types_chrono_net::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsNonNullDateRangeOrigin,
        >();
        assert_schema_example_deserializes::<pg_types_chrono_net::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNonNullTimestampRangeOrigin>();
        assert_schema_example_deserializes::<pg_types_chrono_net::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTzRangeOrigin>();
        assert_schema_example_deserializes::<pg_types_numeric::OptionalI16AsNullableInt2Origin>();
        let _nullable_value = serde_json::from_value::<
            pg_types_numeric::OptionalI16AsNullableInt2Origin,
        >(serde_json::Value::Null)
        .expect("4063a869 generated_schema_examples_deserialize_for_every_wire_kind invariant must hold");
    }
    #[test]
    fn generated_wire_contract_rejects_invalid_values() {
        drop(
            serde_json::from_value::<pg_types_numeric::I16AsNonNullInt2Origin>(serde_json::json!(
                32768
            ))
            .expect_err(constants_str::VALUE_18E07769),
        );
        drop(
            serde_json::from_value::<
                pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidInitializationByClientOrigin,
            >(serde_json::json!("not-a-uuid"))
            .expect_err(constants_str::VALUE_4805266C),
        );
        drop(
            serde_json::from_value::<
                pg_types_chrono_net::SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin,
            >(serde_json::json!({"hour": 24, "min": 0, "sec": 0, "micro": 0}))
            .expect_err(constants_str::VALUE_66B5606B),
        );
        drop(
            serde_json::from_value::<
                pg_types_chrono_net::SqlxTypesMacAddressMacAddressAsNonNullMacAddrOrigin,
            >(serde_json::json!([0, 1, 2]))
            .expect_err(constants_str::CABD480A),
        );
    }
    #[test]
    fn generated_float8_rejects_non_finite_values() {
        let _finite = pg_types_numeric::F64AsNonNullFloat8Origin::try_new(1.5f64)
            .expect("40483cd5 generated_float8_rejects_non_finite_values invariant must hold");
        drop(
            pg_types_numeric::F64AsNonNullFloat8Origin::try_new(f64::NAN)
                .expect_err(constants_str::VALUE_A3C9AE5D),
        );
        drop(
            pg_types_numeric::F64AsNonNullFloat8Origin::try_new(f64::INFINITY)
                .expect_err(constants_str::VALUE_CD23DFD9),
        );
        drop(
            <pg_types_numeric::F64AsNonNullFloat8Origin as serde::Deserialize>::deserialize(
                serde::de::value::F64Deserializer::<serde::de::value::Error>::new(
                    f64::NEG_INFINITY,
                ),
            )
            .expect_err(constants_str::VALUE_D22548CF),
        );
    }
    #[test]
    fn generated_wrapper_roles_have_standard_conversions_and_borrows() {
        assert_wrapper_traits::<
            pg_types_numeric::I16AsNonNullInt2TableType,
            pg_types_numeric::I16AsNonNullInt2Origin,
        >();
        assert_wrapper_traits::<
            pg_types_text_misc::SqlxPgTypesPgIntervalAsNonNullIntervalCreate,
            pg_types_text_misc::SqlxPgTypesPgIntervalAsNonNullIntervalOrigin,
        >();
        assert_wrapper_traits::<
            pg_types_numeric::I16AsNonNullInt2Read,
            pg_types_numeric::I16AsNonNullInt2Origin,
        >();
        assert_wrapper_traits::<
            pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPgReadIds,
            pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPgRead,
        >();
        assert_wrapper_traits::<
            pg_types_numeric::I16AsNonNullInt2Update,
            pg_types_numeric::I16AsNonNullInt2Origin,
        >();
        assert_wrapper_traits::<
            pg_types_numeric::I16AsNonNullInt2UpdateForQuery,
            pg_types_numeric::I16AsNonNullInt2Origin,
        >();
    }
    #[test]
    fn generated_secret_text_is_redacted_and_borrowable() {
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
        assert_traits::<pg_types_text_misc::StringAsNonNullTextSecret>();
        let secret = pg_types_text_misc::StringAsNonNullTextSecret::from(
            constants_str::SECRET_VALUE.to_owned(),
        );
        assert_eq!(format!("{secret:?}"), "[REDACTED]");
        let borrowed = pg_types_text_misc::StringAsNonNullTextSecretRef::from(&secret);
        assert_eq!(format!("{borrowed:?}"), "[REDACTED]");
        assert_eq!(borrowed.as_ref(), "secret-value");
    }
}
