pub(super) async fn authenticate(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
    peer: super::AdminPeerAddr,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let token = super::super::find_admin_cookie(headers, super::super::AdminCookieKind::Access)
        .ok_or(super::AdminError::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.issuer.as_ref()]);
    validation.set_audience(&[state.audience.as_ref()]);
    let claims = state
        .decoding_keys
        .as_ref()
        .iter()
        .find_map(|decoding_key| {
            jsonwebtoken::decode::<super::super::AdminAccessClaims>(
                token.as_ref(),
                decoding_key,
                &validation,
            )
            .ok()
            .map(|data| data.claims)
        })
        .ok_or(super::AdminError::Authentication)?;
    let context_hash =
        super::authorization_session_context_hash::session_context_hash(headers, peer)
            .map_err(super::AdminError::secret_text)?;
    let active =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_ACTIVE_ACCESS_SESSION_SQL)
            .bind(claims.session_id().get().get())
            .bind(claims.user_id().get())
            .bind(context_hash.expose().as_ref())
            .fetch_one(state.pool.as_ref())
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map(crate::domain_types::StdAdminBool::from)
            .map_err(super::AdminError::postgresql)?;
    if !active.get() {
        return Err(super::AdminError::Authentication);
    }
    super::persistence::load_authenticated_admin(state, claims.user_id(), claims.session_id()).await
}
