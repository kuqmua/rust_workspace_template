pub(crate) async fn load_authenticated_admin(
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    session_id: crate::admin_session_id::AdminSessionId,
) -> Result<
    crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    let mut db = crate::admin_db_ref::AdminDbRef::Pool(
        crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
            state.get_pool().as_ref(),
        ),
    );
    crate::load_authenticated_admin_from_db::load_authenticated_admin_from_db(
        &mut db, user_id, session_id,
    )
    .await
}
