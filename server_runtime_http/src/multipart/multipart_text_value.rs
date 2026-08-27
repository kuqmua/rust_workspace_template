#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct MultipartTextValue(String);

impl TryFrom<String> for MultipartTextValue {
    type Error = super::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 65_536usize {
            return Err(Self::Error::TooLong {
                actual: super::MultipartValueLength::from(value.len()),
            });
        }
        if value.contains('\0') {
            return Err(Self::Error::Nul);
        }
        Ok(Self(value))
    }
}
