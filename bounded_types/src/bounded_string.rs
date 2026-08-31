#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct BoundedString<
    const MINIMUM_LENGTH: usize = { constants_usize::ZERO },
    const MAXIMUM_LENGTH: usize = { usize::MAX },
    const COUNT_CHARS: bool = false,
> {
    value: String,
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    #[must_use]
    fn value_len(value: &str) -> usize {
        if COUNT_CHARS {
            value.chars().count()
        } else {
            value.len()
        }
    }

    #[must_use]
    pub(crate) const fn from_prevalidated(value: String) -> Self {
        Self { value }
    }

    pub fn validate_str(
        value: &str,
    ) -> Result<(), crate::bounded_string_error::BoundedStringError> {
        let value_len = Self::value_len(value);
        let actual_length = crate::bounded_len::BoundedLen::from(value_len);
        if value_len < MINIMUM_LENGTH {
            return Err(
                crate::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length,
                    minimum_length: crate::bounded_len::BoundedLen::from(MINIMUM_LENGTH),
                },
            );
        }
        if value_len > MAXIMUM_LENGTH {
            return Err(
                crate::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    maximum_length: crate::bounded_len::BoundedLen::from(MAXIMUM_LENGTH),
                },
            );
        }
        Ok(())
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.value.as_str()
    }

    #[must_use]
    pub const fn as_string(&self) -> &String {
        &self.value
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }

    #[must_use]
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(Self::value_len(self.value.as_str()))
    }
}

impl BoundedString {
    #[must_use]
    pub const fn from_unbounded(value: String) -> Self {
        Self::from_prevalidated(value)
    }

    #[must_use]
    pub const fn as_mut_string(&mut self) -> &mut String {
        &mut self.value
    }
}

impl<const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    BoundedString<{ constants_usize::ZERO }, MAXIMUM_LENGTH, COUNT_CHARS>
{
    #[must_use]
    pub fn from_truncated(mut value: String) -> Self {
        if value.len() > MAXIMUM_LENGTH {
            let mut truncation_length = MAXIMUM_LENGTH;
            while !value.is_char_boundary(truncation_length) {
                truncation_length = truncation_length.saturating_sub(1usize);
            }
            value.truncate(truncation_length);
        }
        Self::from_prevalidated(value)
    }
}

impl<const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool> Default
    for BoundedString<{ constants_usize::ZERO }, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn default() -> Self {
        Self::from_prevalidated(String::new())
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool> AsRef<str>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    PartialEq<str> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    PartialEq<&str> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    PartialEq<String> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    std::borrow::Borrow<str> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    std::fmt::Display for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    std::ops::Deref for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    TryFrom<String> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    type Error = crate::bounded_string_error::BoundedStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let actual_length = Self::value_len(value.as_str());
        if actual_length < MINIMUM_LENGTH {
            return Err(
                crate::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length: actual_length.into(),
                    minimum_length: MINIMUM_LENGTH.into(),
                },
            );
        }
        if actual_length > MAXIMUM_LENGTH {
            return Err(
                crate::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length: actual_length.into(),
                    maximum_length: MAXIMUM_LENGTH.into(),
                },
            );
        }
        Ok(Self::from_prevalidated(value))
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    From<BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>> for String
{
    fn from(value: BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>) -> Self {
        value.into_string()
    }
}

impl<'de, const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    serde::Deserialize<'de> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let deserialize = |input: Deserializer| {
            let value = <String as serde::Deserialize>::deserialize(input)?;
            Self::try_from(value).map_err(serde::de::Error::custom)
        };
        deserialize(deserializer)
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    serde::Serialize for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serde::Serialize::serialize(&self.value, serializer)
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    utoipa::PartialSchema for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        if COUNT_CHARS {
            return utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::schema::Type::String)
                .min_length(Some(MINIMUM_LENGTH))
                .max_length(Some(MAXIMUM_LENGTH))
                .build()
                .into();
        }
        let extensions_builder = utoipa::openapi::extensions::ExtensionsBuilder::new()
            .add(constants_str::OPENAPI_MIN_BYTES_EXTENSION, MINIMUM_LENGTH);
        let extensions = if MAXIMUM_LENGTH.checked_add(1usize).is_none() {
            extensions_builder
        } else {
            extensions_builder.add(constants_str::OPENAPI_MAX_BYTES_EXTENSION, MAXIMUM_LENGTH)
        };
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .extensions(Some(extensions.build()))
            .build()
            .into()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    utoipa::ToSchema for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    schemars::JsonSchema for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(stringify!(BoundedString))
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!(
            "BoundedString<{MINIMUM_LENGTH},{MAXIMUM_LENGTH},{COUNT_CHARS}>"
        ))
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        generator.subschema_for::<String>()
    }
}
