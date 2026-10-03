pub(super) fn parse_bounded_json_owned(
    bounded_bytes: crate::bounded_bytes::BoundedBytes,
) -> Result<
    crate::bounded_json_text::BoundedJsonText,
    crate::bounded_json_read_error::BoundedJsonReadError,
> {
    let text = String::from_utf8(bounded_bytes.into_inner()).map_err(|error| {
        crate::bounded_json_read_error::BoundedJsonReadError::Read(
            crate::bounded_read_error::BoundedReadError::Utf8 {
                source: crate::bounded_read_from_utf8_error::BoundedReadFromUtf8Error::from(error),
            },
        )
    })?;
    crate::bounded_json_text::BoundedJsonText::try_from(text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_owned_and_borrowed_json_byte_parsers_preserve_original_formatting() {
        let mut padded = ' '.to_string();
        padded.push_str(constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY);
        padded.push('\n');
        assert!(
            [
                constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY.to_owned(),
                padded
            ]
            .into_iter()
            .all(|text| {
                let bytes = crate::bounded_bytes::BoundedBytes::from(text.as_bytes().to_vec());
                let owned =
                    crate::parse_bounded_json_owned::parse_bounded_json_owned(bytes.clone());
                let borrowed = crate::parse_bounded_json::parse_bounded_json(&bytes);
                [owned, borrowed]
                    .into_iter()
                    .all(|result| result.is_ok_and(|json| json.as_ref() == text))
                    && bytes.into_inner() == text.as_bytes()
            })
        );
    }

    #[test]
    fn test_owned_json_byte_parser_preserves_invalid_utf8_details() {
        assert!([vec![0xffu8], vec![b'x', 0xffu8], vec![0xc3u8], vec![0xc3u8, b'(']]
            .into_iter().all(|bytes| {
                String::from_utf8(bytes.clone()).is_err_and(|expected| {
                    crate::parse_bounded_json_owned::parse_bounded_json_owned(
                        crate::bounded_bytes::BoundedBytes::from(bytes),
                    ).is_err_and(|error| match error {
                        crate::bounded_json_read_error::BoundedJsonReadError::Read(
                            crate::bounded_read_error::BoundedReadError::Utf8 { source }
                        ) => source.to_string() == expected.to_string()
                            && format!("{source:?}") == format!("{:?}", crate::bounded_read_from_utf8_error::BoundedReadFromUtf8Error::from(expected)),
                        crate::bounded_json_read_error::BoundedJsonReadError::Read(_)
                        | crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(_) => false,
                    })
                })
            }));
    }
}
