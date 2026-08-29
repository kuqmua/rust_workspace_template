#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn data_tables_list(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        server_admin_contract::admin_permission::AdminPermission::TablesRead.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    )
    .await?;
    let admin = crate::authenticated_admin_contract::authenticated_admin_contract(&actor)?;
    let items = server_admin_contract::admin_data_table::AdminDataTable::ALL
        .into_iter()
        .filter(|table| bool::from(admin.has_permission(table.permission())))
        .collect::<Vec<_>>();
    Ok(crate::json_response::json_response(
        server_admin_contract::admin_data_table_catalog::AdminDataTableCatalog::new(
            server_admin_contract::admin_data_tables::AdminDataTables::try_from(items)
                .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        ),
    ))
}
