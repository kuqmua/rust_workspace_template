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
pub struct BoundedStringStorage<
    const MINIMUM_LENGTH: usize = 0usize,
    const MAXIMUM_LENGTH: usize = { usize::MAX },
    const COUNT_CHARS: bool = false,
> {
    value: String,
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    BoundedStringStorage<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
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
    pub const fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        Self::value_len(self.value.as_str())
    }

    pub fn try_push(
        &mut self,
        char: char,
    ) -> Result<(), crate::bounded_string_storage_error::BoundedStringStorageError> {
        let mut buffer = [0u8; 4usize];
        self.try_push_str(char.encode_utf8(&mut buffer))
    }

    pub fn try_push_str(
        &mut self,
        str: &str,
    ) -> Result<(), crate::bounded_string_storage_error::BoundedStringStorageError> {
        let actual_length =
            Self::value_len(self.value.as_str()).saturating_add(Self::value_len(str));
        if actual_length > MAXIMUM_LENGTH {
            return Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                    actual_length,
                    maximum_length: MAXIMUM_LENGTH,
                },
            );
        }
        self.value.push_str(str);
        Ok(())
    }

    pub fn validate_str(
        str: &str,
    ) -> Result<(), crate::bounded_string_storage_error::BoundedStringStorageError> {
        let actual_length = Self::value_len(str);
        if actual_length < MINIMUM_LENGTH {
            return Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::BelowMinimum {
                    actual_length,
                    minimum_length: MINIMUM_LENGTH,
                },
            );
        }
        if actual_length > MAXIMUM_LENGTH {
            return Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                    actual_length,
                    maximum_length: MAXIMUM_LENGTH,
                },
            );
        }
        Ok(())
    }

    fn value_len(str: &str) -> usize {
        if COUNT_CHARS {
            str.chars().count()
        } else {
            str.len()
        }
    }
}

impl BoundedStringStorage {
    pub const fn as_mut_string(&mut self) -> &mut String {
        &mut self.value
    }

    #[must_use]
    pub const fn from_unbounded(value: String) -> Self {
        Self { value }
    }
}

impl<const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    BoundedStringStorage<0usize, MAXIMUM_LENGTH, COUNT_CHARS>
{
    #[must_use]
    pub fn from_truncated(mut value: String) -> Self {
        if Self::value_len(value.as_str()) > MAXIMUM_LENGTH {
            if COUNT_CHARS {
                let truncation_byte_index = value
                    .char_indices()
                    .nth(MAXIMUM_LENGTH)
                    .map_or(value.len(), |(index, _)| index);
                value.truncate(truncation_byte_index);
            } else {
                let mut truncation_byte_index = MAXIMUM_LENGTH;
                while !value.is_char_boundary(truncation_byte_index) {
                    truncation_byte_index = truncation_byte_index.saturating_sub(1usize);
                }
                value.truncate(truncation_byte_index);
            }
        }
        Self { value }
    }
}

impl<const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool> Default
    for BoundedStringStorage<0usize, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn default() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    TryFrom<String> for BoundedStringStorage<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    type Error = crate::bounded_string_storage_error::BoundedStringStorageError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate_str(value.as_str())?;
        Ok(Self { value })
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool> AsRef<str>
    for BoundedStringStorage<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    std::fmt::Display for BoundedStringStorage<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    std::ops::Deref for BoundedStringStorage<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const MINIMUM_LENGTH: usize, const MAXIMUM_LENGTH: usize, const COUNT_CHARS: bool>
    quote::ToTokens for BoundedStringStorage<MINIMUM_LENGTH, MAXIMUM_LENGTH, COUNT_CHARS>
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::ToTokens::to_tokens(&self.value, tokens);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_try_from_enforces_byte_bounds() {
        let _below =
            crate::bounded_string_storage::BoundedStringStorage::<1usize, 2usize, false>::try_from(
                String::new(),
            )
            .expect_err(constants_str::DIAGNOSTIC_90DF28FB);
        let _above =
            crate::bounded_string_storage::BoundedStringStorage::<1usize, 2usize, false>::try_from(
                ['a', 'b', 'c'].into_iter().collect::<String>(),
            )
            .expect_err(constants_str::DIAGNOSTIC_170980BA);
    }

    #[test]
    fn test_try_from_counts_characters_when_requested() {
        let value =
            crate::bounded_string_storage::BoundedStringStorage::<2usize, 2usize, true>::try_from(
                [char::from_u32(0x430), char::from_u32(0x431)]
                    .into_iter()
                    .flatten()
                    .collect::<String>(),
            )
            .expect(constants_str::DIAGNOSTIC_B61A0E23);
        assert_eq!(value.len(), 2usize);
    }
}
