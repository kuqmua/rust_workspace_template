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
        let below =
            crate::bounded_string_storage::BoundedStringStorage::<1usize, 2usize, false>::try_from(
                String::new(),
            )
            .expect_err(constants_str::DIAGNOSTIC_90DF28FB);
        let above =
            crate::bounded_string_storage::BoundedStringStorage::<1usize, 2usize, false>::try_from(
                ['a', 'b', 'c'].into_iter().collect::<String>(),
            )
            .expect_err(constants_str::DIAGNOSTIC_170980BA);
        assert_eq!(
            below,
            crate::bounded_string_storage_error::BoundedStringStorageError::BelowMinimum {
                actual_length: 0usize,
                minimum_length: 1usize,
            }
        );
        assert_eq!(
            above,
            crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                actual_length: 3usize,
                maximum_length: 2usize,
            }
        );
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

    #[test]
    fn test_core_storage_character_validation_reports_scalar_lengths() {
        let below =
            super::BoundedStringStorage::<2usize, 2usize, true>::try_from('\u{00e9}'.to_string());
        assert_eq!(
            below,
            Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::BelowMinimum {
                    actual_length: 1usize,
                    minimum_length: 2usize,
                }
            )
        );
        let text = ['\u{00e9}', '\u{03b2}', '\u{1f600}']
            .into_iter()
            .collect::<String>();
        assert_eq!(
            super::BoundedStringStorage::<2usize, 2usize, true>::try_from(text),
            Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                    actual_length: 3usize,
                    maximum_length: 2usize,
                }
            )
        );
    }

    #[test]
    fn test_character_truncation_preserves_unicode_boundaries() {
        let text = ['\u{00e9}', '\u{03b2}', 'x']
            .into_iter()
            .collect::<String>();
        let value = super::BoundedStringStorage::<0usize, 2usize, true>::from_truncated(text);
        assert_eq!(value.len(), 2usize);
        assert!(value.as_str().chars().eq(['\u{00e9}', '\u{03b2}']));
        assert!(!value.is_empty());
        let empty = super::BoundedStringStorage::<0usize, 0usize, true>::from_truncated(
            '\u{00e9}'.to_string(),
        );
        assert!(empty.as_str().is_empty());
        assert!(empty.is_empty());
        let unchanged = super::BoundedStringStorage::<0usize, 2usize, true>::from_truncated(
            constants_str::X.to_owned(),
        );
        assert_eq!(unchanged.as_str(), constants_str::X);
    }

    #[test]
    fn test_storage_borrowing_and_tokens_preserve_escaped_text() {
        let text = ['x', '"', '\\', '\n'].into_iter().collect::<String>();
        let expected = proc_macro2::Literal::string(&text).to_string();
        let result = super::BoundedStringStorage::<0usize, 16usize, false>::try_from(text);
        assert!(result.is_ok());
        let Ok(value) = result else {
            return;
        };
        assert_eq!(
            <super::BoundedStringStorage<0usize, 16usize, false> as AsRef<str>>::as_ref(&value),
            value.as_str()
        );
        assert_eq!(
            <super::BoundedStringStorage<0usize, 16usize, false> as std::ops::Deref>::deref(&value),
            value.as_str()
        );
        assert_eq!(value.to_string(), value.as_str());
        let mut tokens = proc_macro2::TokenStream::new();
        quote::ToTokens::to_tokens(&value, &mut tokens);
        assert_eq!(tokens.to_string(), expected);
    }

    #[test]
    fn test_core_storage_failed_byte_append_preserves_value() {
        let mut value = super::BoundedStringStorage::<0usize, 3usize, false>::default();
        assert_eq!(value.try_push('\u{00e9}'), Ok(()));
        assert_eq!(value.try_push_str(constants_str::X), Ok(()));
        let expected = ['\u{00e9}', 'x'].into_iter().collect::<String>();
        assert_eq!(
            value.try_push('\u{03b2}'),
            Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                    actual_length: 5usize,
                    maximum_length: 3usize,
                }
            )
        );
        assert_eq!(value.as_string(), &expected);
        assert_eq!(value.try_push_str(constants_str::EMPTY), Ok(()));
        assert_eq!(value.len(), 3usize);
        assert_eq!(value.into_string(), expected);
    }

    #[test]
    fn test_core_storage_character_append_counts_unicode_scalars() {
        let mut value = super::BoundedStringStorage::<0usize, 2usize, true>::default();
        assert_eq!(value.try_push('\u{00e9}'), Ok(()));
        assert_eq!(value.try_push('\u{03b2}'), Ok(()));
        assert_eq!(value.len(), 2usize);
        assert_eq!(
            value.try_push_str(constants_str::X),
            Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                    actual_length: 3usize,
                    maximum_length: 2usize,
                }
            )
        );
        assert!(value.as_str().chars().eq(['\u{00e9}', '\u{03b2}']));
    }

    #[test]
    fn test_core_storage_byte_truncation_handles_split_and_zero_boundaries() {
        let text = ['\u{00e9}', '\u{03b2}'].into_iter().collect::<String>();
        let split = super::BoundedStringStorage::<0usize, 3usize, false>::from_truncated(text);
        assert_eq!(split.len(), 2usize);
        assert!(split.as_str().chars().eq(['\u{00e9}']));
        let empty = super::BoundedStringStorage::<0usize, 0usize, false>::from_truncated(
            '\u{00e9}'.to_string(),
        );
        assert!(empty.is_empty());
        let exact = super::BoundedStringStorage::<0usize, 2usize, false>::from_truncated(
            '\u{00e9}'.to_string(),
        );
        assert!(exact.as_str().chars().eq(['\u{00e9}']));
    }
}
