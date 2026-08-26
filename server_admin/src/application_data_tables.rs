#![allow(clippy::single_call_fn)] // route inventory registers focused data-table operations once

pub(super) async fn list(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorization::authorize_generated_request(
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
    Ok(super::shared::json_response(
        server_admin_contract::domain_types::AdminDataTableCatalog::new(
            server_admin_contract::domain_types::AdminDataTables::try_from(items)
                .map_err(|_error| super::AdminError::Validation)?,
        ),
    ))
}
pub(super) async fn get(
    auth: super::AdminAuthReq,
    super::AxumAdminPath(table): super::AxumAdminPath<
        server_admin_contract::domain_types::AdminDataTable,
    >,
    super::AxumAdminQuery(query): super::AxumAdminQuery<
        server_admin_contract::domain_types::AdminDataTableQuery,
    >,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let _actor = super::authorization::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        table.permission().as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    crate::adapters::repository::data_tables::read(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        table,
        &query,
    )
    .await
    .map_err(super::shared::map_repository_error)
    .map(super::shared::json_response)
}
