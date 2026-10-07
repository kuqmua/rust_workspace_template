#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_to_err_string::ToErrString,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct AxumBodySizeError(axum::Error);

#[cfg(test)]
mod tests {
    #[test]
    fn test_native_body_error_diagnostics_preserve_display_and_bounded_fallbacks() {
        [
            (constants_str::EMPTY.to_owned(), false),
            (constants_str::X.to_owned(), false),
            (constants_str::U_1F496.to_owned(), false),
            (
                constants_str::X.repeat(constants_usize::VALUE_1_048_576),
                false,
            ),
            (
                constants_str::NEWLINE.repeat(constants_usize::VALUE_1_048_576),
                false,
            ),
            (
                constants_str::U_1F496.repeat(constants_usize::VALUE_1_048_576),
                true,
            ),
            (
                constants_str::X.repeat(constants_usize::VALUE_1_048_576.saturating_add(1usize)),
                true,
            ),
        ]
        .into_iter()
        .fold((), |(), (message, expected_overflow)| {
            let native_error = axum::Error::new(std::io::Error::other(message));
            let display = native_error.to_string();
            let error = crate::axum_body_size_error::AxumBodySizeError::from(native_error);
            let expected = match to_err_string::error_text::ErrorText::try_from(display) {
                Ok(text) => {
                    assert!(!expected_overflow);
                    text
                }
                Err(source) => {
                    assert!(expected_overflow);
                    to_err_string::error_text::ErrorText::from(source)
                }
            };
            assert_eq!(
                to_err_string::to_err_string::ToErrString::to_err_string(&error),
                expected
            );
        });
    }
}
