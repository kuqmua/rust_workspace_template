pub(super) fn validate_admin_access_claims(
    claims: &crate::admin_access_claims::AdminAccessClaims,
    issuer: &config_lib::admin_token_issuer::AdminTokenIssuer,
    audience: &config_lib::admin_token_audience::AdminTokenAudience,
) -> Result<(), crate::jsonwebtoken_admin_error::JsonwebtokenAdminError> {
    let error_kind = if claims.get_issuer() != issuer {
        Some(jsonwebtoken::errors::ErrorKind::InvalidIssuer)
    } else if claims.get_audience() != audience {
        Some(jsonwebtoken::errors::ErrorKind::InvalidAudience)
    } else if *claims.get_expires_at().get_inner() <= jsonwebtoken::get_current_timestamp() {
        Some(jsonwebtoken::errors::ErrorKind::ExpiredSignature)
    } else {
        None
    };
    error_kind.map_or(Ok(()), |error_kind| {
        Err(
            crate::jsonwebtoken_admin_error::JsonwebtokenAdminError::from(
                jsonwebtoken::errors::Error::from(error_kind),
            ),
        )
    })
}
