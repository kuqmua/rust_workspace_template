pub fn try_from_error_text<Value, Error>(error: Error) -> Result<Value, Error>
where
    Value: TryFrom<String, Error = Error>,
    Error: std::fmt::Display,
{
    Value::try_from(error.to_string())
}

#[cfg(test)]
mod tests {
    fn core_error_text_fixture() -> crate::bounded_string_storage_error::BoundedStringStorageError {
        crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
            actual_length: 2usize,
            maximum_length: 1usize,
        }
    }

    #[test]
    fn test_try_from_error_text_accepts_display_text() {
        let error = core_error_text_fixture();
        let bounded_string_storage = super::try_from_error_text::<
            crate::bounded_string_storage::BoundedStringStorage<0usize, 1_024usize, false>,
            crate::bounded_string_storage_error::BoundedStringStorageError,
        >(error);
        assert!(matches!(bounded_string_storage, Ok(value) if value.as_str() == error.to_string()));
    }

    #[test]
    fn test_try_from_error_text_rejects_short_target() {
        let error = core_error_text_fixture();
        assert!(matches!(
            super::try_from_error_text::<
                crate::bounded_string_storage::BoundedStringStorage<0usize, 1usize, false>,
                crate::bounded_string_storage_error::BoundedStringStorageError,
            >(error),
            Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                    maximum_length: 1usize,
                    ..
                }
            )
        ));
    }

    #[test]
    fn test_try_from_error_text_rejects_large_minimum() {
        let error = crate::bounded_string_storage_error::BoundedStringStorageError::BelowMinimum {
            actual_length: 1usize,
            minimum_length: 1_024usize,
        };
        assert!(matches!(
            super::try_from_error_text::<
                crate::bounded_string_storage::BoundedStringStorage<1_024usize, 2_048usize, false>,
                crate::bounded_string_storage_error::BoundedStringStorageError,
            >(error),
            Err(
                crate::bounded_string_storage_error::BoundedStringStorageError::BelowMinimum {
                    minimum_length: 1_024usize,
                    ..
                }
            )
        ));
    }
}
