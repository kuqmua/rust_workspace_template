#[cfg(test)]
mod tests {
    #[test]
    fn test_wire_token_requires_exactly_three_nonempty_valid_parts() {
        assert!([0usize, 1usize, 2usize, 4usize].into_iter().all(|count| {
            let text = vec![constants_str::X; count].join(&'.'.to_string());
            crate::versioned_url_safe_wire_token_text::VersionedUrlSafeWireTokenText::try_from(text)
                == Err(crate::versioned_url_safe_wire_token_text_error::VersionedUrlSafeWireTokenTextError::InvalidStructure)
        }));
        assert!((0usize..3usize).all(|position| {
            [String::default(), '/'.to_string(), '\u{e9}'.to_string(), constants_str::X.repeat(4097usize)]
                .into_iter().all(|invalid| {
                    let text = (0usize..3usize).map(|index| if index == position { invalid.as_str() } else { constants_str::X })
                        .collect::<Vec<_>>().join(&'.'.to_string());
                    crate::versioned_url_safe_wire_token_text::VersionedUrlSafeWireTokenText::try_from(text)
                        == Err(crate::versioned_url_safe_wire_token_text_error::VersionedUrlSafeWireTokenTextError::InvalidPart)
                })
        }));
    }

    #[test]
    fn test_wire_token_total_size_limit_preserves_each_component() {
        let payload = constants_str::X.repeat(4096usize);
        assert!([8191usize, 8192usize, 8193usize].into_iter().all(|length| {
            let signature = constants_str::X.repeat(length.saturating_sub(4099usize));
            let text = format!("{}.{payload}.{signature}", constants_str::X);
            let result = crate::versioned_url_safe_wire_token_text::VersionedUrlSafeWireTokenText::try_from(text);
            if length <= 8192usize {
                result.is_ok_and(|token| token.version().as_ref() == constants_str::X
                    && token.encoded_payload().as_ref() == payload
                    && token.encoded_signature().as_ref() == signature)
            } else {
                result == Err(crate::versioned_url_safe_wire_token_text_error::VersionedUrlSafeWireTokenTextError::TooLong)
            }
        }));
    }

    #[test]
    fn test_versioned_wire_token_splits_valid_parts() {
        let value =
            crate::versioned_url_safe_wire_token_text::VersionedUrlSafeWireTokenText::try_from(
                constants_str::TEST_VERSIONED_URL_SAFE_WIRE_TOKEN.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_8C3D9457);
        assert_eq!(value.version().as_ref(), constants_str::TEST_TOKEN_VERSION);
        assert_eq!(
            value.encoded_payload().as_ref(),
            constants_str::TEST_TOKEN_PAYLOAD
        );
        assert_eq!(
            value.encoded_signature().as_ref(),
            constants_str::TEST_TOKEN_SIGNATURE
        );
    }
}
