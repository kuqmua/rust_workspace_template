#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub(super) struct SourceText(Box<str>);
impl TryFrom<String> for SourceText {
    type Error = crate::source_text_try_from_string_error::SourceTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(
                crate::source_text_try_from_string_error::SourceTextTryFromStringError::TooLong {
                    len: crate::analyzer_count::AnalyzerCount::from(value.len()),
                },
            );
        }
        Ok(Self(value.into_boxed_str()))
    }
}
impl From<SourceText> for String {
    fn from(value: SourceText) -> Self {
        value.0.into_string()
    }
}
