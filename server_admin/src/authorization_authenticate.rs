pub(crate) async fn authorization_authenticate(
    admin_auth_svc_state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    http_admin_header_map_ref: crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'_>,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
) -> Result<
    crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    let token = crate::find_admin_cookie::find_admin_cookie(
        http_admin_header_map_ref,
        crate::admin_cookie_kind::AdminCookieKind::Access,
    )
    .ok_or(crate::admin_error::AdminError::Authentication)?;
    let validation = crate::admin_access_token_validation::admin_access_token_validation();
    let claims = admin_auth_svc_state
        .get_decoding_keys()
        .as_ref()
        .iter()
        .find_map(|decoding_key| {
            let data = jsonwebtoken::decode::<crate::admin_access_claims::AdminAccessClaims>(
                token.as_ref(),
                decoding_key,
                &validation,
            )
            .ok()?;
            match crate::validate_admin_access_claims::validate_admin_access_claims(
                &data.claims,
                admin_auth_svc_state.get_issuer(),
                admin_auth_svc_state.get_audience(),
            ) {
                Ok(()) => Some(data.claims),
                Err(error) => {
                    drop(error);
                    None
                }
            }
        })
        .ok_or(crate::admin_error::AdminError::Authentication)?;
    let context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            http_admin_header_map_ref,
            admin_peer_addr,
        )
        .map_err(crate::admin_error::AdminError::secret_text)?;
    let active =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_ACTIVE_ACCESS_SESSION_SQL)
            .bind(claims.session_id().get().get())
            .bind(claims.user_id().get())
            .bind(context_hash.expose().as_ref())
            .fetch_one(admin_auth_svc_state.get_pool().as_ref())
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map(server_admin_core::std_admin_bool::StdAdminBool::from)
            .map_err(crate::admin_error::AdminError::postgresql)?;
    if !active.get() {
        return Err(crate::admin_error::AdminError::Authentication);
    }
    crate::load_authenticated_admin_from_db::load_authenticated_admin_from_db(
        &mut crate::admin_db_ref::AdminDbRef::Pool(
            crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
                admin_auth_svc_state.get_pool().as_ref(),
            ),
        ),
        claims.user_id(),
        claims.session_id(),
    )
    .await
}
