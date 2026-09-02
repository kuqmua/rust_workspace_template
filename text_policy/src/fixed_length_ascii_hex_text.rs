#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::IntoInner,
)]
pub struct FixedLengthAsciiHexText(String);
impl TryFrom<String> for FixedLengthAsciiHexText {
    type Error = crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() != 40usize {
            Err(Self::Error::InvalidLength)
        } else if !string
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            Err(Self::Error::InvalidSymbol)
        } else {
            Ok(Self(string))
        }
    }
}
