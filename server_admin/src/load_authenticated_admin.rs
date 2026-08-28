pub(crate) async fn load_authenticated_admin(
    state: &crate::AdminAuthSvcState,
    user_id: crate::AdminUserId,
    session_id: crate::AdminSessionId,
) -> Result<crate::AuthenticatedAdmin, crate::AdminError> {
    let mut db = crate::AdminDbRef::Pool(crate::repository::SqlxAdminRepositoryPoolRef::from(
        state.pool.as_ref(),
    ));
    crate::load_authenticated_admin_from_db(&mut db, user_id, session_id).await
}
