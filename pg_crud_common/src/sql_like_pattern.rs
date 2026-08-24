#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlLikeMatchMode {
    Contains,
    EndsWith,
    StartsWith,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlLikeInputRef<'value_lt>(&'value_lt str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = SqlLikePattern::validate)]
#[serde(try_from = "String")]
pub struct SqlLikePattern(String);
impl SqlLikePattern {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), SqlLikePatternError> {
        if value.len() > super::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(SqlLikePatternError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::SQL_LIKE_PATTERN_EXCEEDS_MAXIMUM_LENGTH)]
pub struct SqlLikePatternError;
pub fn build_sql_like_pattern(
    input: SqlLikeInputRef<'_>,
    match_mode: SqlLikeMatchMode,
) -> Result<SqlLikePattern, SqlLikePatternError> {
    let wildcard_count = match match_mode {
        SqlLikeMatchMode::Contains => 2usize,
        SqlLikeMatchMode::EndsWith | SqlLikeMatchMode::StartsWith => constants_usize::ONE,
    };
    let reserved_count = input
        .0
        .bytes()
        .filter(|byte| matches!(byte, b'\\' | b'%' | b'_'))
        .count();
    let mut output = String::with_capacity(
        input
            .0
            .len()
            .saturating_add(reserved_count)
            .saturating_add(wildcard_count),
    );
    if matches!(
        match_mode,
        SqlLikeMatchMode::Contains | SqlLikeMatchMode::EndsWith
    ) {
        output.push('%');
    }
    input.0.chars().for_each(|character| {
        if matches!(character, '\\' | '%' | '_') {
            output.push('\\');
        }
        output.push(character);
    });
    if matches!(
        match_mode,
        SqlLikeMatchMode::Contains | SqlLikeMatchMode::StartsWith
    ) {
        output.push('%');
    }
    SqlLikePattern::try_from(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn match_modes_place_wildcards_at_the_requested_edges() {
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                super::SqlLikeMatchMode::Contains,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_CONTAINS_PATTERN
        ));
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                super::SqlLikeMatchMode::StartsWith,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_STARTS_WITH_PATTERN
        ));
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_INPUT.into(),
                super::SqlLikeMatchMode::EndsWith,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_ENDS_WITH_PATTERN
        ));
    }

    #[test]
    fn reserved_symbols_are_escaped_as_literals() {
        assert!(matches!(
            super::build_sql_like_pattern(
                constants_str::TEST_SQL_LIKE_RESERVED_INPUT.into(),
                super::SqlLikeMatchMode::Contains,
            ),
            Ok(pattern) if pattern.as_ref() == constants_str::TEST_SQL_LIKE_RESERVED_PATTERN
        ));
    }

    #[test]
    fn deserialization_uses_bounded_try_from() {
        let _error = <super::SqlLikePattern as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                "x".repeat(super::super::PG_CRUD_STRING_WRAPPER_MAX_LEN + constants_usize::ONE),
            ),
        )
        .expect_err("432eaebe");
    }
}
