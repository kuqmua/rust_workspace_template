use super::*;
pub fn decode_access_token(
    token: &StdAdminAccessToken,
    secret: &AdminJwtSecret,
    issuer: &config_lib::domain_types::AdminTokenIssuer,
    audience: &config_lib::domain_types::AdminTokenAudience,
) -> Result<AdminAccessClaims, AdminAccessTokenError> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[issuer.as_ref()]);
    validation.set_audience(&[audience.as_ref()]);
    jsonwebtoken::decode::<AdminAccessClaims>(
        token.as_ref(),
        &jsonwebtoken::DecodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(JsonwebtokenAdminError::from)
    .map_err(AdminAccessTokenError::from)
}
