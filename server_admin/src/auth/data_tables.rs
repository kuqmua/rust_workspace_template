#![allow(clippy::single_call_fn)] // route inventory registers focused data-table operations once

pub(super) async fn list(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    data_table_catalog(auth).await.map(|catalog| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            catalog,
        )))
    })
}
pub(super) async fn data_table_catalog(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AdminDataTableCatalog, super::AdminError> {
    let actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::TablesRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let admin = super::authenticated_admin_contract(&actor)?;
    let items = server_admin_contract::AdminDataTable::ALL
        .into_iter()
        .filter(|table| bool::from(admin.has_permission(table.permission())))
        .collect::<Vec<_>>();
    Ok(server_admin_contract::AdminDataTableCatalog::new(
        server_admin_contract::AdminDataTables::try_from(items)
            .map_err(|_error| super::AdminError::Validation)?,
    ))
}
pub(super) async fn data_table_view(
    auth: super::AdminAuthReq,
    table: server_admin_contract::AdminDataTable,
    query: &server_admin_contract::AdminDataTableQuery,
) -> Result<server_admin_contract::AdminDataTableView, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        table.permission().as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::super::repository::data_tables::read(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        table,
        query,
    )
    .await
    .map_err(super::shared::map_repository_error)
}
pub(super) async fn get(
    auth: super::AdminAuthReq,
    super::AxumAdminPath(table): super::AxumAdminPath<server_admin_contract::AdminDataTable>,
    super::AxumAdminQuery(query): super::AxumAdminQuery<server_admin_contract::AdminDataTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    data_table_view(auth, table, &query).await.map(|view| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            view,
        )))
    })
}
