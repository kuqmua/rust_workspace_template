pub fn build_sql_like_pattern(
    input: crate::sql_like_input_ref::SqlLikeInputRef<'_>,
    match_mode: crate::sql_like_match_mode::SqlLikeMatchMode,
) -> Result<
    crate::sql_like_pattern::SqlLikePattern,
    crate::sql_like_pattern_error::SqlLikePatternError,
> {
    let wildcard_count = match match_mode {
        crate::sql_like_match_mode::SqlLikeMatchMode::Contains => 2usize,
        crate::sql_like_match_mode::SqlLikeMatchMode::EndsWith
        | crate::sql_like_match_mode::SqlLikeMatchMode::StartsWith => constants_usize::ONE,
    };
    let input_value = input.get();
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
        match_mode,
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
        match_mode,
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
    fn match_modes_place_wildcards_at_the_requested_edges() {
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
    fn reserved_symbols_are_escaped_as_literals() {
        assert!(matches!(
            crate::build_sql_like_pattern::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_RESERVED_INPUT.into(),
                crate::sql_like_match_mode::SqlLikeMatchMode::Contains,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_RESERVED_PATTERN
        ));
    }
}
