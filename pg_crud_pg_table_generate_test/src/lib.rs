#![allow(
    unused_crate_dependencies,
    reason = "split proc-macro crates are dependencies of generated fixture crates represented as token streams in this test support crate"
)]

#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct JsonContractValue {
        operation: String,
    }
    fn table_input(token_stream: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        quote::quote! {
            #[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config{{
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            }}]
            #[proc_macro_generate_pg_table_cm_logic::cm_logic{}]
            #[proc_macro_generate_pg_table_rm_logic::rm_logic{}]
            #[proc_macro_generate_pg_table_um_logic::um_logic{}]
            #[proc_macro_generate_pg_table_dm_logic::dm_logic{}]
            #[proc_macro_generate_pg_table_common_logic::common_logic{}]
            pub struct TableExample {
                #[generate_pg_table_primary_key]
                primary_key_column: pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
                #token_stream
            }
        }
    }
    #[test]
    fn test_shared_json_contract_helper_round_trips_table_fixture() {
        macro_helpers::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<
            JsonContractValue,
        >(macro_helpers::json_fixture_ref::JsonFixtureRef::from(
            constants_str::OPERATION_RM,
        ))
        .expect(constants_str::DIAGNOSTIC_F9F9AF71);
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "full table source generation is covered by native generator tests and is prohibitively slow under interpretation"
    )]
    fn test_generated_crud_contract_excludes_single_record_operations() {
        let input = table_input(&quote::quote! {
            column_0: pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2,
        });
        let generated = generate_pg_table_src::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        )
        .to_string();
        assert!(
            generated.contains(
                &quote::quote! {
                    enum TableExampleOperation {
                        CreateMany,
                        DeleteMany,
                        Read,
                        UpdateMany,
                    }
                }
                .to_string()
            )
        );
        assert!(
            [
                quote::quote! { ReadOne },
                quote::quote! { CreateOne },
                quote::quote! { DeleteOne },
                quote::quote! { create_one },
                quote::quote! { delete_one },
                quote::quote! { read_one },
                quote::quote! { ro_logic },
                quote::quote! { ro_error_variants },
            ]
            .iter()
            .all(|token_stream| !generated.contains(&token_stream.to_string()))
        );
    }
    #[test]
    fn test_duplicate_frontend_order_is_rejected_during_generation() {
        let input = table_input(&quote::quote! {
            #[generate_pg_table_frontend(order = 1)]
            column_0: pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2,
            #[generate_pg_table_frontend(order = 1)]
            column_1: pg_types_numeric::generate_pg_types_mod::I32AsNonNullInt4,
        });
        let generated = generate_pg_table_src::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert!(
            generated
                .to_string()
                .contains(constants_str::VALUE_3490DFE2)
        );
    }
    #[test]
    fn test_unknown_frontend_option_is_rejected_during_generation() {
        let input = table_input(&quote::quote! {
            #[generate_pg_table_frontend(unknown)]
            column_0: pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2,
        });
        let generated = generate_pg_table_src::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert!(
            generated
                .to_string()
                .contains(constants_str::VALUE_03D31649)
        );
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "full table source generation is covered by native generator tests and is prohibitively slow under interpretation"
    )]
    fn test_generated_metrics_use_bounded_labels() {
        let input = table_input(&quote::quote! {
            column_0: pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2,
        });
        let generated = generate_pg_table_src::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        )
        .to_string();
        assert!(generated.contains(constants_str::VALUE_7D441FF7));
        assert!(generated.contains(constants_str::VALUE_DAA8E501));
        assert!(generated.contains(constants_str::VALUE_C3E6615B));
        assert!(generated.contains(constants_str::VALUE_F8E6EF62));
        assert!(!generated.contains(constants_str::VALUE_6D06D33A));
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "compiler subprocess validation is covered by the native Clippy gate"
    )]
    fn test_pg_table_generate_clippy() {
        let fixture_dependencies = constants_str::DEPENDENCIES_NEWLINE_APP_STATE_WORKSPACE_TRUE_NEWLINE_AXUM_WORKSPACE_TRUE_NEWLINE_FUTURES
            .replace(
                constants_str::PG_TABLE_MONOLITHIC_WORKSPACE_DEPENDENCY,
                constants_str::PG_TABLE_SPLIT_WORKSPACE_DEPENDENCIES,
            )
            .replace(
                constants_str::LOCATION_MONOLITHIC_WORKSPACE_DEPENDENCY,
                constants_str::LOCATION_DERIVE_WORKSPACE_DEPENDENCY,
            );
        macro_clippy_check_test_common::clippy_check(
            constants_str::GENERATE_PG_TABLE_TEST_CNT,
            constants_str::PG_CRUD_PG_TABLE,
            fixture_dependencies.as_str(),
            &{
                #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                enum AddGeneratePgTablePrimaryKey {
                    False,
                    True,
                }
                #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
                enum FixtureRevisionMode {
                    Disabled,
                    Enabled,
                }
                let allow_clippy_arbitrary_src_item_ordering =
                    token_patterns::AllowClippyArbitrarySrcItemOrdering;
                let generate_table_example_token_stream = |add_generate_pg_table_primary_key: AddGeneratePgTablePrimaryKey, fixture_revision_mode: FixtureRevisionMode| {
                    let maybe_generate_pg_table_primary_key_token_stream = match add_generate_pg_table_primary_key {
                        AddGeneratePgTablePrimaryKey::False => proc_macro2::TokenStream::new(),
                        AddGeneratePgTablePrimaryKey::True => {
                            quote::quote! {#[generate_pg_table_primary_key]}
                        }
                    };
                    let (identifier, revision_config, revision_field) = match fixture_revision_mode {
                        FixtureRevisionMode::Disabled => (quote::quote! { TableExample }, proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
                        FixtureRevisionMode::Enabled => (
                            quote::quote! { RevisionTableExample },
                            quote::quote! { "optimistic_revision_field": "revision", "idempotent_mutations": true, },
                            quote::quote! { revision: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8, },
                        ),
                    };
                    quote::quote! {
                        #allow_clippy_arbitrary_src_item_ordering
                        #[allow(
                            dead_code,
                            reason = "the compile-only macro fixture validates generated contracts without reading its source model"
                        )]
                        #[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                        #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config{{
                            #revision_config
                            "cm_write_into_file": "False",
                            "rm_write_into_file": "False",
                            "um_write_into_file": "False",
                            "dm_write_into_file": "False",
                            "tests_write_into_file": "False",
                            "common_write_into_file": "False",
                            "whole_write_into_file": "False"
                        }}]
                        #[proc_macro_generate_pg_table_common_error_variants::common_error_variants{
                            enum CommonErrorVariants {
                                CheckCommit {
                                    #[eo_location]
                                    check_commit: route_validators::commit_error::CommitError,
                                    location: location_lib::location::Location,
                                },
                            }
                        }]
                        #[proc_macro_generate_pg_table_cm_logic::cm_logic{}]
                        #[proc_macro_generate_pg_table_rm_logic::rm_logic{}]
                        #[proc_macro_generate_pg_table_um_logic::um_logic{}]
                        #[proc_macro_generate_pg_table_dm_logic::dm_logic{}]
                        #[proc_macro_generate_pg_table_common_logic::common_logic{}]
                        pub struct #identifier {
                            #maybe_generate_pg_table_primary_key_token_stream
                            primary_key_column:
                                pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
                            #revision_field
                            column_0: pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2,
                            column_1: pg_types_numeric::generate_pg_types_mod::OptionalI16AsNullableInt2,
                            column_2: pg_types_numeric::generate_pg_types_mod::I32AsNonNullInt4,
                        }
                    }
                };
                [FixtureRevisionMode::Disabled, FixtureRevisionMode::Enabled].into_iter().map(|fixture_revision_mode| {
                    let input = generate_table_example_token_stream(AddGeneratePgTablePrimaryKey::True, fixture_revision_mode);
                    let generated = generate_pg_table_src::generate_pg_table::generate_pg_table(
                        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
                    );
                    let repeated = generate_pg_table_src::generate_pg_table::generate_pg_table(
                        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
                    );
                    assert_eq!(generated.to_string(), repeated.to_string());
                    let table = generate_table_example_token_stream(AddGeneratePgTablePrimaryKey::False, fixture_revision_mode);
                    quote::quote! { #generated #table }
                }).collect::<proc_macro2::TokenStream>()
            }
            .to_string(),
        );
    }
}
