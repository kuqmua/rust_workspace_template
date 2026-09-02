pub fn encode_access_token(
    admin_access_claims: &crate::admin_access_claims::AdminAccessClaims,
    runtime_admin_jwt_secret: &crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret,
) -> Result<
    crate::std_admin_access_token::StdAdminAccessToken,
    crate::admin_access_token_error::AdminAccessTokenError,
> {
    let encoded = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        admin_access_claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(runtime_admin_jwt_secret.get_inner().as_ref())
                .as_bytes(),
        ),
    )
    .map_err(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError::from)
    .map_err(crate::admin_access_token_error::AdminAccessTokenError::from)?;
    crate::std_admin_access_token::StdAdminAccessToken::try_from(encoded)
        .map_err(crate::admin_access_token_error::AdminAccessTokenError::from)
}
