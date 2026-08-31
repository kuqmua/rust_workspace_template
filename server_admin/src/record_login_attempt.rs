pub(crate) async fn record_login_attempt(
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    login: &server_admin_contract::admin_login::AdminLogin,
    peer: crate::admin_peer_addr::AdminPeerAddr,
    succeeded: server_admin_core::std_admin_bool::StdAdminBool,
) -> Result<(), crate::admin_error::AdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
        .bind(login.as_ref())
        .bind(peer.socket_addr().get().ip())
        .bind(succeeded.get())
        .bind(uuid::Uuid::new_v4())
        .execute(state.pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
        .map_err(crate::admin_error::AdminError::postgresql)
}
