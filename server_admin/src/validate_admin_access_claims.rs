pub(super) fn validate_admin_access_claims(
    admin_access_claims: &crate::admin_access_claims::AdminAccessClaims,
    admin_token_issuer: &config_lib::admin_token_issuer::AdminTokenIssuer,
    admin_token_audience: &config_lib::admin_token_audience::AdminTokenAudience,
) -> Result<(), crate::jsonwebtoken_admin_error::JsonwebtokenAdminError> {
    let error_kind = if admin_access_claims.get_issuer() != admin_token_issuer {
        Some(jsonwebtoken::errors::ErrorKind::InvalidIssuer)
    } else if admin_access_claims.get_audience() != admin_token_audience {
        Some(jsonwebtoken::errors::ErrorKind::InvalidAudience)
    } else if *admin_access_claims.get_expires_at().get_inner()
        <= jsonwebtoken::get_current_timestamp()
    {
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
