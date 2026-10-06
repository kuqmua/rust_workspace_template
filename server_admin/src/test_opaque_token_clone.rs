#[test]
fn test_opaque_token_clones_preserve_independent_bounded_secret_storage_and_redaction() {
    assert!(
        [
            String::new(),
            constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES.to_owned(),
            constants_str::X.repeat(8_192usize),
            '\u{00e9}'.to_string().repeat(4_096usize),
        ]
        .into_iter()
        .all(|text| {
            let length = text.len();
            server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(text).is_ok_and(
                |secret| {
                    let original = crate::admin_opaque_token::AdminOpaqueToken::new(secret);
                    let cloned =
                        crate::admin_opaque_token::AdminOpaqueToken::new(original.clone_secret());
                    assert!(
                        cloned
                            .expose()
                            .as_ref()
                            .bytes()
                            .eq(original.expose().as_ref().bytes()),
                    );
                    assert!(
                        length == 0usize
                            || cloned.expose().as_ref().as_ptr()
                                != original.expose().as_ref().as_ptr(),
                    );
                    let original_debug = format!("{original:?}");
                    drop(original);
                    assert_eq!(cloned.expose().as_ref().len(), length);
                    assert!(cloned.expose().as_ref().bytes().all(|byte| byte != 0u8));
                    assert_eq!(format!("{cloned:?}"), original_debug);
                    assert!(original_debug.contains(constants_str::REDACTED_ALT_3));
                    true
                },
            )
        }),
    );
}
