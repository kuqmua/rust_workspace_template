#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct FileStagingDirectoryName(crate::multipart_bounded_text::MultipartBoundedText<256usize>);

impl TryFrom<String> for FileStagingDirectoryName {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::multipart_bounded_text::MultipartBoundedText::try_from(value).map(Self)
    }
}
