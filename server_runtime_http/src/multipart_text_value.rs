#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct MultipartTextValue(String);

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
        Ok(Self(value))
    }
}
