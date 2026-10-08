#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub(crate) struct RuntimeTestUrl(
    bounded_types::bounded_string::BoundedString<0usize, 8_192usize, false>,
);

impl TryFrom<String> for RuntimeTestUrl {
    type Error = crate::service_base_url_error::ServiceBaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(crate::service_base_url_error::ServiceBaseUrlError::Length)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => crate::service_base_url_error::ServiceBaseUrlError::Length,
                })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_runtime_test_url_preserves_empty_text_and_exact_utf8_byte_limits() {
        assert!(
            crate::runtime_test_url::RuntimeTestUrl::try_from(String::new())
                .is_ok_and(|runtime_test_url| runtime_test_url.as_ref().is_empty())
        );
        let maximum = constants_usize::VALUE_8_192;
        assert!(
            [maximum - 1usize, maximum, maximum + 1usize]
                .into_iter()
                .all(|length| {
                    [
                        constants_str::X,
                        constants_str::NON_ASCII_U_E9,
                        constants_str::TEST_TEXT_WITH_NUL,
                    ]
                    .into_iter()
                    .all(|suffix| {
                        let prefix_length = length - suffix.len();
                        let mut text = constants_str::X.repeat(prefix_length);
                        text.push_str(suffix);
                        let pointer = text.as_ptr();
                        let result = crate::runtime_test_url::RuntimeTestUrl::try_from(text);
                        if length <= maximum {
                            result.is_ok_and(|runtime_test_url| {
                                runtime_test_url.as_ref().len() == length
                                    && runtime_test_url.as_ref().as_ptr() == pointer
                                    && runtime_test_url.as_ref().ends_with(suffix)
                                    && runtime_test_url
                                        .as_ref()
                                        .as_bytes()
                                        .iter()
                                        .take(prefix_length)
                                        .all(|byte| *byte == b'x')
                            })
                        } else {
                            result
                                == Err(crate::service_base_url_error::ServiceBaseUrlError::Length)
                        }
                    })
                })
        );
    }
}
