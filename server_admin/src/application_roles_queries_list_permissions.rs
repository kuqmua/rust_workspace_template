#[allow(clippy::single_call_fn)] // typed-route delegate owns permissions query transport workflow
pub(in crate::domain_types::auth) async fn queries_list_permissions(
    auth: super::super::AdminAuthReq,
    query: super::super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let _actor =
        super::super::authorization_authorize_generated_request::authorization_authorize_generated_request(
            auth.state.as_ref(),
            super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
            auth.peer,
            super::super::super::AdminPermission::PermissionsRead.as_str(),
            super::super::super::StdAdminBool::from(false),
        )
        .await?;
    super::super::shared::validate_table_sort::validate_table_sort(
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
    Ok(super::super::shared::json_response::json_response(
        server_admin_contract::domain_types::AdminPermissionsPage::new(
            permissions,
            super::super::shared::page_total::page_total(
                crate::adapters::repository::AdminPageTotalCount::from(total),
            )?,
        ),
    ))
}
