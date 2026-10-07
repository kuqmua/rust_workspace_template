#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct HttpBodySizeHint(http_body::SizeHint);

impl HttpBodySizeHint {
    #[cfg(test)]
    pub(crate) fn upper(self) -> Option<u64> {
        self.0.upper()
    }
}

impl to_err_string::to_err_string::ToErrString for HttpBodySizeHint {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(format!("{:#?}", self.0))
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_body_size_hint_diagnostics_preserve_unknown_exact_and_extreme_bounds() {
        let unknown = http_body::SizeHint::new();
        let mut exact_zero = http_body::SizeHint::new();
        exact_zero.set_exact(0u64);
        let mut exact_maximum = http_body::SizeHint::new();
        exact_maximum.set_exact(u64::MAX);
        let mut ranged = http_body::SizeHint::new();
        ranged.set_lower(3u64);
        ranged.set_upper(7u64);
        [unknown, exact_zero, exact_maximum, ranged]
            .into_iter()
            .fold((), |(), native_hint| {
                let expected =
                    to_err_string::error_text::ErrorText::try_from(format!("{native_hint:#?}"))
                        .unwrap_or_else(|error| std::panic::panic_any(error));
                let expected_upper = native_hint.upper();
                let wrapped_hint = crate::http_body_size_hint::HttpBodySizeHint::from(native_hint);
                assert_eq!(wrapped_hint.upper(), expected_upper);
                assert_eq!(
                    to_err_string::to_err_string::ToErrString::to_err_string(&wrapped_hint),
                    expected
                );
            });
    }
}
