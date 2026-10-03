#[test]
fn test_model_can_be_parsed_and_validated_without_emitting_source() {
    let input = quote::quote! {{
        "pg_table_cols_write_into_file": "False",
        "whole_write_into_file": "False",
        "variant": {"Subset": ["I16AsInt2", "StringAsText"]}
    }};
    let parsed = crate::parse_generate_pg_types::parse_generate_pg_types(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(constants_str::DIAGNOSTIC_35A0F719);
    let built = crate::build_generate_pg_types::build_generate_pg_types(parsed)
        .expect(constants_str::DIAGNOSTIC_3C8D514F);
    let validated = crate::validate_generate_pg_types::validate_generate_pg_types(built)
        .expect(constants_str::DIAGNOSTIC_B24816DE);
    assert_eq!(usize::from(validated.entry_count()), 2usize);
}

#[test]
fn test_malformed_config_is_a_typed_parse_error() {
    let input = quote::quote! {{"variant": "MissingFields"}};
    assert!(matches!(
        crate::parse_generate_pg_types::parse_generate_pg_types(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        ),
        Err(crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError::Parse(_error))
    ));
}

#[test]
fn test_generated_type_list_deserialization_rejects_too_many_entries() {
    let serialized = serde_json::to_string(
        &[crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsInt2;
            crate::generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN + constants_usize::ONE][..],
    )
    .expect(constants_str::DIAGNOSTIC_7CD2E0AF);
    let _error = serde_json::from_str::<crate::generate_pg_types::GeneratePgTypes>(&serialized)
        .expect_err(constants_str::VALUE_28B750CB);
}

#[test]
fn test_checked_pg_initialization_conversion_accepts_exact_supported_catalog() {
    let supported = [
        crate::pg_type_catalog_kind::PgTypeCatalogKind::F32AsFloat4,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::F64AsFloat8,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::StringAsText,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveTimeAsTime,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesTimeTimeAsTime,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateAsDate,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateTimeAsTimestamp,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI32AsInt4Range,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI64AsInt8Range,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    ];
    assert!(
        <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter().all(
            |kind| {
                match crate::pg_type_initialization_try_new::PgTypeInitializationTryNew::try_from(
                    &kind,
                ) {
                    Ok(initialization) => {
                        supported.contains(&kind)
                            && crate::pg_type_catalog_kind::PgTypeCatalogKind::from(&initialization)
                                == kind
                    }
                    Err(()) => !supported.contains(&kind),
                }
            }
        )
    );
}

#[test]
fn test_pg_range_conversion_emits_scalar_element_identifiers_and_rejects_other_types() {
    let cases = [
        (crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI32AsInt4Range, crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsInt4, quote::quote! { I32AsInt4NonNull }),
        (crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI64AsInt8Range, crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsInt8, quote::quote! { I64AsInt8NonNull }),
        (crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange, crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateAsDate, quote::quote! { SqlxTypesChronoNaiveDateAsDateNonNull }),
        (crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange, crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateTimeAsTimestamp, quote::quote! { SqlxTypesChronoNaiveDateTimeAsTimestampNonNull }),
        (crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange, crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz, quote::quote! { SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzNonNull }),
    ];
    assert!(
        <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter().all(
            |kind| {
                let expected = cases.iter().find(|(range_kind, _, _)| *range_kind == kind);
                match crate::range::Range::try_from(&kind) {
                    Ok(range) => expected.is_some_and(|(_, element, tokens)| {
                        assert_eq!(range.to_string(), tokens.to_string());
                        crate::pg_type_catalog_kind::PgTypeCatalogKind::from(&range) == *element
                            && range.to_string() == tokens.to_string()
                            && quote::quote! { #range }.to_string() == tokens.to_string()
                    }),
                    Err(()) => expected.is_none(),
                }
            }
        )
    );
}

#[test]
fn test_pg_record_deserialization_validates_nullability_for_every_catalog_kind() {
    let nonnullable = [
        crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsSmallSerialInitializationByPg,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsSerialInitializationByPg,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsBigSerialInitializationByPg,
        crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidV4InitializationByPg,
    ];
    assert!(
        <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter().all(
            |kind| {
                [
                    pg_crud_macro_common::is_nullable::IsNullable::False,
                    pg_crud_macro_common::is_nullable::IsNullable::True,
                ]
                .into_iter()
                .all(|nullable| {
                    let original = crate::pg_type_record::PgTypeRecord::new(
                        kind,
                        nullable,
                        crate::pg_type_pattern::PgTypePattern::Standard,
                    );
                    let Ok(serialized) = serde_json::to_value(original) else {
                        return false;
                    };
                    let restored =
                        serde_json::from_value::<crate::pg_type_record::PgTypeRecord>(serialized);
                    if nullable == pg_crud_macro_common::is_nullable::IsNullable::True
                        && nonnullable.contains(&kind)
                    {
                        restored.is_err_and(|error| {
                            error
                                .to_string()
                                .starts_with(constants_str::CANT_SUPPORT_NULLABLE_VARIANTS)
                        })
                    } else {
                        restored.is_ok_and(|record| {
                            record == original
                                && *record.get_pg_type() == kind
                                && *record.get_is_nullable() == nullable
                                && *record.get_pg_type_pattern()
                                    == crate::pg_type_pattern::PgTypePattern::Standard
                        })
                    }
                })
            }
        )
    );
}

#[test]
fn test_pg_record_collections_enforce_length_at_conversion_and_deserialization() {
    let record = crate::pg_type_record::PgTypeRecord::new(
        crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsInt2,
        pg_crud_macro_common::is_nullable::IsNullable::False,
        crate::pg_type_pattern::PgTypePattern::Standard,
    );
    let maximum = crate::generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN;
    assert!([0usize, maximum, maximum + 1usize].into_iter().all(|length| {
        let records = vec![record; length];
        let Ok(serialized) = serde_json::to_value(&records) else { return false; };
        let direct = crate::generate_pg_type_records::GeneratePgTypeRecords::try_from(records);
        let deserialized = serde_json::from_value::<crate::generate_pg_type_records::GeneratePgTypeRecords>(serialized);
        if length > maximum {
            matches!(direct, Err(crate::generate_pg_types_length_error::GeneratePgTypesLengthError::TooLarge))
                && deserialized.is_err_and(|error| error.to_string().contains(stringify!(TooLarge)))
        } else {
            direct.is_ok_and(|values| values.len() == length && values.iter().all(|value| *value == record))
                && deserialized.is_ok_and(|values| values.len() == length && values.iter().all(|value| *value == record))
        }
    }));
}

#[test]
fn test_pg_type_pipeline_counts_all_empty_subset_and_concrete_variants() {
    let all_count =
        <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter().count();
    let cases = [
        (
            quote::quote! {{ "pg_table_cols_write_into_file": "False", "whole_write_into_file": "False", "variant": "All" }},
            all_count,
        ),
        (
            quote::quote! {{ "pg_table_cols_write_into_file": "False", "whole_write_into_file": "False", "variant": {"Subset": []} }},
            0usize,
        ),
        (
            quote::quote! {{ "pg_table_cols_write_into_file": "False", "whole_write_into_file": "False", "variant": {"Concrete": []} }},
            0usize,
        ),
        (
            quote::quote! {{ "pg_table_cols_write_into_file": "False", "whole_write_into_file": "False", "variant": {"Concrete": [
                {"pg_type": "I16AsInt2", "is_nullable": "False", "pg_type_pattern": "Standard"},
                {"pg_type": "StringAsText", "is_nullable": "True", "pg_type_pattern": "Standard"}
            ]} }},
            2usize,
        ),
    ];
    assert!(cases.into_iter().all(|(input, expected_count)| {
        crate::parse_generate_pg_types::parse_generate_pg_types(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        )
        .and_then(crate::build_generate_pg_types::build_generate_pg_types)
        .and_then(crate::validate_generate_pg_types::validate_generate_pg_types)
        .is_ok_and(|validated| usize::from(validated.entry_count()) == expected_count)
    }));
}

#[test]
fn test_pg_type_pipeline_preserves_concrete_nullability_validation_failure() {
    let input = quote::quote! {{
        "pg_table_cols_write_into_file": "False",
        "whole_write_into_file": "False",
        "variant": {"Concrete": [
            {"pg_type": "I16AsSmallSerialInitializationByPg", "is_nullable": "True", "pg_type_pattern": "Standard"}
        ]}
    }};
    assert!(
        crate::parse_generate_pg_types::parse_generate_pg_types(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        )
        .is_err_and(|error| {
            matches!(
                error,
                crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError::Parse(_)
            ) && error
                .to_string()
                .contains(constants_str::CANT_SUPPORT_NULLABLE_VARIANTS)
        })
    );
}

#[test]
fn test_pg_type_token_facade_emits_one_compile_error_for_invalid_configurations() {
    assert!([
        quote::quote! {{ "variant": "MissingFields" }},
        quote::quote! { invalid_config },
        quote::quote! {{
            "pg_table_cols_write_into_file": "False",
            "whole_write_into_file": "False",
            "variant": {"Concrete": [
                {"pg_type": "I16AsSmallSerialInitializationByPg", "is_nullable": "True", "pg_type_pattern": "Standard"}
            ]}
        }},
    ].into_iter().all(|input| {
        let generated = crate::generate_pg_types_tokens::generate_pg_types_tokens(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        let tokens = proc_macro2::TokenStream::from(generated).into_iter().collect::<Vec<_>>();
        let [proc_macro2::TokenTree::Ident(identifier), proc_macro2::TokenTree::Punct(bang), proc_macro2::TokenTree::Group(arguments), proc_macro2::TokenTree::Punct(semicolon)] = tokens.as_slice() else { return false; };
        let message_tokens = arguments.stream().into_iter().collect::<Vec<_>>();
        let [proc_macro2::TokenTree::Literal(message)] = message_tokens.as_slice() else { return false; };
        identifier == stringify!(compile_error)
            && bang.as_char() == '!'
            && semicolon.as_char() == ';'
            && arguments.delimiter() == proc_macro2::Delimiter::Parenthesis
            && message.to_string().contains(stringify!(GeneratePgTypesConfig))
    }));
}

#[test]
fn test_pg_type_emitter_rejects_duplicate_concrete_records() {
    let input = quote::quote! {{
        "pg_table_cols_write_into_file": "False",
        "whole_write_into_file": "False",
        "variant": {"Concrete": [
            {"pg_type": "I16AsInt2", "is_nullable": "False", "pg_type_pattern": "Standard"},
            {"pg_type": "I16AsInt2", "is_nullable": "False", "pg_type_pattern": "Standard"}
        ]}
    }};
    let generated = crate::generate_pg_types_tokens::generate_pg_types_tokens(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    );
    let message = constants_str::DUPLICATE_PG_TYPE_CONFIG_ENTRY;
    assert_eq!(
        proc_macro2::TokenStream::from(generated).to_string(),
        quote::quote! { compile_error!(#message); }.to_string()
    );
}

#[test]
fn test_pg_type_subset_expands_nullability_and_deduplicates_in_catalog_order() {
    let subset = quote::quote! {{
        "pg_table_cols_write_into_file": "False",
        "whole_write_into_file": "False",
        "variant": {"Subset": [
            "SqlxTypesUuidUuidAsUuidV4InitializationByPg", "StringAsText",
            "I64AsBigSerialInitializationByPg", "I16AsInt2",
            "I32AsSerialInitializationByPg", "I16AsSmallSerialInitializationByPg", "StringAsText"
        ]}
    }};
    let concrete = quote::quote! {{
        "pg_table_cols_write_into_file": "False",
        "whole_write_into_file": "False",
        "variant": {"Concrete": [
            {"pg_type": "I16AsInt2", "is_nullable": "False", "pg_type_pattern": "Standard"},
            {"pg_type": "I16AsInt2", "is_nullable": "True", "pg_type_pattern": "Standard"},
            {"pg_type": "I16AsSmallSerialInitializationByPg", "is_nullable": "False", "pg_type_pattern": "Standard"},
            {"pg_type": "I32AsSerialInitializationByPg", "is_nullable": "False", "pg_type_pattern": "Standard"},
            {"pg_type": "I64AsBigSerialInitializationByPg", "is_nullable": "False", "pg_type_pattern": "Standard"},
            {"pg_type": "StringAsText", "is_nullable": "False", "pg_type_pattern": "Standard"},
            {"pg_type": "StringAsText", "is_nullable": "True", "pg_type_pattern": "Standard"},
            {"pg_type": "SqlxTypesUuidUuidAsUuidV4InitializationByPg", "is_nullable": "False", "pg_type_pattern": "Standard"}
        ]}
    }};
    let emit = |input| {
        proc_macro2::TokenStream::from(crate::generate_pg_types_tokens::generate_pg_types_tokens(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(input),
        ))
        .to_string()
    };
    let subset_output = emit(&subset);
    assert!(!subset_output.contains(stringify!(compile_error)));
    assert!(subset_output.contains(stringify!(I16AsNonNullInt2)));
    assert!(subset_output.contains(stringify!(StringAsNullableText)));
    assert_eq!(subset_output, emit(&concrete));
}

#[test]
fn test_pg_type_secret_generation_is_opt_in_even_with_empty_type_selection() {
    let cases = [
        (
            quote::quote! {{
                "pg_table_cols_write_into_file": "False",
                "whole_write_into_file": "False",
                "variant": {"Subset": []}
            }},
            false,
        ),
        (
            quote::quote! {{
                "pg_table_cols_write_into_file": "False",
                "whole_write_into_file": "False",
                "generate_secret_text": false,
                "variant": {"Subset": []}
            }},
            false,
        ),
        (
            quote::quote! {{
                "pg_table_cols_write_into_file": "False",
                "whole_write_into_file": "False",
                "generate_secret_text": true,
                "variant": {"Subset": []}
            }},
            true,
        ),
    ];
    let output = cases.map(|(input, expected_secret)| {
        let generated = crate::generate_pg_types_tokens::generate_pg_types_tokens(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        )
        .to_string();
        assert!(!generated.contains(stringify!(compile_error)));
        assert_eq!(
            generated.contains(stringify!(StringAsNonNullTextSecret)),
            expected_secret
        );
        assert_eq!(
            generated.contains(stringify!(StringAsNonNullTextSecretRef)),
            expected_secret
        );
        generated
    });
    let [default_output, disabled_output, enabled_output] = output;
    assert_eq!(default_output, disabled_output);
    assert_ne!(enabled_output, disabled_output);
}
