#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_user_roles_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let query = if admin_csr_query.user_role_id().is_some() {
        server_admin_contract::admin_table_query::AdminTableQuery::default()
    } else {
        crate::admin_table_query::admin_table_query(admin_csr_query)?
    };
    let filter_query = match admin_csr_query.user_role_id() {
        Some(user_role_id) => {
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
                        user_role_id.to_string(),
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
    let request = server_admin_contract::admin_data_table_query::AdminDataTableQuery::new(
        filter_query,
        query,
    );
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::UserRolesTable,
    )
    .await
}
