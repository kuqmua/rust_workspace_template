#[cfg(test)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::needless_for_each)] // table-driven assertions avoid repository-forbidden for loops
mod tests {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct JsonContractValue {
        value: Vec<i32>,
    }
    #[test]
    fn shared_json_contract_helper_round_trips_filter_fixture() {
        macro_helpers::domain_types::json_contract::ensure_json_contract_round_trip::<
            JsonContractValue,
        >(
            macro_helpers::domain_types::json_contract::JsonFixtureRef::from(
                constants_str::VALUE_1_2,
            ),
        )
        .expect(
            "46f3bec1 shared_json_contract_helper_round_trips_filter_fixture invariant must hold",
        );
    }
    #[test]
    fn text_search_patterns_escape_reserved_symbols_for_every_mode() {
        let cases = [
            (
                where_filters::domain_types::TextSearchMode::Contains,
                constants_str::PERCENT_A_PERCENT_B_PERCENT,
            ),
            (
                where_filters::domain_types::TextSearchMode::StartsWith,
                constants_str::A_PERCENT_B_PERCENT,
            ),
            (
                where_filters::domain_types::TextSearchMode::EndsWith,
                constants_str::PERCENT_A_PERCENT_B,
            ),
        ];
        cases.into_iter().for_each(|(mode, expected)| {
            let pattern =
                where_filters::domain_types::build_text_search_pattern(constants_str::A_PERCENT_B, mode)
                    .expect("bfcd929a text_search_patterns_escape_reserved_symbols_for_every_mode invariant must hold");
            assert_eq!(pattern.as_ref(), expected);
        });
    }
    #[test]
    fn text_search_rejects_empty_and_oversized_values() {
        assert_eq!(
            where_filters::domain_types::build_text_search_pattern(
                "",
                where_filters::domain_types::TextSearchMode::Contains
            ),
            Err(where_filters::domain_types::TextSearchValueError::Empty)
        );
        let oversized = constants_str::A_ALT.repeat(
            usize::from(
                where_filters::domain_types::TextSearchPolicy::DEFAULT.maximum_input_bytes(),
            )
            .saturating_add(constants_usize::ONE),
        );
        assert_eq!(
            where_filters::domain_types::build_text_search_pattern(
                oversized.as_str(),
                where_filters::domain_types::TextSearchMode::Contains
            ),
            Err(where_filters::domain_types::TextSearchValueError::TooLong {
                actual_bytes: oversized.len(),
                maximum_bytes: usize::from(
                    where_filters::domain_types::TextSearchPolicy::DEFAULT.maximum_input_bytes(),
                ),
            })
        );
    }
    #[test]
    fn text_search_query_fragment_uses_ilike_escape_and_ordered_placeholder() {
        let filter = where_filters::domain_types::PgTypeWhereTextSearch::try_new(
            pg_crud_common::domain_types::Operator::And,
            where_filters::domain_types::TextSearchMode::Contains,
            constants_str::LITERAL_PERCENT_VALUE.to_owned(),
        )
        .expect("20d018ab text_search_query_fragment_uses_ilike_escape_and_ordered_placeholder invariant must hold");
        let mut parameter_index = 4u64;
        let column = constants_str::DISPLAY_NAME.to_owned();
        let fragment = <where_filters::domain_types::PgTypeWhereTextSearch as pg_crud_common::domain_types::PgTypeWhereFilter>::query_part(
            &filter,
            &mut parameter_index,
            pg_crud_common::domain_types::SqlColumnRef::from(&column),
            pg_crud_common::domain_types::AddOperator::from(true),
        )
        .expect("509f61f8 text_search_query_fragment_uses_ilike_escape_and_ordered_placeholder invariant must hold");
        assert_eq!(fragment.as_ref(), "and display_name ILIKE $5 ESCAPE '\\'");
        assert_eq!(parameter_index, 5u64);
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "compiler subprocess validation is covered by the native Clippy gate"
    )]
    fn where_filters_generate_clippy() {
        macro_clippy_check_common::clippy_check(
            constants_str::GENERATE_WHERE_FLTS_TEST_CNT,
            constants_str::PG_CRUD_WHERE_FILTERS,
            constants_str::DEPENDENCIES_NEWLINE_SQLX_WORKSPACE_TRUE_NEWLINE_SERDE_WORKSPACE_TRUE_NEWLINE_SCHEMARS_WORKSPACE,
            &format!(
                "#![allow(dead_code)]\n#![allow(unreachable_pub)]\n#![allow(unused_imports)]\n#[allow(clippy::wildcard_imports)]\nuse where_filters::domain_types::*;\n{}",
                generate_where_filters_src::domain_types::source::generate_where_filters_source(
                    generate_where_filters_src::domain_types::source::ProcMacro2GenerateWhereFiltersInput::from(
                        &quote::quote! {
                            {
                                "pg_types_write_into_file": "False",
                                "whole_write_into_file": "False"
                            }
                        }
                    )
                )
            ),
        );
    }
}
