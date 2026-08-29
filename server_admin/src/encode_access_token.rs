pub fn encode_access_token(
    claims: &crate::admin_access_claims::AdminAccessClaims,
    secret: &crate::admin_jwt_secret::AdminJwtSecret,
) -> Result<
    crate::std_admin_access_token::StdAdminAccessToken,
    crate::admin_access_token_error::AdminAccessTokenError,
> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
    )
    .map(crate::std_admin_access_token::StdAdminAccessToken)
    .map_err(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError::from)
    .map_err(crate::admin_access_token_error::AdminAccessTokenError::from)
}
