#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::Display,
)]
pub struct BoundedCharsString<const MIN: usize, const MAX: usize>(String);

impl<const MIN: usize, const MAX: usize> BoundedCharsString<MIN, MAX> {
    pub fn validate_str(value: &str) -> Result<(), crate::bounded_value_error::BoundedValueError> {
        crate::validate_len::validate_len::<MIN, MAX>(crate::bounded_len::BoundedLen::from(
            value.chars().count(),
        ))
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    #[must_use]
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(self.0.chars().count())
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<str> for BoundedCharsString<MIN, MAX> {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<String> for BoundedCharsString<MIN, MAX> {
    type Error = crate::bounded_value_error::BoundedValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate_str(value.as_str()).map(|()| Self(value))
    }
}

impl<const MIN: usize, const MAX: usize> serde::Serialize for BoundedCharsString<MIN, MAX> {
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serde::Serialize::serialize(&self.0, serializer)
    }
}

impl<'de, const MIN: usize, const MAX: usize> serde::Deserialize<'de>
    for BoundedCharsString<MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl<const MIN: usize, const MAX: usize> utoipa::PartialSchema for BoundedCharsString<MIN, MAX> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .min_length(Some(MIN))
            .max_length(Some(MAX))
            .build()
            .into()
    }
}

impl<const MIN: usize, const MAX: usize> utoipa::ToSchema for BoundedCharsString<MIN, MAX> {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unicode_is_measured_in_chars() {
        let value = crate::bounded_chars_string::BoundedCharsString::<2, 2>::try_from(
            [char::from_u32(0x430), char::from_u32(0x431)]
                .into_iter()
                .flatten()
                .collect::<String>(),
        )
        .expect(constants_str::DIAGNOSTIC_AD96C37E);
        assert_eq!(value.len().get(), 2);
        assert_eq!(value.as_ref().chars().count(), 2);
    }

    #[test]
    fn test_char_bounds_are_inclusive() {
        assert!(matches!(
            crate::bounded_chars_string::BoundedCharsString::<1, 2>::try_from(String::new()),
            Err(crate::bounded_value_error::BoundedValueError::BelowMin { .. })
        ));
        assert!(matches!(
            crate::bounded_chars_string::BoundedCharsString::<1, 2>::try_from(String::from(
                constants_str::ABC_ALT_3
            )),
            Err(crate::bounded_value_error::BoundedValueError::AboveMax { .. })
        ));
    }
}
