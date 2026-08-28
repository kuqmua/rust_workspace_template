pub(crate) fn hash_opaque_token(
    token: &crate::AdminOpaqueToken,
) -> Result<crate::AdminTokenHash, crate::AdminSecretTextError> {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(token.expose().as_ref().as_bytes());
    let hash = base16ct::lower::encode_string(&digest);
    Ok(crate::SecrecyAdminString::try_from(hash).map(crate::AdminTokenHash::new)?)
}
