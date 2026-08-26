pub(in crate::domain_types::auth) async fn users_page(
    auth: super::super::AdminAuthReq,
    query: super::super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<server_admin_contract::domain_types::AdminUsersPage, super::super::AdminError> {
    let _actor =
        super::super::authorization_authorize_generated_request::authorize_generated_request(
            auth.state.as_ref(),
            super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
            auth.peer,
            super::super::super::AdminPermission::UsersRead.as_str(),
            super::super::super::StdAdminBool::from(false),
        )
        .await?;
    super::super::shared::validate_table_sort::validate_table_sort(
        &query.0,
        &server_admin_contract::domain_types::AdminTableSortField::USER,
    )?;
    let user_pool = auth.state.as_ref().pool.as_ref();
    let (users, total) = async {
        let search = query.0.search().as_ref();
        let total =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_USERS_SQL)
                .bind(search)
                .fetch_one(user_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let rows = sqlx::query_as::<_, (i64, String, String, bool)>(
            constants_str::SERVER_ADMIN_PAGE_USERS_SQL,
        )
        .bind(search)
        .bind(query.0.sort().as_ref())
        .bind(query.0.direction().as_ref())
        .bind(i64::from(u16::from(query.0.limit())))
        .bind(i64::from(u32::from(query.0.offset())))
        .fetch_all(user_pool)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
        let user_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links =
            sqlx::query_as::<_, (i64, i64)>(constants_str::SERVER_ADMIN_LIST_USER_ROLE_IDS_SQL)
                .bind(user_ids.as_slice())
                .fetch_all(user_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let mut role_ids_by_user =
            links.into_iter().try_fold(
                std::collections::HashMap::<
                    i64,
                    Vec<server_admin_contract::domain_types::AdminRoleId>,
                >::with_capacity(user_ids.len()),
                |mut values, (user_id, role_id)| {
                    values.entry(user_id).or_default().push(
                    server_admin_contract::domain_types::AdminRoleId::try_from(role_id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                );
                    Ok::<_, crate::adapters::repository::AdminRepositoryError>(values)
                },
            )?;
        let items = rows
            .into_iter()
            .map(|(id, login, display_name, is_banned)| {
                Ok(server_admin_contract::domain_types::AdminUserSummary::new(
                    server_admin_contract::domain_types::AdminDisplayName::try_from(display_name)
                        .map_err(|_error| {
                        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                    })?,
                    server_admin_contract::domain_types::AdminUserId::try_from(id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminBool::from(is_banned),
                    server_admin_contract::domain_types::AdminLogin::try_from(login).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminRoleIds::try_from(
                        role_ids_by_user.remove(&id).unwrap_or_default(),
                    )
                    .map_err(|_error| {
                        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, crate::adapters::repository::AdminRepositoryError>>()?;
        Ok::<_, crate::adapters::repository::AdminRepositoryError>((
            server_admin_contract::domain_types::AdminUserSummaries::try_from(items).map_err(
                |_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue,
            )?,
            crate::adapters::repository::AdminPageTotalCount::from(total),
        ))
    }
    .await
    .map_err(super::super::shared::map_repository_error::map_repository_error)?;
    let roles = async {
        let role_catalog_pool = auth.state.as_ref().pool.as_ref();
        let rows =
            sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_LIST_ROLES_SQL)
                .fetch_all(role_catalog_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links = sqlx::query_as::<_, (i64, i64)>(
            constants_str::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL,
        )
        .bind(role_ids.as_slice())
        .fetch_all(role_catalog_pool)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
        let mut permission_ids_by_role = links.into_iter().try_fold(
            std::collections::HashMap::<
                i64,
                Vec<server_admin_contract::domain_types::AdminPermissionId>,
            >::with_capacity(role_ids.len()),
            |mut values, (role_id, permission_id)| {
                values.entry(role_id).or_default().push(
                    server_admin_contract::domain_types::AdminPermissionId::try_from(permission_id)
                        .map_err(|_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        })?,
                );
                Ok::<_, crate::adapters::repository::AdminRepositoryError>(values)
            },
        )?;
        let values = rows
            .into_iter()
            .map(|(id, name, is_system)| {
                Ok(server_admin_contract::domain_types::AdminRoleSummary::new(
                    server_admin_contract::domain_types::AdminRoleId::try_from(id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminBool::from(is_system),
                    server_admin_contract::domain_types::AdminRoleName::try_from(name).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminPermissionIds::try_from(
                        permission_ids_by_role.remove(&id).unwrap_or_default(),
                    )
                    .map_err(|_error| {
                        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, crate::adapters::repository::AdminRepositoryError>>()?;
        server_admin_contract::domain_types::AdminRoleSummaries::try_from(values)
            .map_err(|_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue)
    }
    .await
    .map_err(super::super::shared::map_repository_error::map_repository_error)?;
    Ok(server_admin_contract::domain_types::AdminUsersPage::new(
        users,
        roles,
        super::super::shared::page_total::page_total(total)?,
    ))
}
