pub(crate) async fn record_login_attempt(
    db: &mut crate::admin_db_ref::AdminDbRef<'_, '_>,
    login: &server_admin_contract::admin_login::AdminLogin,
    peer: crate::admin_peer_addr::AdminPeerAddr,
    succeeded: server_admin_core::std_admin_bool::StdAdminBool,
) -> Result<(), crate::admin_error::AdminError> {
    let attempt_id = uuid::Uuid::new_v4();
    let query = || {
        sqlx::query(constants_str::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
            .bind(login.as_ref())
            .bind(peer.socket_addr().get().ip())
            .bind(succeeded.get())
            .bind(attempt_id)
    };
    match db {
        crate::admin_db_ref::AdminDbRef::Connection(connection) => {
            query().execute(&mut ***connection).await
        }
        crate::admin_db_ref::AdminDbRef::Pool(pool) => query().execute(&***pool).await,
    }
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map(drop)
    .map_err(crate::admin_error::AdminError::postgresql)
}
