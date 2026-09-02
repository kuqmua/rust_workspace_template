pub(crate) async fn record_login_attempt(
    admin_db_ref: &mut crate::admin_db_ref::AdminDbRef<'_, '_>,
    admin_login: &server_admin_contract::admin_login::AdminLogin,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
    std_admin_bool: server_admin_core::std_admin_bool::StdAdminBool,
) -> Result<(), crate::admin_error::AdminError> {
    let attempt_id = uuid::Uuid::new_v4();
    let query = || {
        sqlx::query(constants_str::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
            .bind(admin_login.as_ref())
            .bind(admin_peer_addr.socket_addr().get().ip())
            .bind(std_admin_bool.get())
            .bind(attempt_id)
    };
    match admin_db_ref {
        crate::admin_db_ref::AdminDbRef::Connection(connection) => {
            query().execute(&mut ***connection).await
        }
        crate::admin_db_ref::AdminDbRef::Pool(pool) => query().execute(&***pool).await,
    }
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map(drop)
    .map_err(crate::admin_error::AdminError::postgresql)
}
