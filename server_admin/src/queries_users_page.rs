pub(crate) async fn queries_users_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<server_admin_contract::admin_users_page::AdminUsersPage, crate::admin_error::AdminError>
{
    let _actor =
        crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
            admin_auth_request.get_state().as_ref(),
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(admin_auth_request.get_headers().as_ref()),
            *admin_auth_request.get_peer(),
            server_admin_contract::admin_permission::AdminPermission::UsersRead.as_str(),
            server_admin_core::std_admin_bool::StdAdminBool::from(false),
        )
        .await?;
    crate::validate_table_sort::validate_table_sort(
        axum_admin_query.get_inner(),
        &server_admin_contract::admin_table_sort_field::AdminTableSortField::USER,
    )?;
    let user_pool = admin_auth_request.get_state().as_ref().get_pool().as_ref();
    let (users, total) = async {
        let search = axum_admin_query.get_inner().search().as_ref();
        let total =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_USERS_SQL)
                .bind(search)
                .fetch_one(user_pool)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let rows = sqlx::query_as::<_, (i64, String, String, bool)>(
            constants_str::SERVER_ADMIN_PAGE_USERS_SQL,
        )
        .bind(search)
        .bind(axum_admin_query.get_inner().sort().as_ref())
        .bind(axum_admin_query.get_inner().direction().as_ref())
        .bind(i64::from(u16::from(axum_admin_query.get_inner().limit())))
        .bind(i64::from(u32::from(axum_admin_query.get_inner().offset())))
        .fetch_all(user_pool)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let user_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links =
            sqlx::query_as::<_, (i64, i64)>(constants_str::SERVER_ADMIN_LIST_USER_ROLE_IDS_SQL)
                .bind(user_ids.as_slice())
                .fetch_all(user_pool)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let mut role_ids_by_user =
            links.into_iter().try_fold(
                std::collections::HashMap::<
                    i64,
                    Vec<server_admin_contract::admin_role_id::AdminRoleId>,
                >::with_capacity(user_ids.len()),
                |mut values, (user_id, role_id)| {
                    values.entry(user_id).or_default().push(
                        server_admin_contract::admin_role_id::AdminRoleId::try_from(role_id)
                            .map_err(|_error| {
                                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                            })?,
                    );
                    Ok::<_, crate::admin_repository_error::AdminRepositoryError>(values)
                },
            )?;
        let items = rows
            .into_iter()
            .map(|(id, login, display_name, is_banned)| {
                Ok(server_admin_contract::admin_user_summary::AdminUserSummary::new(
                    server_admin_contract::admin_display_name::AdminDisplayName::try_from(display_name)
                        .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                    server_admin_contract::admin_user_id::AdminUserId::try_from(id).map_err(
                        |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    )?,
                    server_admin_contract::admin_bool::AdminBool::from(is_banned),
                    server_admin_contract::admin_login::AdminLogin::try_from(login).map_err(
                        |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    )?,
                    server_admin_contract::admin_role_ids::AdminRoleIds::try_from(
                        role_ids_by_user.remove(&id).unwrap_or_default(),
                    )
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
        Ok::<_, crate::admin_repository_error::AdminRepositoryError>((
            server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(items)
                .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?,
            crate::admin_page_total_count::AdminPageTotalCount::from(total),
        ))
    }
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    let roles = async {
        let role_catalog_pool = admin_auth_request.get_state().as_ref().get_pool().as_ref();
        let rows =
            sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_LIST_ROLES_SQL)
                .fetch_all(role_catalog_pool)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links = sqlx::query_as::<_, (i64, i64)>(
            constants_str::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL,
        )
        .bind(role_ids.as_slice())
        .fetch_all(role_catalog_pool)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let mut permission_ids_by_role = links.into_iter().try_fold(
            std::collections::HashMap::<
                i64,
                Vec<server_admin_contract::admin_permission_id::AdminPermissionId>,
            >::with_capacity(role_ids.len()),
            |mut values, (role_id, permission_id)| {
                values.entry(role_id).or_default().push(
                    server_admin_contract::admin_permission_id::AdminPermissionId::try_from(
                        permission_id,
                    )
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                );
                Ok::<_, crate::admin_repository_error::AdminRepositoryError>(values)
            },
        )?;
        let values = rows
            .into_iter()
            .map(|(id, name, is_system)| {
                Ok(server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                    server_admin_contract::admin_role_id::AdminRoleId::try_from(id).map_err(
                        |_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::admin_bool::AdminBool::from(is_system),
                    server_admin_contract::admin_role_name::AdminRoleName::try_from(name).map_err(
                        |_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                        permission_ids_by_role.remove(&id).unwrap_or_default(),
                    )
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(values).map_err(
            |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
        )
    }
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    Ok(
        server_admin_contract::admin_users_page::AdminUsersPage::new(
            users,
            roles,
            crate::page_total::page_total(total)?,
        ),
    )
}
