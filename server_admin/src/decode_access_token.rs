pub fn decode_access_token(
    token: &crate::std_admin_access_token::StdAdminAccessToken,
    secret: &crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret,
    issuer: &config_lib::admin_token_issuer::AdminTokenIssuer,
    audience: &config_lib::admin_token_audience::AdminTokenAudience,
) -> Result<
    crate::admin_access_claims::AdminAccessClaims,
    crate::admin_access_token_error::AdminAccessTokenError,
> {
    let validation = crate::admin_access_token_validation::admin_access_token_validation();
    jsonwebtoken::decode::<crate::admin_access_claims::AdminAccessClaims>(
        token.as_ref(),
        &jsonwebtoken::DecodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.get_inner().as_ref()).as_bytes(),
        ),
        &validation,
    )
    .map_err(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError::from)
    .and_then(|data| {
        crate::validate_admin_access_claims::validate_admin_access_claims(
            &data.claims,
            issuer,
            audience,
        )?;
        Ok(data.claims)
    })
    .map_err(crate::admin_access_token_error::AdminAccessTokenError::from)
}
