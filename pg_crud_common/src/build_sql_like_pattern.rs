pub fn build_sql_like_pattern(
    input: crate::domain_types::SqlLikeInputRef<'_>,
    match_mode: crate::domain_types::SqlLikeMatchMode,
) -> Result<crate::domain_types::SqlLikePattern, crate::domain_types::SqlLikePatternError> {
    let wildcard_count = match match_mode {
        crate::domain_types::SqlLikeMatchMode::Contains => 2usize,
        crate::domain_types::SqlLikeMatchMode::EndsWith
        | crate::domain_types::SqlLikeMatchMode::StartsWith => constants_usize::ONE,
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
        crate::domain_types::SqlLikeMatchMode::Contains
            | crate::domain_types::SqlLikeMatchMode::EndsWith
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
        crate::domain_types::SqlLikeMatchMode::Contains
            | crate::domain_types::SqlLikeMatchMode::StartsWith
    ) {
        output.push('%');
    }
    crate::domain_types::SqlLikePattern::try_from(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn match_modes_place_wildcards_at_the_requested_edges() {
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                crate::domain_types::SqlLikeMatchMode::Contains,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_CONTAINS_PATTERN
        ));
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                crate::domain_types::SqlLikeMatchMode::StartsWith,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_STARTS_WITH_PATTERN
        ));
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                crate::domain_types::SqlLikeMatchMode::EndsWith,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_ENDS_WITH_PATTERN
        ));
    }

    #[test]
    fn reserved_symbols_are_escaped_as_literals() {
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_RESERVED_INPUT.into(),
                crate::domain_types::SqlLikeMatchMode::Contains,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_RESERVED_PATTERN
        ));
    }
}
