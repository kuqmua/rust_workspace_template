pub(in super::super) async fn record_login_attempt(
    state: &super::super::AdminAuthSvcState,
    login: &super::super::super::AdminLogin,
    peer: super::super::AdminPeerAddr,
    succeeded: super::super::super::StdAdminBool,
) -> Result<(), super::super::AdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
        .bind(login.as_ref())
        .bind(peer.socket_addr().get().ip())
        .bind(succeeded.get())
        .bind(uuid::Uuid::new_v4())
        .execute(state.pool.as_ref())
        .await
        .map_err(super::super::super::SqlxAdminError::from)
        .map(drop)
        .map_err(super::super::AdminError::postgresql)
}
