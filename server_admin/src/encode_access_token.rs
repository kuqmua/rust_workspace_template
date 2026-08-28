use crate::*;
pub fn encode_access_token(
    claims: &AdminAccessClaims,
    secret: &AdminJwtSecret,
) -> Result<StdAdminAccessToken, AdminAccessTokenError> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
    )
    .map(StdAdminAccessToken)
    .map_err(JsonwebtokenAdminError::from)
    .map_err(AdminAccessTokenError::from)
}
