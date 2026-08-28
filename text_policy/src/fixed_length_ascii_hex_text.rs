use crate::domain_types::FixedLengthAsciiHexTextError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::IntoInner,
)]
pub struct FixedLengthAsciiHexText(String);
impl TryFrom<String> for FixedLengthAsciiHexText {
    type Error = FixedLengthAsciiHexTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 40usize {
            Err(Self::Error::InvalidLength)
        } else if !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            Err(Self::Error::InvalidSymbol)
        } else {
            Ok(Self(value))
        }
    }
}
