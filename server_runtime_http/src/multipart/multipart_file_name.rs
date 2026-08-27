#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct MultipartFileName(String);

impl TryFrom<String> for MultipartFileName {
    type Error = super::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::EmptyFileName);
        }
        if value.len() > constants_usize::VALUE_1_024 {
            return Err(Self::Error::TooLong {
                actual: super::MultipartValueLength::from(value.len()),
            });
        }
        if value.chars().any(char::is_control) {
            return Err(Self::Error::ControlCharacter);
        }
        if value.contains(['/', '\\'])
            || std::path::Path::new(&value)
                .file_name()
                .and_then(|name| name.to_str())
                != Some(&value)
        {
            return Err(Self::Error::PathComponent);
        }
        Ok(Self(value))
    }
}
