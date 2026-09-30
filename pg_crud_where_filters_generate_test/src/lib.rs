#[cfg(test)]
#[allow(
    clippy::needless_for_each,
    reason = "lib uses iterator traversal to comply with the workspace no-for-loop policy"
)]
mod tests {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct JsonContractValue {
        value: Vec<i32>,
    }
    #[test]
    fn test_shared_json_contract_helper_round_trips_filter_fixture() {
        macro_helpers::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<
            JsonContractValue,
        >(macro_helpers::json_fixture_ref::JsonFixtureRef::from(
            constants_str::VALUE_1_2,
        ))
        .expect(constants_str::DIAGNOSTIC_46F3BEC1);
    }
    #[test]
    fn test_text_search_patterns_escape_reserved_symbols_for_every_mode() {
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
            let pattern = where_filters::domain_types::build_text_search_pattern(
                constants_str::A_PERCENT_B,
                mode,
            )
            .expect(constants_str::DIAGNOSTIC_BFCD929A);
            assert_eq!(pattern.as_ref(), expected);
        });
    }
    #[test]
    fn test_text_search_rejects_empty_and_oversized_values() {
        assert_eq!(
            where_filters::domain_types::build_text_search_pattern(
                constants_str::EMPTY,
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
    fn test_text_search_deserialization_rejects_empty_value() {
        assert!(matches!(
            macro_helpers::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<
                where_filters::domain_types::PgTypeWhereTextSearch,
            >(macro_helpers::json_fixture_ref::JsonFixtureRef::from(
                constants_str::PG_CRUD_TEXT_SEARCH_INVALID_EMPTY_JSON,
            )),
            Err(macro_helpers::contract_error::ContractError::DeserializeFixture(_))
        ));
    }
    #[test]
    fn test_text_search_valid_json_contract_round_trips() {
        assert!(
            macro_helpers::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<
                where_filters::domain_types::PgTypeWhereTextSearch,
            >(macro_helpers::json_fixture_ref::JsonFixtureRef::from(
                constants_str::PG_CRUD_TEXT_SEARCH_VALID_JSON,
            ))
            .ok()
            .is_some()
        );
    }
    #[test]
    fn test_text_search_deserialization_rejects_oversized_value() {
        let oversized = constants_str::A_ALT.repeat(
            usize::from(
                where_filters::domain_types::TextSearchPolicy::DEFAULT.maximum_input_bytes(),
            ) + constants_usize::ONE,
        );
        let mut quoted = String::with_capacity(oversized.len() + 2usize);
        quoted.push('"');
        quoted.push_str(oversized.as_str());
        quoted.push('"');
        let fixture = constants_str::PG_CRUD_TEXT_SEARCH_VALID_JSON
            .replace(constants_str::PG_CRUD_QUOTED_A, quoted.as_str());
        assert!(matches!(
            macro_helpers::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<
                where_filters::domain_types::PgTypeWhereTextSearch,
            >(macro_helpers::json_fixture_ref::JsonFixtureRef::from(
                fixture.as_str()
            )),
            Err(macro_helpers::contract_error::ContractError::DeserializeFixture(_))
        ));
    }
    #[test]
    fn test_text_search_query_fragment_uses_ilike_escape_and_ordered_placeholder() {
        let filter = where_filters::domain_types::PgTypeWhereTextSearch::try_new(
            pg_crud_common::operator::Operator::And,
            where_filters::domain_types::TextSearchMode::Contains,
            constants_str::LITERAL_PERCENT_VALUE.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_20D018AB);
        let mut parameter_index = 4u64;
        let column = constants_str::DISPLAY_NAME.to_owned();
        let fragment = <where_filters::domain_types::PgTypeWhereTextSearch as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &filter,
            &mut parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        )
        .expect(constants_str::DIAGNOSTIC_509F61F8);
        assert_eq!(fragment.as_ref(), constants_str::VALUE_BA922EFF);
        assert_eq!(parameter_index, 5u64);
    }
    #[test]
    fn test_strict_range_filters_use_strict_postgres_operators() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        let left = where_filters::domain_types::PgTypeWhereStrictlyToLeftOfRange::<i32>::new(
            pg_crud_common::operator::Operator::And,
            1i32,
        );
        let right = where_filters::domain_types::PgTypeWhereStrictlyToRightOfRange::<i32>::new(
            pg_crud_common::operator::Operator::And,
            1i32,
        );
        let mut left_parameter_index = 0u64;
        let mut right_parameter_index = 0u64;
        let left_fragment = <where_filters::domain_types::PgTypeWhereStrictlyToLeftOfRange<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &left,
            &mut left_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        let right_fragment = <where_filters::domain_types::PgTypeWhereStrictlyToRightOfRange<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &right,
            &mut right_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        assert!(left_fragment.is_ok_and(|fragment| {
            fragment
                .as_ref()
                .contains(constants_str::PG_CRUD_STRICTLY_LEFT_SQL_OPERATOR)
        }));
        assert!(right_fragment.is_ok_and(|fragment| {
            fragment
                .as_ref()
                .contains(constants_str::PG_CRUD_STRICTLY_RIGHT_SQL_OPERATOR)
        }));
    }
    #[test]
    fn test_range_bound_filters_check_inclusivity() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        let lower = where_filters::domain_types::PgTypeWhereIncludedLowerBound::<i32>::new(
            pg_crud_common::operator::Operator::And,
            1i32,
        );
        let upper = where_filters::domain_types::PgTypeWhereExcludedUpperBound::<i32>::new(
            pg_crud_common::operator::Operator::And,
            1i32,
        );
        let mut lower_parameter_index = 0u64;
        let mut upper_parameter_index = 0u64;
        let lower_fragment = <where_filters::domain_types::PgTypeWhereIncludedLowerBound<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &lower,
            &mut lower_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        let upper_fragment = <where_filters::domain_types::PgTypeWhereExcludedUpperBound<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &upper,
            &mut upper_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        assert!(
            lower_fragment
                .is_ok_and(|fragment| fragment.as_ref().contains(constants_str::LOWER_INC))
        );
        assert!(
            upper_fragment
                .is_ok_and(|fragment| fragment.as_ref().contains(constants_str::NOT_UPPER_INC))
        );
        assert_eq!(lower_parameter_index, 1u64);
        assert_eq!(upper_parameter_index, 1u64);
    }
    #[test]
    fn test_greater_than_range_bound_filters_check_inclusivity() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        let lower =
            where_filters::domain_types::PgTypeWhereGreaterThanIncludedLowerBound::<i32>::new(
                pg_crud_common::operator::Operator::And,
                1i32,
            );
        let upper =
            where_filters::domain_types::PgTypeWhereGreaterThanExcludedUpperBound::<i32>::new(
                pg_crud_common::operator::Operator::And,
                1i32,
            );
        let mut lower_parameter_index = 0u64;
        let mut upper_parameter_index = 0u64;
        let lower_fragment = <where_filters::domain_types::PgTypeWhereGreaterThanIncludedLowerBound<
            i32,
        > as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &lower,
            &mut lower_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        let upper_fragment = <where_filters::domain_types::PgTypeWhereGreaterThanExcludedUpperBound<
            i32,
        > as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &upper,
            &mut upper_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        assert!(lower_fragment.is_ok_and(|fragment| {
            fragment.as_ref().contains(constants_str::LOWER_INC)
                && fragment.as_ref().contains(constants_str::TEXT_ALT_11)
        }));
        assert!(upper_fragment.is_ok_and(|fragment| {
            fragment.as_ref().contains(constants_str::NOT_UPPER_INC)
                && fragment.as_ref().contains(constants_str::TEXT_ALT_11)
        }));
        assert_eq!(lower_parameter_index, 1u64);
        assert_eq!(upper_parameter_index, 1u64);
    }
    #[test]
    fn test_range_length_accepts_integer_and_interval_value_types() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        let integer_filter = where_filters::domain_types::PgTypeWhereRangeLen::<
            pg_crud_common::pg_numeric_range_length::PgNumericRangeLength,
        >::new(
            pg_crud_common::operator::Operator::And,
            pg_crud_common::pg_numeric_range_length::PgNumericRangeLength::default(),
        );
        let interval_filter = where_filters::domain_types::PgTypeWhereRangeLen::<
            pg_crud_common::std_duration_range_length::StdDurationRangeLength,
        >::new(
            pg_crud_common::operator::Operator::And,
            pg_crud_common::std_duration_range_length::StdDurationRangeLength::default(),
        );
        let mut integer_parameter_index = 0u64;
        let mut interval_parameter_index = 0u64;
        let integer_fragment = <where_filters::domain_types::PgTypeWhereRangeLen<
            pg_crud_common::pg_numeric_range_length::PgNumericRangeLength,
        > as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &integer_filter,
            &mut integer_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        let interval_fragment = <where_filters::domain_types::PgTypeWhereRangeLen<
            pg_crud_common::std_duration_range_length::StdDurationRangeLength,
        > as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &interval_filter,
            &mut interval_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        assert!(integer_fragment.is_ok_and(|fragment| {
            fragment.as_ref().contains(constants_str::UPPER)
                && fragment.as_ref().contains(constants_str::LOWER)
                && fragment
                    .as_ref()
                    .matches(constants_str::PG_CRUD_NUMERIC_SQL_CAST)
                    .count()
                    == 2usize
        }));
        assert!(interval_fragment.is_ok_and(|fragment| {
            fragment.as_ref().contains(constants_str::UPPER)
                && fragment.as_ref().contains(constants_str::LOWER)
                && !fragment
                    .as_ref()
                    .contains(constants_str::PG_CRUD_NUMERIC_SQL_CAST)
        }));
        assert_eq!(integer_parameter_index, 1u64);
        assert_eq!(interval_parameter_index, 1u64);
    }
    #[test]
    fn test_current_time_filters_use_time_without_time_zone() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        let current = where_filters::domain_types::PgTypeWhereCurrentTime::new(
            pg_crud_common::operator::Operator::And,
        );
        let greater = where_filters::domain_types::PgTypeWhereGreaterThanCurrentTime::new(
            pg_crud_common::operator::Operator::And,
        );
        let mut current_parameter_index = 0u64;
        let mut greater_parameter_index = 0u64;
        let current_fragment = <where_filters::domain_types::PgTypeWhereCurrentTime as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &current,
            &mut current_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        let greater_fragment = <where_filters::domain_types::PgTypeWhereGreaterThanCurrentTime as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
            &greater,
            &mut greater_parameter_index,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            pg_crud_common::add_operator::AddOperator::from(true),
        );
        assert!(
            current_fragment
                .is_ok_and(|fragment| fragment.as_ref().contains(constants_str::LOCALTIME))
        );
        assert!(
            greater_fragment
                .is_ok_and(|fragment| fragment.as_ref().contains(constants_str::LOCALTIME_ALT))
        );
        assert_eq!(current_parameter_index, 0u64);
        assert_eq!(greater_parameter_index, 0u64);
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "compiler subprocess validation is covered by the native Clippy gate"
    )]
    fn test_where_filters_generate_clippy() {
        let fixture_dependencies = constants_str::DEPENDENCIES_NEWLINE_SQLX_WORKSPACE_TRUE_NEWLINE_SERDE_WORKSPACE_TRUE_NEWLINE_SCHEMARS_WORKSPACE
            .replace(
                constants_str::LOCATION_MONOLITHIC_WORKSPACE_DEPENDENCY,
                constants_str::LOCATION_DERIVE_WORKSPACE_DEPENDENCY,
            )
            .replace(
                constants_str::NEWTYPE_MONOLITHIC_WORKSPACE_DEPENDENCY,
                constants_str::NEWTYPE_SPLIT_WORKSPACE_DEPENDENCIES,
            );
        macro_clippy_check_test_common::clippy_check(
            constants_str::GENERATE_WHERE_FILTERS_TEST_CONTENT,
            constants_str::PG_CRUD_WHERE_FILTERS,
            fixture_dependencies.as_str(),
            &format!(
                "#![allow(dead_code, reason = \"lib declares fixture or generated API members exercised outside ordinary reachability analysis\")]\n#![allow(unreachable_pub, reason = \"lib declares fixture or generated API members exercised outside ordinary reachability analysis\")]\n#![allow(unused_imports, reason = \"lib declares fixture or generated API members exercised outside ordinary reachability analysis\")]\n#[allow(clippy::wildcard_imports, reason = \"lib declares fixture or generated API members exercised outside ordinary reachability analysis\")]\nuse where_filters::domain_types::*;\nuse where_filters::between;\nuse where_filters::encode_format;\nuse where_filters::pg_type_not_empty_unique_vec;\nuse where_filters::regex_case;\nuse where_filters::regex_regex;\n{}",
                generate_where_filters_src::generate_where_filters_source::generate_where_filters_source(
                    generate_where_filters_src::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(
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
