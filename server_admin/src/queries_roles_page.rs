pub(crate) async fn queries_roles_page(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<server_admin_contract::admin_roles_page::AdminRolesPage, crate::admin_error::AdminError>
{
    let _actor =
        crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
            auth.state.as_ref(),
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
            auth.peer,
            server_admin_contract::admin_permission::AdminPermission::RolesRead.as_str(),
            server_admin_core::std_admin_bool::StdAdminBool::from(false),
        )
        .await?;
    crate::validate_table_sort::validate_table_sort(
        &query.0,
        &server_admin_contract::admin_table_sort_field::AdminTableSortField::ROLE,
    )?;
    let role_pool = auth.state.as_ref().pool.as_ref();
    let (roles, total) = async {
        let search = query.0.search().as_ref();
        let total = sqlx::query_scalar::<_, i64>(
            constants_str::integration_fixtures::SERVER_ADMIN_COUNT_FILTERED_ROLES_SQL,
        )
        .bind(search)
        .fetch_one(role_pool)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let rows = sqlx::query_as::<_, (i64, String, bool)>(
            constants_str::integration_fixtures::SERVER_ADMIN_PAGE_ROLES_SQL,
        )
        .bind(search)
        .bind(query.0.sort().as_ref())
        .bind(query.0.direction().as_ref())
        .bind(i64::from(u16::from(query.0.limit())))
        .bind(i64::from(u32::from(query.0.offset())))
        .fetch_all(role_pool)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links = sqlx::query_as::<_, (i64, i64)>(
            constants_str::integration_fixtures::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL,
        )
        .bind(role_ids.as_slice())
        .fetch_all(role_pool)
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
        let items = rows
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
        Ok::<_, crate::admin_repository_error::AdminRepositoryError>((
            server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(items)
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
            crate::admin_page_total_count::AdminPageTotalCount::from(total),
        ))
    }
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    let permissions = async {
        let permission_pool = auth.state.as_ref().pool.as_ref();
        let values = sqlx::query_as::<_, (i64, String)>(
            constants_str::integration_fixtures::SERVER_ADMIN_LIST_PERMISSIONS_SQL,
        )
        .fetch_all(permission_pool)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?
        .into_iter()
        .map(|(id, name)| {
            Ok(
                server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                    server_admin_contract::admin_permission_id::AdminPermissionId::try_from(id)
                        .map_err(|_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        })?,
                    server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                        name,
                    )
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                ),
            )
        })
        .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
        server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
            values,
        )
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
    }
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    Ok(
        server_admin_contract::admin_roles_page::AdminRolesPage::new(
            roles,
            permissions,
            crate::page_total::page_total(total)?,
        ),
    )
}
