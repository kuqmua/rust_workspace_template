pub(crate) async fn data_tables_list(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        crate::AdminPermission::TablesRead.as_str(),
        crate::StdAdminBool::from(false),
    )
    .await?;
    let admin = crate::authenticated_admin_contract(&actor)?;
    let items = server_admin_contract::domain_types::AdminDataTable::ALL
        .into_iter()
        .filter(|table| bool::from(admin.has_permission(table.permission())))
        .collect::<Vec<_>>();
    Ok(crate::shared::json_response::json_response(
        server_admin_contract::domain_types::AdminDataTableCatalog::new(
            server_admin_contract::domain_types::AdminDataTables::try_from(items)
                .map_err(|_error| crate::AdminError::Validation)?,
        ),
    ))
}
