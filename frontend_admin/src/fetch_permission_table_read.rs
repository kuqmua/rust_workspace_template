#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_permission_table_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
    admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let is_detail = match admin_data_table {
        server_admin_contract::admin_data_table::AdminDataTable::PermissionActions => {
            admin_csr_query.permission_action_id().is_some()
        }
        server_admin_contract::admin_data_table::AdminDataTable::PermissionResourceActions => {
            admin_csr_query.permission_resource_action_id().is_some()
        }
        server_admin_contract::admin_data_table::AdminDataTable::PermissionResources => {
            admin_csr_query.permission_resource_id().is_some()
        }
        _ => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    let query = if is_detail {
        server_admin_contract::admin_table_query::AdminTableQuery::default()
    } else {
        crate::admin_table_query::admin_table_query(admin_csr_query)?
    };
    let where_many = if is_detail {
        let filter_query =
            crate::admin_identifier_filter_query::admin_identifier_filter_query(admin_csr_query)?;
        server_admin_contract::admin_where_many::AdminWhereMany::try_from_identifier_filter(
            &filter_query,
        )
        .map_err(|error| {
            match server_admin_contract::admin_text::AdminText::try_from(error.to_string()) {
                Ok(admin_text) => {
                    crate::admin_table_load_error::AdminTableLoadError::QuerySource(admin_text)
                }
                Err(text_error) => {
                    crate::admin_table_load_error::AdminTableLoadError::ReadText(text_error)
                }
            }
        })?
    } else {
        None
    };
    match admin_data_table {
        server_admin_contract::admin_data_table::AdminDataTable::PermissionActions => {
            let base_request = server_admin_contract::admin_permission_actions_read_request::AdminPermissionActionsReadRequest::try_from(&query)?;
            let request = server_admin_contract::admin_permission_actions_read_request::AdminPermissionActionsReadRequest::new(
                *base_request.get_pagination(),
                *base_request.get_select(),
                *base_request.get_order_by(),
                where_many,
            );
            crate::fetch_account_read_request::fetch_account_read_request(
                &request,
                admin_data_table.api_route(),
            )
            .await
        }
        server_admin_contract::admin_data_table::AdminDataTable::PermissionResourceActions => {
            let base_request = server_admin_contract::admin_permission_resource_actions_read_request::AdminPermissionResourceActionsReadRequest::try_from(&query)?;
            let request = server_admin_contract::admin_permission_resource_actions_read_request::AdminPermissionResourceActionsReadRequest::new(
                *base_request.get_pagination(),
                *base_request.get_select(),
                *base_request.get_order_by(),
                where_many,
            );
            crate::fetch_account_read_request::fetch_account_read_request(
                &request,
                admin_data_table.api_route(),
            )
            .await
        }
        server_admin_contract::admin_data_table::AdminDataTable::PermissionResources => {
            let base_request = server_admin_contract::admin_permission_resources_read_request::AdminPermissionResourcesReadRequest::try_from(&query)?;
            let request = server_admin_contract::admin_permission_resources_read_request::AdminPermissionResourcesReadRequest::new(
                *base_request.get_pagination(),
                *base_request.get_select(),
                *base_request.get_order_by(),
                where_many,
            );
            crate::fetch_account_read_request::fetch_account_read_request(
                &request,
                admin_data_table.api_route(),
            )
            .await
        }
        _ => Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    }
}
