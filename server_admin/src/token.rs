#![allow(clippy::single_call_fn)] // stable root token API delegates to the private cryptographic responsibility module
pub(super) fn generate_token() -> super::AdminGeneratedToken {
    let token =
        super::AdminOpaqueToken::new(super::SecrecyAdminString::from(secrecy::SecretBox::new(
            Box::new(format!("{}.{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4())),
        )));
    let hash = hash_opaque_token(&token);
    super::AdminGeneratedToken { hash, token }
}
pub(super) fn hash_opaque_token(token: &super::AdminOpaqueToken) -> super::AdminTokenHash {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(
        secrecy::ExposeSecret::expose_secret(token.0.as_ref()).as_bytes(),
    );
    super::AdminTokenHash::new(super::SecrecyAdminString::from(secrecy::SecretBox::new(
        Box::new(format!("{digest:x}")),
    )))
}
pub(super) fn encode_access_token(
    claims: &super::AdminAccessClaims,
    secret: &super::AdminJwtSecret,
) -> Result<super::StdAdminAccessToken, super::AdminAccessTokenEr> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
    )
    .map(super::StdAdminAccessToken)
    .map_err(|er| super::AdminAccessTokenEr(super::JsonwebtokenAdminEr::from(er)))
}
pub(super) fn decode_access_token(
    token: &super::StdAdminAccessToken,
    secret: &super::AdminJwtSecret,
    issuer: &super::AdminTokenIssuer,
    audience: &super::AdminTokenAudience,
) -> Result<super::AdminAccessClaims, super::AdminAccessTokenEr> {
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
    .map_err(|er| super::AdminAccessTokenEr(super::JsonwebtokenAdminEr::from(er)))
}
