pub(in crate::domain_types::auth) async fn roles_page(
    auth: super::super::AdminAuthReq,
    query: super::super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<server_admin_contract::domain_types::AdminRolesPage, super::super::AdminError> {
    let _actor = super::super::authorization::authorize_generated_request(
        auth.state.as_ref(),
        super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::super::AdminPermission::RolesRead.as_str(),
        super::super::super::StdAdminBool::from(false),
    )
    .await?;
    super::super::shared::validate_table_sort(
        &query.0,
        &server_admin_contract::domain_types::AdminTableSortField::ROLE,
    )?;
    let role_pool = auth.state.as_ref().pool.as_ref();
    let (roles, total) = async {
        let search = query.0.search().as_ref();
        let total =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_ROLES_SQL)
                .bind(search)
                .fetch_one(role_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let rows =
            sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_PAGE_ROLES_SQL)
                .bind(search)
                .bind(query.0.sort().as_ref())
                .bind(query.0.direction().as_ref())
                .bind(i64::from(u16::from(query.0.limit())))
                .bind(i64::from(u32::from(query.0.offset())))
                .fetch_all(role_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links = sqlx::query_as::<_, (i64, i64)>(
            constants_str::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL,
        )
        .bind(role_ids.as_slice())
        .fetch_all(role_pool)
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
        let items = rows
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
        Ok::<_, crate::adapters::repository::AdminRepositoryError>((
            server_admin_contract::domain_types::AdminRoleSummaries::try_from(items).map_err(
                |_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue,
            )?,
            crate::adapters::repository::AdminPageTotalCount::from(total),
        ))
    }
    .await
    .map_err(super::super::shared::map_repository_error)?;
    let permissions = async {
        let permission_pool = auth.state.as_ref().pool.as_ref();
        let values =
            sqlx::query_as::<_, (i64, String)>(constants_str::SERVER_ADMIN_LIST_PERMISSIONS_SQL)
                .fetch_all(permission_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?
                .into_iter()
                .map(|(id, name)| {
                    Ok(
                server_admin_contract::domain_types::AdminPermissionSummary::new(
                    server_admin_contract::domain_types::AdminPermissionId::try_from(id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminPermissionValue::try_from(name)
                        .map_err(|_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        })?,
                ),
            )
                })
                .collect::<Result<Vec<_>, crate::adapters::repository::AdminRepositoryError>>()?;
        server_admin_contract::domain_types::AdminPermissionSummaries::try_from(values)
            .map_err(|_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue)
    }
    .await
    .map_err(super::super::shared::map_repository_error)?;
    Ok(server_admin_contract::domain_types::AdminRolesPage::new(
        roles,
        permissions,
        super::super::shared::page_total(total)?,
    ))
}
#[allow(clippy::single_call_fn)] // Typed-route delegate keeps transport response mapping out of the query workflow.
pub(in crate::domain_types::auth) async fn list(
    auth: super::super::AdminAuthReq,
    query: super::super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    roles_page(auth, query)
        .await
        .map(super::super::shared::json_response)
}
#[allow(clippy::single_call_fn)] // Typed-route delegate owns the permissions query transport workflow.
pub(in crate::domain_types::auth) async fn list_permissions(
    auth: super::super::AdminAuthReq,
    query: super::super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let _actor = super::super::authorization::authorize_generated_request(
        auth.state.as_ref(),
        super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::super::AdminPermission::PermissionsRead.as_str(),
        super::super::super::StdAdminBool::from(false),
    )
    .await?;
    super::super::shared::validate_table_sort(
        &query.0,
        &server_admin_contract::domain_types::AdminTableSortField::PERMISSION,
    )?;
    let permission_pool = auth.state.as_ref().pool.as_ref();
    let search = query.0.search().as_ref();
    let total =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_PERMISSIONS_SQL)
            .bind(search)
            .fetch_one(permission_pool)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map_err(super::super::AdminError::from)?;
    let items =
        sqlx::query_as::<_, (i64, String)>(constants_str::SERVER_ADMIN_PAGE_PERMISSIONS_SQL)
            .bind(search)
            .bind(query.0.sort().as_ref())
            .bind(query.0.direction().as_ref())
            .bind(i64::from(u16::from(query.0.limit())))
            .bind(i64::from(u32::from(query.0.offset())))
            .fetch_all(permission_pool)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map_err(super::super::AdminError::from)?
            .into_iter()
            .map(|(id, name)| {
                Ok(
                    server_admin_contract::domain_types::AdminPermissionSummary::new(
                        server_admin_contract::domain_types::AdminPermissionId::try_from(id)
                            .map_err(|_error| super::super::AdminError::Validation)?,
                        server_admin_contract::domain_types::AdminPermissionValue::try_from(name)
                            .map_err(|_error| super::super::AdminError::Validation)?,
                    ),
                )
            })
            .collect::<Result<Vec<_>, super::super::AdminError>>()?;
    let permissions =
        server_admin_contract::domain_types::AdminPermissionSummaries::try_from(items)
            .map_err(|_error| super::super::AdminError::Validation)?;
    Ok(super::super::shared::json_response(
        server_admin_contract::domain_types::AdminPermissionsPage::new(
            permissions,
            super::super::shared::page_total(
                crate::adapters::repository::AdminPageTotalCount::from(total),
            )?,
        ),
    ))
}
