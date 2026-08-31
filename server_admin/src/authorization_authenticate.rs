pub(crate) async fn authorization_authenticate(
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    headers: crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'_>,
    peer: crate::admin_peer_addr::AdminPeerAddr,
) -> Result<
    crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    let token = crate::find_admin_cookie::find_admin_cookie(
        headers,
        crate::admin_cookie_kind::AdminCookieKind::Access,
    )
    .ok_or(crate::admin_error::AdminError::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.get_issuer().as_ref()]);
    validation.set_audience(&[state.get_audience().as_ref()]);
    let claims = state
        .get_decoding_keys()
        .as_ref()
        .iter()
        .find_map(|decoding_key| {
            jsonwebtoken::decode::<crate::admin_access_claims::AdminAccessClaims>(
                token.as_ref(),
                decoding_key,
                &validation,
            )
            .ok()
            .map(|data| data.claims)
        })
        .ok_or(crate::admin_error::AdminError::Authentication)?;
    let context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            headers, peer,
        )
        .map_err(crate::admin_error::AdminError::secret_text)?;
    let active =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_ACTIVE_ACCESS_SESSION_SQL)
            .bind(claims.session_id().get().get())
            .bind(claims.user_id().get())
            .bind(context_hash.expose().as_ref())
            .fetch_one(state.get_pool().as_ref())
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map(server_admin_core::std_admin_bool::StdAdminBool::from)
            .map_err(crate::admin_error::AdminError::postgresql)?;
    if !active.get() {
        return Err(crate::admin_error::AdminError::Authentication);
    }
    crate::load_authenticated_admin::load_authenticated_admin(
        state,
        claims.user_id(),
        claims.session_id(),
    )
    .await
}
