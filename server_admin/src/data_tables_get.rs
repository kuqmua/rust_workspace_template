#![allow(clippy::single_call_fn)] // route inventory registers the data-table read operation once

pub(super) async fn data_tables_get(
    auth: super::AdminAuthReq,
    super::AxumAdminPath(table): super::AxumAdminPath<
        server_admin_contract::domain_types::AdminDataTable,
    >,
    super::AxumAdminQuery(query): super::AxumAdminQuery<
        server_admin_contract::domain_types::AdminDataTableQuery,
    >,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let _actor = super::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        table.permission().as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    crate::repository::data_tables::read(
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref()),
        table,
        &query,
    )
    .await
    .map_err(super::shared::map_repository_error::map_repository_error)
    .map(super::shared::json_response::json_response)
}
