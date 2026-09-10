pub(crate) async fn replace_user_roles_in_connection(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    admin_role_ids: &server_admin_contract::admin_role_ids::AdminRoleIds,
    optional_admin_role_ids: Option<&server_admin_contract::admin_role_ids::AdminRoleIds>,
) -> Result<(), crate::admin_error::AdminError> {
    if std::iter::once(admin_role_ids)
        .chain(optional_admin_role_ids)
        .any(|identifiers| {
            let values =
                AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(identifiers);
            values
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
                != values.len()
        })
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    if let Some(expected) = optional_admin_role_ids {
        let current_role_ids =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_USER_ROLE_IDS_SQL)
                .bind(admin_user_record_id.get())
                .fetch_all(&mut **sqlx_admin_repository_connection_mut_ref)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let expected_role_ids =
            AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(expected)
                .iter()
                .copied()
                .map(i64::from)
                .collect::<std::collections::BTreeSet<_>>();
        if current_role_ids.into_iter().ne(expected_role_ids) {
            return Err(crate::admin_error::AdminError::Conflict);
        }
    }
    let raw_ids =
        AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(admin_role_ids)
            .iter()
            .copied()
            .map(i64::from)
            .collect::<Vec<_>>();
    let existing_count = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_ROLES_SQL)
        .bind(&raw_ids)
        .fetch_one(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let _delete_result = sqlx::query(constants_str::SERVER_ADMIN_REPLACE_USER_ROLES_DELETE_SQL)
        .bind(admin_user_record_id.get())
        .execute(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let _insert_result = sqlx::query(constants_str::SERVER_ADMIN_REPLACE_USER_ROLES_INSERT_SQL)
        .bind(admin_user_record_id.get())
        .bind(&raw_ids)
        .execute(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    Ok(())
}
