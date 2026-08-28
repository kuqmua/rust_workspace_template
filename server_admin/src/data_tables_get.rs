#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn data_tables_get(
    auth: crate::AdminAuthReq,
    crate::AxumAdminPath(table): crate::AxumAdminPath<
        server_admin_contract::domain_types::AdminDataTable,
    >,
    crate::AxumAdminQuery(query): crate::AxumAdminQuery<
        server_admin_contract::domain_types::AdminDataTableQuery,
    >,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        table.permission().as_str(),
        crate::StdAdminBool::from(false),
    )
    .await?;
    crate::repository::data_tables::read(
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref()),
        table,
        &query,
    )
    .await
    .map_err(crate::shared::map_repository_error::map_repository_error)
    .map(crate::shared::json_response::json_response)
}
