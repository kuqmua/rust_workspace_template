#[cfg(test)]
mod tests {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct JsonContractValue {
        operation: String,
    }
    fn table_input(field_attrs: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        quote::quote! {
            #[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
            #[generate_pg_table::generate_pg_table_config{{
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            }}]
            #[generate_pg_table::cm_logic{}]
            #[generate_pg_table::co_logic{}]
            #[generate_pg_table::rm_logic{}]
            #[generate_pg_table::ro_logic{}]
            #[generate_pg_table::um_logic{}]
            #[generate_pg_table::uo_logic{}]
            #[generate_pg_table::dm_logic{}]
            #[generate_pg_table::dlo_logic{}]
            #[generate_pg_table::common_logic{}]
            pub struct TableExample {
                #[generate_pg_table_primary_key]
                pub primary_key_column: pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
                #field_attrs
            }
        }
    }
    #[test]
    fn shared_json_contract_helper_round_trips_table_fixture() {
        macros_helpers::json_contract::ensure_json_contract_round_trip::<JsonContractValue>(
            macros_helpers::json_contract::JsonFixtureRef::from(str_constants::OPERATION_RM),
        )
        .expect(
            "f9f9af71 shared_json_contract_helper_round_trips_table_fixture invariant must hold",
        );
    }
    #[test]
    fn duplicate_frontend_order_is_rejected_during_generation() {
        let input = table_input(&quote::quote! {
            #[generate_pg_table_frontend(order = 1)]
            pub column_0: pg_types_numeric::I16AsNonNullInt2,
            #[generate_pg_table_frontend(order = 1)]
            pub column_1: pg_types_numeric::I32AsNonNullInt4,
        });
        let generated = generate_pg_table_src::generate_pg_table(
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        );
        assert!(generated.to_string().contains("35d30bd7"));
    }
    #[test]
    fn unknown_frontend_option_is_rejected_during_generation() {
        let input = table_input(&quote::quote! {
            #[generate_pg_table_frontend(unknown)]
            pub column_0: pg_types_numeric::I16AsNonNullInt2,
        });
        let generated = generate_pg_table_src::generate_pg_table(
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        );
        assert!(generated.to_string().contains("bc1d3b08"));
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "full table source generation is covered by native generator tests and is prohibitively slow under interpretation"
    )]
    fn generated_metrics_use_bounded_labels() {
        let input = table_input(&quote::quote! {
            pub column_0: pg_types_numeric::I16AsNonNullInt2,
        });
        let generated = generate_pg_table_src::generate_pg_table(
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        )
        .to_string();
        assert!(generated.contains("pg_table_requests_total"));
        assert!(generated.contains("\"table\" => \"table_example\""));
        assert!(generated.contains("\"status\" => \"409\""));
        assert!(generated.contains("\"status\" => \"425\""));
        assert!(!generated.contains("\"table\" => table_owned"));
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "compiler subprocess validation is covered by the native Clippy gate"
    )]
    fn clippy() {
        macro_clippy_check_common::clippy_check(
            str_constants::GENERATE_PG_TABLE_TEST_CNT,
            str_constants::PG_CRUD_PG_TABLE,
            str_constants::DEPENDENCIES_NEWLINE_APP_STATE_WORKSPACE_TRUE_NEWLINE_AXUM_WORKSPACE_TRUE_NEWLINE_FUTURES,
            &{
                #[derive(optimal_memory_layout::OptimalMemoryLayout)]
                enum AddGeneratePgTablePrimaryKey {
                    False,
                    True,
                }
                let allow_clippy_arbitrary_src_item_ordering =
                    token_patterns::AllowClippyArbitrarySrcItemOrdering;
                let generate_table_example_token_stream = |add_generate_pg_table_primary_key: AddGeneratePgTablePrimaryKey| {
                    let maybe_generate_pg_table_primary_key_token_stream = match add_generate_pg_table_primary_key {
                        AddGeneratePgTablePrimaryKey::False => proc_macro2::TokenStream::new(),
                        AddGeneratePgTablePrimaryKey::True => {
                            quote::quote! {#[generate_pg_table_primary_key]}
                        }
                    };
                    quote::quote! {
                        #allow_clippy_arbitrary_src_item_ordering
                        #[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
                        #[generate_pg_table::generate_pg_table_config{{
                            "cm_write_into_file": "False",
                            "co_write_into_file": "False",
                            "rm_write_into_file": "False",
                            "ro_write_into_file": "False",
                            "um_write_into_file": "False",
                            "uo_write_into_file": "False",
                            "dm_write_into_file": "False",
                            "dlo_write_into_file": "False",
                            "tests_write_into_file": "False",
                            "common_write_into_file": "False",
                            "whole_write_into_file": "False"
                        }}]
                        #[generate_pg_table::common_error_variants{
                            enum CommonErrorVariants {
                                CheckCommit {
                                    #[eo_location]
                                    check_commit: route_validators::check_commit::CommitError,
                                    location: location_lib::location::Location,
                                },
                            }
                        }]
                        #[generate_pg_table::cm_logic{}]
                        #[generate_pg_table::co_logic{}]
                        #[generate_pg_table::rm_logic{}]
                        #[generate_pg_table::ro_logic{}]
                        #[generate_pg_table::um_logic{}]
                        #[generate_pg_table::uo_logic{}]
                        #[generate_pg_table::dm_logic{}]
                        #[generate_pg_table::dlo_logic{}]
                        #[generate_pg_table::common_logic{}]
                        pub struct TableExample {
                            #maybe_generate_pg_table_primary_key_token_stream
                            pub primary_key_column:
                                pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
                            pub column_0: pg_types_numeric::I16AsNonNullInt2,
                            pub column_1: pg_types_numeric::OptionalI16AsNullableInt2,
                            pub column_2: pg_types_numeric::I32AsNonNullInt4,
                        }
                    }
                };
                let generate_pg_table_input_token_stream = generate_table_example_token_stream(AddGeneratePgTablePrimaryKey::True);
                let ts = generate_pg_table_src::generate_pg_table(
                    macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&generate_pg_table_input_token_stream),
                );
                let repeated_token_stream = generate_pg_table_src::generate_pg_table(
                    macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&generate_pg_table_input_token_stream),
                );
                assert_eq!(ts.to_string(), repeated_token_stream.to_string());
                let table_struct_token_stream = generate_table_example_token_stream(AddGeneratePgTablePrimaryKey::False);
                quote::quote! {
                    #ts
                    #table_struct_token_stream
                }
            }
            .to_string(),
        );
    }
}
