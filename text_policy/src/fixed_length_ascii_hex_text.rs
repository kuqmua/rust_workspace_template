#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct FixedLengthAsciiHexText(
    bounded_types::bounded_string::BoundedString<40usize, 40usize, false>,
);
impl TryFrom<String> for FixedLengthAsciiHexText {
    type Error = crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 40usize {
            Err(Self::Error::InvalidLength)
        } else if !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            Err(Self::Error::InvalidSymbol)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::InvalidLength,
                })
        }
    }
}
