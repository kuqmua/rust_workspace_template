#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct MultipartTextValue(String);

impl TryFrom<String> for MultipartTextValue {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > 65_536usize {
            return Err(Self::Error::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(string.len()),
            });
        }
        if string.contains('\0') {
            return Err(Self::Error::Nul);
        }
        Ok(Self(string))
    }
}
