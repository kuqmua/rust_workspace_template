#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
)]
pub struct FileStagingDirectoryName(String);

impl TryFrom<String> for FileStagingDirectoryName {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 256usize {
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(value.len()),
            })
        } else {
            Ok(Self(value))
        }
    }
}
