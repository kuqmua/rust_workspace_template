#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct MultipartTextValue(
    bounded_types::bounded_string::BoundedString<0usize, 65_536usize, false>,
);

impl TryFrom<String> for MultipartTextValue {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 65_536usize {
            return Err(Self::Error::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(value.len()),
            });
        }
        if value.contains('\0') {
            return Err(Self::Error::Nul);
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length,
                    ..
                } => Self::Error::TooLong {
                    actual: crate::multipart_value_length::MultipartValueLength::from(
                        actual_length.get(),
                    ),
                },
            })
    }
}
