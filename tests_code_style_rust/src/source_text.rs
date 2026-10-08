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
#[cfg(test)]
mod tests {
    #[test]
    fn test_analyzer_source_text_preserves_empty_utf8_and_byte_limit_conversions() {
        assert!(
            crate::source_text::SourceText::try_from(String::new())
                .is_ok_and(|source_text| String::from(source_text).is_empty())
        );
        let maximum = constants_usize::VALUE_16_777_216;
        assert!([maximum - 1usize, maximum, maximum + 1usize].into_iter().all(|length| {
            [constants_str::X, constants_str::NON_ASCII_U_E9].into_iter().all(|suffix| {
                let mut value = constants_str::X.repeat(length - suffix.len());
                value.push_str(suffix);
                let result = crate::source_text::SourceText::try_from(value);
                if length <= maximum {
                    result.is_ok_and(|source_text| {
                        let pointer = source_text.as_ref().as_ptr();
                        let preserved = source_text.as_ref().len() == length && source_text.as_ref().ends_with(suffix);
                        let text = String::from(source_text);
                        preserved && text.as_ptr() == pointer && text.len() == length
                            && text.ends_with(suffix)
                            && text.as_bytes().iter().take(length - suffix.len()).all(|byte| *byte == b'x')
                    })
                } else {
                    result.is_err_and(|error| matches!(error,
                        crate::source_text_try_from_string_error::SourceTextTryFromStringError::TooLong { len }
                            if len.get() == length))
                }
            })
        }));
    }
}
