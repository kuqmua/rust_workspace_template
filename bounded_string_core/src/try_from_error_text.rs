pub fn try_from_error_text<Value, Error>(error: Error) -> Value
where
    Value: TryFrom<String, Error = Error>,
    Error: std::fmt::Display,
{
    let mut text = error.to_string();
    loop {
        match Value::try_from(text) {
            Ok(value) => return value,
            Err(conversion_error) => text = conversion_error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_try_from_error_text_retries_with_display_text() {
        let error = crate::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
            actual_length: 2usize,
            maximum_length: 1usize,
        };
        let bounded_string_storage = super::try_from_error_text::<
            crate::bounded_string_storage::BoundedStringStorage<0usize, 1_024usize, false>,
            crate::bounded_string_storage_error::BoundedStringStorageError,
        >(error);
        assert!(!bounded_string_storage.is_empty());
    }
}
