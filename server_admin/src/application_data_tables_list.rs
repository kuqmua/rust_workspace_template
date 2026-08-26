#![allow(clippy::single_call_fn)] // route inventory registers focused data-table operations once

pub(super) async fn data_tables_list(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::TablesRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let admin = super::authenticated_admin_contract(&actor)?;
    let items = server_admin_contract::domain_types::AdminDataTable::ALL
        .into_iter()
        .filter(|table| bool::from(admin.has_permission(table.permission())))
        .collect::<Vec<_>>();
    Ok(super::shared::json_response::json_response(
        server_admin_contract::domain_types::AdminDataTableCatalog::new(
            server_admin_contract::domain_types::AdminDataTables::try_from(items)
                .map_err(|_error| super::AdminError::Validation)?,
        ),
    ))
}
