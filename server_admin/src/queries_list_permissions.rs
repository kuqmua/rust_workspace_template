#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn queries_list_permissions(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let _actor =
        crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
            auth.state.as_ref(),
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
            auth.peer,
            server_admin_contract::admin_permission::AdminPermission::PermissionsRead.as_str(),
            server_admin_core::std_admin_bool::StdAdminBool::from(false),
        )
        .await?;
    crate::validate_table_sort::validate_table_sort(
        &query.0,
        &server_admin_contract::admin_table_sort_field::AdminTableSortField::PERMISSION,
    )?;
    let permission_pool = auth.state.as_ref().pool.as_ref();
    let search = query.0.search().as_ref();
    let total = sqlx::query_scalar::<_, i64>(
        constants_str::integration_fixtures::SERVER_ADMIN_COUNT_FILTERED_PERMISSIONS_SQL,
    )
    .bind(search)
    .fetch_one(permission_pool)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_error::AdminError::from)?;
    let items = sqlx::query_as::<_, (i64, String)>(
        constants_str::integration_fixtures::SERVER_ADMIN_PAGE_PERMISSIONS_SQL,
    )
    .bind(search)
    .bind(query.0.sort().as_ref())
    .bind(query.0.direction().as_ref())
    .bind(i64::from(u16::from(query.0.limit())))
    .bind(i64::from(u32::from(query.0.offset())))
    .fetch_all(permission_pool)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_error::AdminError::from)?
    .into_iter()
    .map(|(id, name)| {
        Ok(
            server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                server_admin_contract::admin_permission_id::AdminPermissionId::try_from(id)
                    .map_err(|_error| crate::admin_error::AdminError::Validation)?,
                server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(name)
                    .map_err(|_error| crate::admin_error::AdminError::Validation)?,
            ),
        )
    })
    .collect::<Result<Vec<_>, crate::admin_error::AdminError>>()?;
    let permissions =
        server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
            items,
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    Ok(crate::json_response::json_response(
        server_admin_contract::admin_permissions_page::AdminPermissionsPage::new(
            permissions,
            crate::page_total::page_total(
                crate::admin_page_total_count::AdminPageTotalCount::from(total),
            )?,
        ),
    ))
}
