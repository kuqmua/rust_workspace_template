pub(in super::super) async fn load_authenticated_admin(
    state: &super::super::AdminAuthSvcState,
    user_id: super::super::super::AdminUserId,
    session_id: super::super::super::AdminSessionId,
) -> Result<super::super::AuthenticatedAdmin, super::super::AdminError> {
    let mut db = super::AdminDbRef::Pool(crate::repository::SqlxAdminRepositoryPoolRef::from(
        state.pool.as_ref(),
    ));
    super::load_authenticated_admin_from_db(&mut db, user_id, session_id).await
}
