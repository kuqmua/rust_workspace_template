pub fn decode_access_token(
    token: &crate::std_admin_access_token::StdAdminAccessToken,
    secret: &crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret,
    issuer: &config_lib::admin_token_issuer::AdminTokenIssuer,
    audience: &config_lib::admin_token_audience::AdminTokenAudience,
) -> Result<
    crate::admin_access_claims::AdminAccessClaims,
    crate::admin_access_token_error::AdminAccessTokenError,
> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[issuer.as_ref()]);
    validation.set_audience(&[audience.as_ref()]);
    jsonwebtoken::decode::<crate::admin_access_claims::AdminAccessClaims>(
        token.as_ref(),
        &jsonwebtoken::DecodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.get_inner().as_ref()).as_bytes(),
        ),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError::from)
    .map_err(crate::admin_access_token_error::AdminAccessTokenError::from)
}
