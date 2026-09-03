#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct MultipartFieldName(String);

impl TryFrom<String> for MultipartFieldName {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() {
            return Err(Self::Error::EmptyFieldName);
        }
        if string.len() > 256usize {
            return Err(Self::Error::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(string.len()),
            });
        }
        if string.chars().any(char::is_control) {
            return Err(Self::Error::ControlCharacter);
        }
        Ok(Self(string))
    }
}
