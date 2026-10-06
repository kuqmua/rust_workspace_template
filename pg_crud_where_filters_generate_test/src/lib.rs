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
        assert!([
            pg_crud_common::operator::Operator::And,
            pg_crud_common::operator::Operator::AndNot,
            pg_crud_common::operator::Operator::Or,
            pg_crud_common::operator::Operator::OrNot,
        ].into_iter().all(|operator| {
            [false, true].into_iter().all(|add_operator| {
                [0u64, 4u64, u64::MAX].into_iter().all(|initial_index| {
                    where_filters::domain_types::PgTypeWhereTextSearch::try_new(
                        operator,
                        where_filters::domain_types::TextSearchMode::Contains,
                        constants_str::LITERAL_PERCENT_VALUE.to_owned(),
                    ).is_ok_and(|pg_type_where_text_search| {
                        let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
                        let result = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                            &pg_type_where_text_search,
                            &mut increment,
                            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                            pg_crud_common::add_operator::AddOperator::from(add_operator),
                        );
                        match initial_index.checked_add(1u64) {
                            Some(expected_index) => increment.get() == expected_index && result.is_ok_and(|query_part_fragment| {
                                let prefix = operator.to_query_part(pg_crud_common::add_operator::AddOperator::from(add_operator));
                                query_part_fragment.as_ref().strip_prefix(prefix.as_ref()).is_some_and(|text| {
                                    let mut actual = text.split_whitespace();
                                    let mut expected = constants_str::VALUE_BA922EFF.split_whitespace().skip(1usize);
                                    actual.next() == expected.next()
                                        && actual.next() == expected.next()
                                        && actual.next().and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == Some(expected_index)
                                        && actual.eq(expected.skip(1usize))
                                })
                            }),
                            None => increment.get() == initial_index && matches!(result, Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })),
                        }
                    })
                })
            })
        }));
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
    fn test_range_relationship_filters_preserve_operator_and_placeholder_order() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        let mut parameter_index = 0u64;
        assert!([
            (
                <where_filters::domain_types::PgTypeWhereFindRangesWithinGivenRange<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                    &where_filters::domain_types::PgTypeWhereFindRangesWithinGivenRange::new(pg_crud_common::operator::Operator::And, 1i32),
                    &mut parameter_index,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                ),
                constants_str::PG_CRUD_WITHIN_SQL_OPERATOR,
                1u64,
            ),
            (
                <where_filters::domain_types::PgTypeWhereFindRangesThatFullyContainTheGivenRange<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                    &where_filters::domain_types::PgTypeWhereFindRangesThatFullyContainTheGivenRange::new(pg_crud_common::operator::Operator::And, 1i32),
                    &mut parameter_index,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                ),
                constants_str::PG_CRUD_CONTAINS_SQL_OPERATOR,
                2u64,
            ),
            (
                <where_filters::domain_types::PgTypeWhereOverlapWithRange<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                    &where_filters::domain_types::PgTypeWhereOverlapWithRange::new(pg_crud_common::operator::Operator::And, 1i32),
                    &mut parameter_index,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                ),
                constants_str::PG_CRUD_OVERLAPS_SQL_OPERATOR,
                3u64,
            ),
            (
                <where_filters::domain_types::PgTypeWhereAdjacentWithRange<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                    &where_filters::domain_types::PgTypeWhereAdjacentWithRange::new(pg_crud_common::operator::Operator::And, 1i32),
                    &mut parameter_index,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                ),
                constants_str::PG_CRUD_ADJACENT_SQL_OPERATOR,
                4u64,
            ),
        ].into_iter().all(|(result, operator, expected_index)| {
            result.is_ok_and(|fragment| {
                let mut parts = fragment.as_ref().split_whitespace();
                parts.next().is_some_and(|part| part.strip_prefix('(') == Some(column.as_str()))
                    && parts.next() == Some(operator)
                    && parts.next().is_some_and(|part| {
                        part.strip_suffix(')').and_then(|placeholder| placeholder.strip_prefix('$'))
                            .and_then(|index| index.parse::<u64>().ok()) == Some(expected_index)
                    })
                    && parts.next().is_none()
            })
        }));
        assert_eq!(parameter_index, 4u64);
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
    fn test_encoded_string_filter_formats_preserve_sql_and_overflow_counter() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        let single_quote = char::from(39u8);
        assert!([
            (where_filters::encode_format::EncodeFormat::Base64, constants_str::VALUE_371A286D),
            (where_filters::encode_format::EncodeFormat::Escape, constants_str::VALUE_B3140286),
            (where_filters::encode_format::EncodeFormat::Hex, constants_str::VALUE_128DF13C),
        ].into_iter().all(|(format, expected_format)| {
            [0u64, 4u64, u64::MAX].into_iter().all(|initial_index| {
                let filter = where_filters::domain_types::PgTypeWhereEqToEncodedStringRepresentation::new(
                    pg_crud_common::operator::Operator::And, format, constants_str::TEST_SQL_INJECTION.to_owned(),
                );
                let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
                let result = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &filter, &mut increment, pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                );
                match initial_index.checked_add(1u64) {
                    Some(expected_index) => increment.get() == expected_index && result.is_ok_and(|fragment| {
                        let text = fragment.as_ref();
                        let mut parts = text.split_whitespace();
                        parts.next().and_then(|part| part.strip_prefix('(')).and_then(|part| part.split_once('(')).is_some_and(|(function, argument)| {
                            function == stringify!(encode) && argument.strip_suffix(',') == Some(column.as_str())
                        })
                            && parts.next().and_then(|part| part.strip_prefix(single_quote)).and_then(|part| part.strip_suffix(')')).and_then(|part| part.strip_suffix(single_quote)) == Some(expected_format)
                            && parts.next() == Some(constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR)
                            && parts.next().and_then(|part| part.strip_suffix(')')).and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == Some(expected_index)
                            && parts.next().is_none()
                            && !text.contains(constants_str::TEST_SQL_INJECTION)
                    }),
                    None => increment.get() == initial_index && matches!(result, Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })),
                }
            })
        }));
    }
    #[test]
    fn test_regex_filter_case_modes_preserve_sql_and_overflow_counter() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        assert!([
            (where_filters::regex_case::RegexCase::Sensitive, constants_str::TEXT_ALT_15),
            (where_filters::regex_case::RegexCase::Insensitive, constants_str::ASTERISK_ALT),
        ].into_iter().all(|(case, operator)| {
            [0u64, 4u64, u64::MAX].into_iter().all(|initial_index| {
                where_filters::regex_regex::RegexRegex::try_from(constants_str::A_Z_PLUS.to_owned()).is_ok_and(|pattern| {
                    let filter = where_filters::domain_types::PgTypeWhereRegex::new(pg_crud_common::operator::Operator::And, case, pattern);
                    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
                    let result = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                        &filter, &mut increment, pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                        pg_crud_common::add_operator::AddOperator::from(false),
                    );
                    match initial_index.checked_add(1u64) {
                        Some(expected_index) => increment.get() == expected_index && result.is_ok_and(|fragment| {
                            let mut parts = fragment.as_ref().split_whitespace();
                            parts.next().and_then(|part| part.strip_prefix('(')) == Some(column.as_str())
                                && parts.next() == Some(operator)
                                && parts.next().and_then(|part| part.strip_suffix(')')).and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == Some(expected_index)
                                && parts.next().is_none()
                        }),
                        None => increment.get() == initial_index && matches!(result, Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })),
                    }
                })
            })
        }));
    }
    #[test]
    fn test_scalar_comparison_filters_preserve_sql_and_overflow_counter() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        assert!([0u64, 4u64, u64::MAX].into_iter().all(|initial_index| {
            let mut greater_increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
            let mut before_increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
            let valid = [
                (
                    <where_filters::domain_types::PgTypeWhereGreaterThan<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                        &where_filters::domain_types::PgTypeWhereGreaterThan::new(pg_crud_common::operator::Operator::And, 1i32),
                        &mut greater_increment,
                        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                        pg_crud_common::add_operator::AddOperator::from(false),
                    ),
                    constants_str::TEXT_ALT_11,
                ),
                (
                    <where_filters::domain_types::PgTypeWhereBefore<i32> as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                        &where_filters::domain_types::PgTypeWhereBefore::new(pg_crud_common::operator::Operator::And, 1i32),
                        &mut before_increment,
                        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                        pg_crud_common::add_operator::AddOperator::from(false),
                    ),
                    constants_str::PG_CRUD_BEFORE_SQL_OPERATOR,
                ),
            ].into_iter().all(|(result, operator)| match initial_index.checked_add(1u64) {
                Some(expected_index) => result.is_ok_and(|fragment| {
                    let mut parts = fragment.as_ref().split_whitespace();
                    parts.next().and_then(|part| part.strip_prefix('(')) == Some(column.as_str())
                        && parts.next() == Some(operator)
                        && parts.next().and_then(|part| part.strip_suffix(')')).and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == Some(expected_index)
                        && parts.next().is_none()
                }),
                None => matches!(result, Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })),
            });
            valid && [greater_increment, before_increment].into_iter().all(|increment| {
                increment.get() == initial_index.checked_add(1u64).unwrap_or(initial_index)
            })
        }));
    }
    #[test]
    fn test_generated_between_filter_preserves_sql_and_partial_overflow() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        assert!([0u64, 4u64, u64::MAX - 1u64, u64::MAX].into_iter().all(|initial_index| {
            where_filters::between::Between::try_new(1i32, 2i32).is_ok_and(|between| {
                let filter = where_filters::domain_types::PgTypeWhereBetween::new(pg_crud_common::operator::Operator::And, between);
                let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
                let result = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &filter, &mut increment, pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                );
                match initial_index.checked_add(2u64) {
                    Some(final_index) => increment.get() == final_index && result.is_ok_and(|fragment| {
                        let text = fragment.as_ref();
                        text.split_whitespace().next().and_then(|part| part.strip_prefix('(')) == Some(column.as_str())
                            && text.split_whitespace().count() == 5usize
                            && text.split_whitespace().nth(1usize) == Some(constants_str::ADMIN_FILTER_OPERATION_BETWEEN)
                            && text.split_whitespace().nth(2usize).and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == initial_index.checked_add(1u64)
                            && text.split_whitespace().nth(3usize) == Some(constants_str::AND.trim())
                            && text.split_whitespace().nth(4usize).and_then(|part| part.strip_suffix(')')).and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == Some(final_index)
                    }),
                    None => increment.get() == u64::MAX && matches!(result, Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })),
                }
            })
        }));
    }
    #[test]
    fn test_generated_in_filter_preserves_list_sql_and_partial_overflow() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        assert!([0u64, 4u64, u64::MAX - 1u64, u64::MAX].into_iter().all(|initial_index| {
            where_filters::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::try_from(vec![1i32, 2i32, 3i32]).is_ok_and(|values| {
                let filter = where_filters::domain_types::PgTypeWhereIn::new(pg_crud_common::operator::Operator::And, values);
                let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(initial_index);
                let result = pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &filter, &mut increment, pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                );
                match initial_index.checked_add(3u64) {
                    Some(final_index) => increment.get() == final_index && result.is_ok_and(|fragment| {
                        let text = fragment.as_ref();
                        text.split_whitespace().next().and_then(|part| part.strip_prefix('(')) == Some(column.as_str())
                            && text.split_whitespace().count() == 3usize
                            && text.split_whitespace().nth(1usize) == Some(constants_str::ADMIN_FILTER_OPERATION_IN)
                            && text.split_whitespace().nth(2usize).and_then(|part| part.strip_prefix('(')).and_then(|part| part.strip_suffix(')')).and_then(|part| part.strip_suffix(')')).is_some_and(|list| {
                                let mut placeholders = list.split(',');
                                [1u64, 2u64, 3u64].into_iter().all(|offset| {
                                    placeholders.next().and_then(|part| part.strip_prefix('$')).and_then(|part| part.parse::<u64>().ok()) == initial_index.checked_add(offset)
                                }) && placeholders.next().is_none()
                            })
                    }),
                    None => increment.get() == u64::MAX && matches!(result, Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })),
                }
            })
        }));
    }
    #[test]
    fn test_date_and_timestamp_filters_preserve_sql_and_parameter_counter() {
        let column = constants_str::DISPLAY_NAME.to_owned();
        assert!([0u64, 17u64].into_iter().all(|initial_index| {
            let mut parameter_index = initial_index;
            [
                (
                    <where_filters::domain_types::PgTypeWhereCurrentDate as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                        &where_filters::domain_types::PgTypeWhereCurrentDate::new(pg_crud_common::operator::Operator::And),
                        &mut parameter_index,
                        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                        pg_crud_common::add_operator::AddOperator::from(false),
                    ),
                    constants_str::CURRENT_DATE,
                ),
                (
                    <where_filters::domain_types::PgTypeWhereGreaterThanCurrentDate as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                        &where_filters::domain_types::PgTypeWhereGreaterThanCurrentDate::new(pg_crud_common::operator::Operator::And),
                        &mut parameter_index,
                        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                        pg_crud_common::add_operator::AddOperator::from(false),
                    ),
                    constants_str::CURRENT_DATE_ALT,
                ),
                (
                    <where_filters::domain_types::PgTypeWhereCurrentTimestamp as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                        &where_filters::domain_types::PgTypeWhereCurrentTimestamp::new(pg_crud_common::operator::Operator::And),
                        &mut parameter_index,
                        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                        pg_crud_common::add_operator::AddOperator::from(false),
                    ),
                    constants_str::CURRENT_TIMESTAMP,
                ),
                (
                    <where_filters::domain_types::PgTypeWhereGreaterThanCurrentTimestamp as pg_crud_common::pg_type_where_filter::PgTypeWhereFilter>::query_part(
                        &where_filters::domain_types::PgTypeWhereGreaterThanCurrentTimestamp::new(pg_crud_common::operator::Operator::And),
                        &mut parameter_index,
                        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                        pg_crud_common::add_operator::AddOperator::from(false),
                    ),
                    constants_str::CURRENT_TIMESTAMP_ALT,
                ),
            ].into_iter().all(|(result, syntax)| {
                result.is_ok_and(|fragment| {
                    fragment.as_ref().strip_prefix('(')
                        .and_then(|text| text.strip_prefix(column.as_str()))
                        .and_then(|text| text.strip_suffix(')'))
                        .is_some_and(|text| text.trim_start() == syntax)
                })
            }) && parameter_index == initial_index
        }));
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
