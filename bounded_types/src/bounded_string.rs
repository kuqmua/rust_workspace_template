#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefInner,
)]
pub struct BoundedString<const MIN: usize, const MAX: usize>(String);
impl<const MIN: usize, const MAX: usize> BoundedString<MIN, MAX> {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    #[must_use]
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(self.0.len())
    }
}
impl<const MIN: usize, const MAX: usize> AsRef<str> for BoundedString<MIN, MAX> {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl<const MIN: usize, const MAX: usize> std::fmt::Display for BoundedString<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl<const MIN: usize, const MAX: usize> TryFrom<String> for BoundedString<MIN, MAX> {
    type Error = crate::bounded_value_error::BoundedValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::validate_len::validate_len::<MIN, MAX>(crate::bounded_len::BoundedLen::from(
            value.len(),
        ))
        .map(|()| Self(value))
    }
}
impl<const MIN: usize, const MAX: usize> serde::Serialize for BoundedString<MIN, MAX> {
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
impl<'de, const MIN: usize, const MAX: usize> serde::Deserialize<'de> for BoundedString<MIN, MAX> {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl<const MIN: usize, const MAX: usize> utoipa::PartialSchema for BoundedString<MIN, MAX> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let extensions_builder = utoipa::openapi::extensions::ExtensionsBuilder::new()
            .add(constants_str::OPENAPI_MIN_BYTES_EXTENSION, MIN);
        let extensions = if MAX == usize::MAX {
            extensions_builder
        } else {
            extensions_builder.add(constants_str::OPENAPI_MAX_BYTES_EXTENSION, MAX)
        };
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .extensions(Some(extensions.build()))
            .build()
            .into()
    }
}
impl<const MIN: usize, const MAX: usize> utoipa::ToSchema for BoundedString<MIN, MAX> {}

#[cfg(test)]
mod tests {
    #[test]
    fn string_validates_inclusive_bounds() {
        let value = crate::bounded_string::BoundedString::<1, 1>::try_from(String::from(
            constants_str::A_ALT,
        ))
        .expect("3ca72d81 string_validates_inclusive_bounds invariant must hold");
        assert_eq!(value.as_ref(), "a");
        assert!(matches!(
            crate::bounded_string::BoundedString::<1, 1>::try_from(String::new()),
            Err(crate::bounded_value_error::BoundedValueError::BelowMin { .. })
        ));
    }
}
