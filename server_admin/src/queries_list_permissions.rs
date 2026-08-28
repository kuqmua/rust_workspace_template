#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn queries_list_permissions(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let _actor =
        crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
            auth.state.as_ref(),
            crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
            auth.peer,
            crate::AdminPermission::PermissionsRead.as_str(),
            crate::StdAdminBool::from(false),
        )
        .await?;
    crate::shared::validate_table_sort::validate_table_sort(
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
            .map_err(crate::AdminError::from)?;
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
            .map_err(crate::AdminError::from)?
            .into_iter()
            .map(|(id, name)| {
                Ok(
                    server_admin_contract::domain_types::AdminPermissionSummary::new(
                        server_admin_contract::domain_types::AdminPermissionId::try_from(id)
                            .map_err(|_error| crate::AdminError::Validation)?,
                        server_admin_contract::domain_types::AdminPermissionValue::try_from(name)
                            .map_err(|_error| crate::AdminError::Validation)?,
                    ),
                )
            })
            .collect::<Result<Vec<_>, crate::AdminError>>()?;
    let permissions =
        server_admin_contract::domain_types::AdminPermissionSummaries::try_from(items)
            .map_err(|_error| crate::AdminError::Validation)?;
    Ok(crate::shared::json_response::json_response(
        server_admin_contract::domain_types::AdminPermissionsPage::new(
            permissions,
            crate::shared::page_total::page_total(crate::repository::AdminPageTotalCount::from(
                total,
            ))?,
        ),
    ))
}
