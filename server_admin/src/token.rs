#![allow(clippy::single_call_fn)] // stable root token API delegates to the private cryptographic responsibility module
pub(super) fn generate_token() -> Result<super::AdminGeneratedToken, super::AdminSecretTextError> {
    let token = super::SecrecyAdminString::try_from(format!(
        "{}.{}",
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4()
    ))
    .map(super::AdminOpaqueToken::new)?;
    hash_opaque_token(&token).map(|hash| super::AdminGeneratedToken { hash, token })
}
pub(super) fn hash_opaque_token(
    token: &super::AdminOpaqueToken,
) -> Result<super::AdminTokenHash, super::AdminSecretTextError> {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(
        secrecy::ExposeSecret::expose_secret(token.0.as_ref()).as_bytes(),
    );
    let hash = base16ct::lower::encode_string(&digest);
    Ok(super::SecrecyAdminString::try_from(hash).map(super::AdminTokenHash::new)?)
}
pub(super) fn encode_access_token(
    claims: &super::AdminAccessClaims,
    secret: &super::AdminJwtSecret,
) -> Result<super::StdAdminAccessToken, super::AdminAccessTokenError> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
    )
    .map(super::StdAdminAccessToken)
    .map_err(|error| super::AdminAccessTokenError(super::JsonwebtokenAdminError::from(error)))
}
pub(super) fn decode_access_token(
    token: &super::StdAdminAccessToken,
    secret: &super::AdminJwtSecret,
    issuer: &config_lib::AdminTokenIssuer,
    audience: &config_lib::AdminTokenAudience,
) -> Result<super::AdminAccessClaims, super::AdminAccessTokenError> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[issuer.as_ref()]);
    validation.set_audience(&[audience.as_ref()]);
    jsonwebtoken::decode::<super::AdminAccessClaims>(
        token.as_ref(),
        &jsonwebtoken::DecodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|error| super::AdminAccessTokenError(super::JsonwebtokenAdminError::from(error)))
}
