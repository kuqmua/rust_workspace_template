pub(crate) fn generate_pg_table_measure_input_token_stream(
    tests_write_into_file: &dyn quote::ToTokens,
) -> crate::quote_token_stream_generate_pg_table_measure_input_token_stream::QuoteTokenStreamGeneratePgTableMeasureInputTokenStream{
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    crate::quote_token_stream_generate_pg_table_measure_input_token_stream::QuoteTokenStreamGeneratePgTableMeasureInputTokenStream::from(
        quote::quote! {
            #allow_clippy_arbitrary_src_item_ordering
            #[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config{{
                "cm_write_into_file": "False",
                "co_write_into_file": "False",
                "rm_write_into_file": "False",
                "ro_write_into_file": "False",
                "um_write_into_file": "False",
                "uo_write_into_file": "False",
                "dm_write_into_file": "False",
                "dlo_write_into_file": "False",
                "tests_write_into_file": #tests_write_into_file,
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
            #[proc_macro_generate_pg_table_co_logic::co_logic{}]
            #[proc_macro_generate_pg_table_rm_logic::rm_logic{}]
            #[proc_macro_generate_pg_table_ro_logic::ro_logic{}]
            #[proc_macro_generate_pg_table_um_logic::um_logic{}]
            #[proc_macro_generate_pg_table_uo_logic::uo_logic{}]
            #[proc_macro_generate_pg_table_dm_logic::dm_logic{}]
            #[proc_macro_generate_pg_table_dlo_logic::dlo_logic{}]
            #[proc_macro_generate_pg_table_common_logic::common_logic{}]
            pub struct TableExample {
                #[generate_pg_table_primary_key]
                primary_key_column: pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
                column_0: pg_types_numeric::generate_pg_types_mod::I16AsNonNullInt2,
                column_1: pg_types_numeric::generate_pg_types_mod::OptionalI16AsNullableInt2,
                column_2: pg_types_numeric::generate_pg_types_mod::I32AsNonNullInt4,
            }
        },
    )
}
