#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct BoundedString<
    const MINIMUM_LENGTH: usize = { constants_usize::ZERO },
    const MAXIMUM_LENGTH: usize = { usize::MAX },
    const COUNT_CHARS: bool = false,
> {
    value: bounded_string_core::bounded_string_storage::BoundedStringStorage<
        MINIMUM_LENGTH,
        MAXIMUM_LENGTH,
        COUNT_CHARS,
    >,
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn map_core_error(
        source: bounded_string_core::bounded_string_storage_error::BoundedStringStorageError,
    ) -> crate::bounded_string_error::BoundedStringError {
        match source {
            bounded_string_core::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                actual_length,
                maximum_length,
            } => crate::bounded_string_error::BoundedStringError::AboveMaximum {
                actual_length: actual_length.into(),
                maximum_length: maximum_length.into(),
            },
            bounded_string_core::bounded_string_storage_error::BoundedStringStorageError::BelowMinimum {
                actual_length,
                minimum_length,
            } => crate::bounded_string_error::BoundedStringError::BelowMinimum {
                actual_length: actual_length.into(),
                minimum_length: minimum_length.into(),
            },
        }
    }

    pub fn validate_str(str: &str) -> Result<(), crate::bounded_string_error::BoundedStringError> {
        bounded_string_core::bounded_string_storage::BoundedStringStorage::<
            MINIMUM_LENGTH,
            MAXIMUM_LENGTH,
            COUNT_CHARS,
        >::validate_str(str)
        .map_err(Self::map_core_error)
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.value.as_str()
    }

    #[must_use]
    pub const fn as_string(&self) -> &String {
        self.value.as_string()
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.value.into_string()
    }

    #[must_use]
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(self.value.len())
    }

    pub fn try_push_str(
        &mut self,
        str: &str,
    ) -> Result<(), crate::bounded_string_error::BoundedStringError> {
        self.value.try_push_str(str).map_err(Self::map_core_error)
    }

    pub fn try_push(
        &mut self,
        char: char,
    ) -> Result<(), crate::bounded_string_error::BoundedStringError> {
        self.value.try_push(char).map_err(Self::map_core_error)
    }
}

impl BoundedString {
    #[must_use]
    pub const fn from_unbounded(string: String) -> Self {
        Self {
            value:
                bounded_string_core::bounded_string_storage::BoundedStringStorage::from_unbounded(
                    string,
                ),
        }
    }

    #[must_use]
    pub const fn as_mut_string(&mut self) -> &mut String {
        self.value.as_mut_string()
    }
}

impl<const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    BoundedString<{ constants_usize::ZERO }, MAXIMUM_LENGTH, COUNT_CHARS>
{
    #[must_use]
    pub fn from_truncated(string: String) -> Self {
        Self {
            value:
                bounded_string_core::bounded_string_storage::BoundedStringStorage::from_truncated(
                    string,
                ),
        }
    }
}

impl<const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool> Default
    for BoundedString<{ constants_usize::ZERO }, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn default() -> Self {
        Self {
            value: bounded_string_core::bounded_string_storage::BoundedStringStorage::default(),
        }
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
    fn eq(&self, str: &str) -> bool {
        self.as_str() == str
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    PartialEq<&str> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn eq(&self, str: &&str) -> bool {
        self.as_str() == *str
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    PartialEq<String> for BoundedString<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn eq(&self, string: &String) -> bool {
        self.as_str() == string
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
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
        bounded_string_core::bounded_string_storage::BoundedStringStorage::try_from(value)
            .map(|bounded_string_core| Self {
                value: bounded_string_core,
            })
            .map_err(Self::map_core_error)
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
        serde::Serialize::serialize(self.value.as_string(), serializer)
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

    fn json_schema(schema_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_generator.subschema_for::<String>()
    }
}
