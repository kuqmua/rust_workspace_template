pub fn build_sql_like_pattern(
    sql_like_input_ref: crate::sql_like_input_ref::SqlLikeInputRef<'_>,
    sql_like_match_mode: crate::sql_like_match_mode::SqlLikeMatchMode,
) -> Result<
    crate::sql_like_pattern::SqlLikePattern,
    crate::sql_like_pattern_error::SqlLikePatternError,
> {
    let wildcard_count = match sql_like_match_mode {
        crate::sql_like_match_mode::SqlLikeMatchMode::Contains => 2usize,
        crate::sql_like_match_mode::SqlLikeMatchMode::EndsWith
        | crate::sql_like_match_mode::SqlLikeMatchMode::StartsWith => constants_usize::ONE,
    };
    let input_value = sql_like_input_ref.get();
    let reserved_count = input_value
        .bytes()
        .filter(|byte| matches!(byte, b'\\' | b'%' | b'_'))
        .count();
    let mut output = String::with_capacity(
        input_value
            .len()
            .saturating_add(reserved_count)
            .saturating_add(wildcard_count),
    );
    if matches!(
        sql_like_match_mode,
        crate::sql_like_match_mode::SqlLikeMatchMode::Contains
            | crate::sql_like_match_mode::SqlLikeMatchMode::EndsWith
    ) {
        output.push('%');
    }
    input_value.chars().for_each(|character| {
        if matches!(character, '\\' | '%' | '_') {
            output.push('\\');
        }
        output.push(character);
    });
    if matches!(
        sql_like_match_mode,
        crate::sql_like_match_mode::SqlLikeMatchMode::Contains
            | crate::sql_like_match_mode::SqlLikeMatchMode::StartsWith
    ) {
        output.push('%');
    }
    crate::sql_like_pattern::SqlLikePattern::try_from(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_escaped_patterns_enforce_output_limit_after_wildcard_expansion() {
        assert!(
            [
                crate::sql_like_match_mode::SqlLikeMatchMode::Contains,
                crate::sql_like_match_mode::SqlLikeMatchMode::StartsWith,
                crate::sql_like_match_mode::SqlLikeMatchMode::EndsWith,
            ]
            .into_iter()
            .all(|sql_like_match_mode| {
                let wildcard_count = if sql_like_match_mode
                    == crate::sql_like_match_mode::SqlLikeMatchMode::Contains
                {
                    2usize
                } else {
                    1usize
                };
                ['%', '_', '\\'].into_iter().all(|reserved_symbol| {
                    let mut input = reserved_symbol.to_string().repeat(
                        (crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN
                            - wildcard_count)
                            .div_euclid(2usize),
                    );
                    if wildcard_count == 1usize {
                        input.push_str(constants_str::X);
                    }
                    let accepted = crate::build_sql_like_pattern::build_sql_like_pattern(
                        crate::sql_like_input_ref::SqlLikeInputRef::from(input.as_str()),
                        sql_like_match_mode,
                    )
                    .is_ok_and(|sql_like_pattern| {
                        sql_like_pattern.as_ref().len()
                            == crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN
                    });
                    input.push_str(constants_str::X);
                    accepted
                        && crate::build_sql_like_pattern::build_sql_like_pattern(
                            crate::sql_like_input_ref::SqlLikeInputRef::from(input.as_str()),
                            sql_like_match_mode,
                        ) == Err(crate::sql_like_pattern_error::SqlLikePatternError::TooLong)
                })
            })
        );
    }

    #[test]
    fn test_match_modes_place_wildcards_at_the_requested_edges() {
        assert!(matches!(
            crate::build_sql_like_pattern::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                crate::sql_like_match_mode::SqlLikeMatchMode::Contains,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_CONTAINS_PATTERN
        ));
        assert!(matches!(
            crate::build_sql_like_pattern::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                crate::sql_like_match_mode::SqlLikeMatchMode::StartsWith,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_STARTS_WITH_PATTERN
        ));
        assert!(matches!(
            crate::build_sql_like_pattern::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                crate::sql_like_match_mode::SqlLikeMatchMode::EndsWith,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_ENDS_WITH_PATTERN
        ));
    }

    #[test]
    fn test_reserved_symbols_are_escaped_as_literals() {
        assert!(matches!(
            crate::build_sql_like_pattern::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_RESERVED_INPUT.into(),
                crate::sql_like_match_mode::SqlLikeMatchMode::Contains,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_RESERVED_PATTERN
        ));
    }
}
