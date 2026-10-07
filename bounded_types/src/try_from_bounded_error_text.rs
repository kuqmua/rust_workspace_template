pub fn try_from_bounded_error_text<Value, Error>(error: Error) -> Result<Value, Error>
where
    Value: TryFrom<String, Error = Error>,
    Error: std::fmt::Display,
{
    bounded_string_core::try_from_error_text::try_from_error_text(error)
}

#[cfg(test)]
mod tests {
    fn bounded_error_text_fixture() -> crate::bounded_string_error::BoundedStringError {
        crate::bounded_string_error::BoundedStringError::AboveMaximum {
            actual_length: crate::bounded_len::BoundedLen::from(2usize),
            maximum_length: crate::bounded_len::BoundedLen::from(1usize),
        }
    }

    #[test]
    fn test_try_from_bounded_error_text_accepts_display_text() {
        let error = bounded_error_text_fixture();
        assert!(matches!(
            super::try_from_bounded_error_text::<
                crate::bounded_string::BoundedString<0usize, 1_024usize, false>,
                crate::bounded_string_error::BoundedStringError,
            >(error),
            Ok(value) if value.as_str() == error.to_string()
        ));
    }

    #[test]
    fn test_try_from_bounded_error_text_rejects_short_target() {
        let error = bounded_error_text_fixture();
        assert_eq!(
            super::try_from_bounded_error_text::<
                crate::bounded_string::BoundedString<0usize, 1usize, false>,
                crate::bounded_string_error::BoundedStringError,
            >(error),
            Err(
                crate::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length: crate::bounded_len::BoundedLen::from(error.to_string().len()),
                    maximum_length: crate::bounded_len::BoundedLen::from(1usize),
                }
            )
        );
    }

    #[test]
    fn test_try_from_bounded_error_text_preserves_lower_bound_rejection_lengths() {
        let error = bounded_error_text_fixture();
        assert_eq!(
            super::try_from_bounded_error_text::<
                crate::bounded_string::BoundedString<1_024usize, 2_048usize, false>,
                crate::bounded_string_error::BoundedStringError,
            >(error),
            Err(
                crate::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length: crate::bounded_len::BoundedLen::from(error.to_string().len()),
                    minimum_length: crate::bounded_len::BoundedLen::from(1_024usize),
                }
            )
        );
    }
}
