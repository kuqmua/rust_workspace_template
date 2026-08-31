#[cfg(test)]
mod tests {
    #[test]
    fn test_versioned_wire_token_splits_valid_parts() {
        let value =
            crate::versioned_url_safe_wire_token_text::VersionedUrlSafeWireTokenText::try_from(
                constants_str::TEST_VERSIONED_URL_SAFE_WIRE_TOKEN.to_owned(),
            )
            .expect("8c3d9457 versioned_wire_token_splits_valid_parts invariant must hold");
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

// Root-owned module compatibility wrappers.
