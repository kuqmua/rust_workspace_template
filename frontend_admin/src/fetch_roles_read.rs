#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_roles_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_roles_page::AdminRolesPage,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let query = crate::admin_table_query::admin_table_query(admin_csr_query)?;
    let filter_query =
        crate::admin_identifier_filter_query::admin_identifier_filter_query(admin_csr_query)?;
    let input_kind = match filter_query.field() {
        None => frontend_contract::input_kind::InputKind::Number,
        Some(field) if field.as_ref() == constants_str::SQL_NAMES_ID => {
            frontend_contract::input_kind::InputKind::Number
        }
        Some(field) if field.as_ref() == constants_str::NAME => {
            frontend_contract::input_kind::InputKind::Text
        }
        Some(field) if field.as_ref() == constants_str::IS_SYSTEM => {
            frontend_contract::input_kind::InputKind::Checkbox
        }
        Some(_) => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    let where_many = match server_admin_contract::admin_where_many::AdminWhereMany::try_from_filter(
        &filter_query,
        input_kind,
    ) {
        Ok(where_many) => where_many,
        Err(_error) => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    let base_request =
        server_admin_contract::admin_roles_read_request::AdminRolesReadRequest::try_from(&query)?;
    let request = server_admin_contract::admin_roles_read_request::AdminRolesReadRequest::new(
        base_request.get_permissions_query().cloned(),
        base_request.get_search().cloned(),
        base_request.get_pagination().clone(),
        base_request.get_select().clone(),
        base_request.get_order_by().clone(),
        where_many,
    );
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::Roles,
    )
    .await
}
