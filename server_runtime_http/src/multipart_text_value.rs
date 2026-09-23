#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct MultipartTextValue(crate::multipart_bounded_text::MultipartBoundedText<65_536usize>);

impl TryFrom<String> for MultipartTextValue {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let multipart_bounded_text =
            crate::multipart_bounded_text::MultipartBoundedText::try_from(value)?;
        if multipart_bounded_text.as_ref().contains('\0') {
            return Err(Self::Error::Nul);
        }
        Ok(Self(multipart_bounded_text))
    }
}
