pub(crate) async fn record_login_attempt(
    state: &crate::AdminAuthSvcState,
    login: &crate::AdminLogin,
    peer: crate::AdminPeerAddr,
    succeeded: crate::StdAdminBool,
) -> Result<(), crate::AdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
        .bind(login.as_ref())
        .bind(peer.socket_addr().get().ip())
        .bind(succeeded.get())
        .bind(uuid::Uuid::new_v4())
        .execute(state.pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
        .map_err(crate::AdminError::postgresql)
}
