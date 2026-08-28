pub(crate) async fn authorization_authenticate(
    state: &crate::AdminAuthSvcState,
    headers: crate::HttpAdminHeaderMapRef<'_>,
    peer: crate::AdminPeerAddr,
) -> Result<crate::AuthenticatedAdmin, crate::AdminError> {
    let token = crate::find_admin_cookie(headers, crate::AdminCookieKind::Access)
        .ok_or(crate::AdminError::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.issuer.as_ref()]);
    validation.set_audience(&[state.audience.as_ref()]);
    let claims = state
        .decoding_keys
        .as_ref()
        .iter()
        .find_map(|decoding_key| {
            jsonwebtoken::decode::<crate::AdminAccessClaims>(
                token.as_ref(),
                decoding_key,
                &validation,
            )
            .ok()
            .map(|data| data.claims)
        })
        .ok_or(crate::AdminError::Authentication)?;
    let context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            headers, peer,
        )
        .map_err(crate::AdminError::secret_text)?;
    let active =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_ACTIVE_ACCESS_SESSION_SQL)
            .bind(claims.session_id().get().get())
            .bind(claims.user_id().get())
            .bind(context_hash.expose().as_ref())
            .fetch_one(state.pool.as_ref())
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map(crate::domain_types::StdAdminBool::from)
            .map_err(crate::AdminError::postgresql)?;
    if !active.get() {
        return Err(crate::AdminError::Authentication);
    }
    crate::persistence::load_authenticated_admin(state, claims.user_id(), claims.session_id()).await
}
