#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct MultipartFieldName(String);

impl TryFrom<String> for MultipartFieldName {
    type Error = super::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::EmptyFieldName);
        }
        if value.len() > 256usize {
            return Err(Self::Error::TooLong {
                actual: super::MultipartValueLength::from(value.len()),
            });
        }
        if value.chars().any(char::is_control) {
            return Err(Self::Error::ControlCharacter);
        }
        Ok(Self(value))
    }
}
