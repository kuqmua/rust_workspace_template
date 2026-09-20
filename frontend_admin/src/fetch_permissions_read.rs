#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_permissions_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_permissions_page::AdminPermissionsPage,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let query = if admin_csr_query.permission_id().is_some() {
        server_admin_contract::admin_table_query::AdminTableQuery::default()
    } else {
        crate::admin_table_query::admin_table_query(admin_csr_query)?
    };
    let filter_query = match admin_csr_query.permission_id() {
        Some(permission_id) => {
            server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
                Some(
                    server_admin_contract::admin_filter_field::AdminFilterField::try_from(
                        constants_str::SQL_NAMES_ID.to_owned(),
                    )
                    .map_err(crate::admin_table_load_error::AdminTableLoadError::ReadFilterField)?,
                ),
                Some(frontend_contract::filter_operation::FilterOperation::Eq),
                Some(
                    server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                        permission_id.to_string(),
                    )
                    .map_err(crate::admin_table_load_error::AdminTableLoadError::ReadFilterValue)?,
                ),
                None,
            )
        }
        None => {
            crate::admin_identifier_filter_query::admin_identifier_filter_query(admin_csr_query)?
        }
    };
    let input_kind = match filter_query.field() {
        None => frontend_contract::input_kind::InputKind::Number,
        Some(field) if field.as_ref() == constants_str::SQL_NAMES_ID => {
            frontend_contract::input_kind::InputKind::Number
        }
        Some(field) if field.as_ref() == constants_str::NAME => {
            frontend_contract::input_kind::InputKind::Text
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
    let base_request = server_admin_contract::admin_permissions_read_request::AdminPermissionsReadRequest::try_from(&query)?;
    let request =
        server_admin_contract::admin_permissions_read_request::AdminPermissionsReadRequest::new(
            base_request.get_search().cloned(),
            base_request.get_pagination().clone(),
            base_request.get_select().clone(),
            base_request.get_order_by().clone(),
            where_many,
        );
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::Permissions,
    )
    .await
}
