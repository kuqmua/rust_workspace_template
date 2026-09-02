pub fn decode_access_token(
    std_admin_access_token: &crate::std_admin_access_token::StdAdminAccessToken,
    runtime_admin_jwt_secret: &crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret,
    admin_token_issuer: &config_lib::admin_token_issuer::AdminTokenIssuer,
    admin_token_audience: &config_lib::admin_token_audience::AdminTokenAudience,
) -> Result<
    crate::admin_access_claims::AdminAccessClaims,
    crate::admin_access_token_error::AdminAccessTokenError,
> {
    let validation = crate::admin_access_token_validation::admin_access_token_validation();
    jsonwebtoken::decode::<crate::admin_access_claims::AdminAccessClaims>(
        std_admin_access_token.as_ref(),
        &jsonwebtoken::DecodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(runtime_admin_jwt_secret.get_inner().as_ref())
                .as_bytes(),
        ),
        &validation,
    )
    .map_err(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError::from)
    .and_then(|data| {
        crate::validate_admin_access_claims::validate_admin_access_claims(
            &data.claims,
            admin_token_issuer,
            admin_token_audience,
        )?;
        Ok(data.claims)
    })
    .map_err(crate::admin_access_token_error::AdminAccessTokenError::from)
}
