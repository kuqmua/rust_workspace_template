pub(crate) fn hash_opaque_token(
    admin_opaque_token: &crate::admin_opaque_token::AdminOpaqueToken,
) -> Result<
    crate::admin_token_hash::AdminTokenHash,
    crate::admin_secret_text_error::AdminSecretTextError,
> {
    let digest =
        <sha2::Sha256 as sha2::Digest>::digest(admin_opaque_token.expose().as_ref().as_bytes());
    let hash = base16ct::lower::encode_string(&digest);
    Ok(
        server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(hash)
            .map(crate::admin_token_hash::AdminTokenHash::new)?,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_opaque_token_hash_matches_standard_sha256_vectors() {
        assert!(
            [
                (
                    String::new(),
                    [
                        0xe3u8, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8,
                        0x99, 0x6f, 0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
                        0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
                    ]
                ),
                (
                    ['a', 'b', 'c'].into_iter().collect::<String>(),
                    [
                        0xbau8, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde,
                        0x5d, 0xae, 0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c,
                        0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
                    ]
                ),
            ]
            .into_iter()
            .all(|(input, expected)| {
                server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(input)
                    .is_ok_and(|secret| {
                        let token = crate::admin_opaque_token::AdminOpaqueToken::new(secret);
                        crate::token::token(&token).is_ok_and(|hash| {
                            let text = hash.expose();
                            text.as_ref() == base16ct::lower::encode_string(&expected)
                                && text.as_ref().len() == 64usize
                                && text.as_ref().bytes().all(|byte| {
                                    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
                                })
                        })
                    })
            })
        );
    }
}
