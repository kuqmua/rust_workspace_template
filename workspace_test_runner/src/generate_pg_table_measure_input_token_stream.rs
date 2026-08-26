pub(crate) fn generate_pg_table_measure_input_token_stream(
    tests_write_into_file: &dyn quote::ToTokens,
) -> crate::domain_types::QuoteTokenStreamGeneratePgTableMeasureInputTokenStream {
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    crate::domain_types::QuoteTokenStreamGeneratePgTableMeasureInputTokenStream::from(
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
                "tests_write_into_file": #tests_write_into_file,
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            }}]
            #[generate_pg_table::common_error_variants{
                enum CommonErrorVariants {
                    CheckCommit {
                        #[eo_location]
                        check_commit: route_validators::domain_types::check_commit::CommitError,
                        location: location_lib::domain_types::Location,
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
                #[generate_pg_table_primary_key]
                pub primary_key_column: pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
                pub column_0: pg_types_numeric::I16AsNonNullInt2,
                pub column_1: pg_types_numeric::OptionalI16AsNullableInt2,
                pub column_2: pg_types_numeric::I32AsNonNullInt4,
            }
        },
    )
}
