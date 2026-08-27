#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct FileStagingDirectoryName(String);

impl TryFrom<String> for FileStagingDirectoryName {
    type Error = super::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 256usize {
            Err(super::MultipartValueError::TooLong {
                actual: super::MultipartValueLength::from(value.len()),
            })
        } else {
            Ok(Self(value))
        }
    }
}
