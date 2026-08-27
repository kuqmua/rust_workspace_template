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
    type Error = crate::domain_types::SqlLikePatternError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > super::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(crate::domain_types::SqlLikePatternError)
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialization_uses_bounded_try_from() {
        let _error = <super::SqlLikePattern as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                constants_str::X
                    .repeat(super::super::PG_CRUD_STRING_WRAPPER_MAX_LEN + constants_usize::ONE),
            ),
        )
        .expect_err(constants_str::VALUE_9EED211B);
    }
}
