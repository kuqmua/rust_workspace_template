#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
)]
pub struct MultipartFileName(String);

impl TryFrom<String> for MultipartFileName {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() {
            return Err(Self::Error::EmptyFileName);
        }
        if string.len() > constants_usize::VALUE_1_024 {
            return Err(Self::Error::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(string.len()),
            });
        }
        if string.chars().any(char::is_control) {
            return Err(Self::Error::ControlCharacter);
        }
        if string.contains(['/', '\\'])
            || std::path::Path::new(&string)
                .file_name()
                .and_then(|name| name.to_str())
                != Some(&string)
        {
            return Err(Self::Error::PathComponent);
        }
        Ok(Self(string))
    }
}
