#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct BoundedString<
    const MINIMUM_LENGTH: usize = { constants_usize::ZERO },
    const MAXIMUM_LENGTH: usize = { usize::MAX },
> {
    value: String,
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize>
    BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    #[must_use]
    pub(crate) const fn from_prevalidated(value: String) -> Self {
        Self { value }
    }

    pub fn validate_str(
        value: &str,
    ) -> Result<(), crate::bounded_string_error::BoundedStringError> {
        let actual_length = crate::bounded_len::BoundedLen::from(value.len());
        if value.len() < MINIMUM_LENGTH {
            return Err(
                crate::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length,
                    minimum_length: crate::bounded_len::BoundedLen::from(MINIMUM_LENGTH),
                },
            );
        }
        if value.len() > MAXIMUM_LENGTH {
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
    pub fn into_string(self) -> String {
        self.value
    }

    #[must_use]
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(self.value.len())
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

impl<const MAXIMUM_LENGTH: usize> BoundedString<{ constants_usize::ZERO }, MAXIMUM_LENGTH> {
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

impl<const MAXIMUM_LENGTH: usize> Default
    for BoundedString<{ constants_usize::ZERO }, MAXIMUM_LENGTH>
{
    fn default() -> Self {
        Self::from_prevalidated(String::new())
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> AsRef<str>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> PartialEq<str>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> PartialEq<&str>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> PartialEq<String>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> std::borrow::Borrow<str>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> std::fmt::Display
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> std::ops::Deref
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> TryFrom<String>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    type Error = crate::bounded_string_error::BoundedStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let actual_length = value.len();
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

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize>
    From<BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>> for String
{
    fn from(value: BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>) -> Self {
        value.into_string()
    }
}

impl<'de, const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> serde::Deserialize<'de>
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        crate::deserialize_bounded_owned_string::deserialize_bounded_owned_string(deserializer)
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> serde::Serialize
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
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

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> utoipa::PartialSchema
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
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

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize> utoipa::ToSchema
    for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH>
{
}
