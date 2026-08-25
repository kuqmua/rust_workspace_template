pub(super) fn hash_opaque_token(
    token: &super::AdminOpaqueToken,
) -> Result<super::AdminTokenHash, super::AdminSecretTextError> {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(
        secrecy::ExposeSecret::expose_secret(token.0.as_ref()).as_bytes(),
    );
    let hash = base16ct::lower::encode_string(&digest);
    Ok(super::SecrecyAdminString::try_from(hash).map(super::AdminTokenHash::new)?)
}
