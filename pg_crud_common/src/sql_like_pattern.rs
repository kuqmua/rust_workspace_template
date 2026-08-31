#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    newtype::AsRefStr,
)]
#[serde(try_from = "String")]
pub struct SqlLikePattern(String);

impl TryFrom<String> for SqlLikePattern {
    type Error = crate::sql_like_pattern_error::SqlLikePatternError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(crate::sql_like_pattern_error::SqlLikePatternError::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialization_uses_bounded_try_from() {
        let _error = <crate::sql_like_pattern::SqlLikePattern as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                constants_str::X.repeat(
                    crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN
                        + constants_usize::ONE,
                ),
            ),
        )
        .expect_err(constants_str::VALUE_9EED211B);
    }
}
