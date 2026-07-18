#![allow(clippy::single_call_fn)] // each MFA function owns one transactional SQL contract

pub(crate) async fn status(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<
    (
        crate::StdAdminBool,
        server_admin_contract::AdminOperationalCount,
    ),
    super::AdminRepositoryError,
> {
    let (enabled, remaining) =
        sqlx::query_as::<_, (bool, i64)>(str_constants::SERVER_ADMIN_MFA_STATUS_SQL)
            .bind(user_id.0)
            .fetch_one(pool.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    Ok((
        crate::StdAdminBool::from(enabled),
        server_admin_contract::AdminOperationalCount::from(
            u64::try_from(remaining)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
    ))
}
pub(crate) async fn upsert_pending(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    encrypted: &crate::StdAdminMfaEncryptedBytes,
    nonce: &crate::StdAdminMfaNonceBytes,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(str_constants::SERVER_ADMIN_UPSERT_MFA_SQL)
        .bind(user_id.0)
        .bind(encrypted.as_ref())
        .bind(nonce.as_ref())
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}
pub(crate) async fn read_secret(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<
    Option<(
        crate::StdAdminMfaEncryptedBytes,
        crate::StdAdminMfaNonceBytes,
        crate::StdAdminBool,
    )>,
    crate::SqlxAdminError,
> {
    sqlx::query_as::<_, (Vec<u8>, Vec<u8>, bool)>(str_constants::SERVER_ADMIN_READ_MFA_SQL)
        .bind(user_id.0)
        .fetch_optional(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| {
            value.map(|(encrypted, nonce, enabled)| {
                (
                    crate::StdAdminMfaEncryptedBytes::from(encrypted),
                    crate::StdAdminMfaNonceBytes::from(nonce),
                    crate::StdAdminBool::from(enabled),
                )
            })
        })
}
pub(crate) async fn enable(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    counter: crate::StdAdminMfaTotpCounter,
    hashes: &crate::StdAdminMfaRecoveryHashes,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    let enabled = sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_ENABLE_MFA_SQL)
        .bind(user_id.0)
        .bind(counter.0)
        .fetch_optional(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .is_some();
    if !enabled {
        return Ok(crate::StdAdminBool::from(false));
    }
    let _deleted = sqlx::query(str_constants::SERVER_ADMIN_DELETE_MFA_RECOVERY_SQL)
        .bind(user_id.0)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let _inserted = sqlx::query(str_constants::SERVER_ADMIN_INSERT_MFA_RECOVERY_SQL)
        .bind(user_id.0)
        .bind(
            hashes
                .as_ref()
                .iter()
                .map(AsRef::<String>::as_ref)
                .collect::<Vec<_>>(),
        )
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    Ok(crate::StdAdminBool::from(true))
}
pub(crate) async fn claim_totp(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    counter: crate::StdAdminMfaTotpCounter,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_CLAIM_MFA_TOTP_SQL)
        .bind(user_id.0)
        .bind(counter.0)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}
pub(crate) async fn consume_recovery(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    hash: &crate::StdAdminString,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_USE_MFA_RECOVERY_SQL)
        .bind(user_id.0)
        .bind(hash.as_ref())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}
pub(crate) async fn disable(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    let deleted = sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_DELETE_MFA_SQL)
        .bind(user_id.0)
        .fetch_optional(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .is_some();
    if deleted {
        let _deleted_codes = sqlx::query(str_constants::SERVER_ADMIN_DELETE_MFA_RECOVERY_SQL)
            .bind(user_id.0)
            .execute(connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    }
    Ok(crate::StdAdminBool::from(deleted))
}
pub(crate) async fn mark_step_up(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_MARK_MFA_STEP_UP_SQL)
        .bind(session_id.0.0)
        .bind(user_id.0)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}
pub(crate) async fn has_recent_step_up(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_HAS_RECENT_MFA_STEP_UP_SQL)
        .bind(user_id.0)
        .bind(session_id.0.0)
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(crate::StdAdminBool::from)
}
