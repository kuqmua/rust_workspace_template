#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct MultipartFieldName(
    bounded_types::bounded_string::BoundedString<1usize, 256usize, false>,
);

impl TryFrom<String> for MultipartFieldName {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::EmptyFieldName);
        }
        if value.len() > 256usize {
            return Err(Self::Error::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(value.len()),
            });
        }
        if value.chars().any(char::is_control) {
            return Err(Self::Error::ControlCharacter);
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    ..
                } => Self::Error::TooLong {
                    actual: crate::multipart_value_length::MultipartValueLength::from(
                        actual_length.get(),
                    ),
                },
                bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::EmptyFieldName,
            })
    }
}
