#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_audit_log_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let query = crate::admin_table_query::admin_table_query(admin_csr_query)?;
    let filter_query =
        crate::admin_identifier_filter_query::admin_identifier_filter_query(admin_csr_query)?;
    let input_kind = match filter_query.field() {
        None => frontend_contract::input_kind::InputKind::Number,
        Some(field)
            if matches!(
                field.as_ref(),
                constants_str::SQL_NAMES_ID | constants_str::USER_ID
            ) =>
        {
            frontend_contract::input_kind::InputKind::Number
        }
        Some(field) if field.as_ref() == constants_str::SUCCEEDED => {
            frontend_contract::input_kind::InputKind::Checkbox
        }
        Some(field) if field.as_ref() == constants_str::CREATED_AT => {
            frontend_contract::input_kind::InputKind::DateTime
        }
        Some(field) if field.as_ref() == constants_str::TEST_JSON_REQUEST_ID => {
            frontend_contract::input_kind::InputKind::Uuid
        }
        Some(field)
            if matches!(
                field.as_ref(),
                constants_str::USER_LOGIN
                    | constants_str::ACTION
                    | constants_str::RESOURCE
                    | constants_str::RESOURCE_ID
            ) =>
        {
            frontend_contract::input_kind::InputKind::Text
        }
        Some(_) => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    let where_many = server_admin_contract::admin_where_many::AdminWhereMany::try_from_filter(
        &filter_query,
        input_kind,
    )
    .map_err(|error| {
        match server_admin_contract::admin_text::AdminText::try_from(error.to_string()) {
            Ok(admin_text) => {
                crate::admin_table_load_error::AdminTableLoadError::QuerySource(admin_text)
            }
            Err(_error) => crate::admin_table_load_error::AdminTableLoadError::Query,
        }
    })?;
    let base_request =
        server_admin_contract::admin_audit_log_read_request::AdminAuditLogReadRequest::try_from(
            &query,
        )?;
    let request =
        server_admin_contract::admin_audit_log_read_request::AdminAuditLogReadRequest::new(
            base_request.get_search().cloned(),
            base_request.get_select().to_owned(),
            *base_request.get_pagination(),
            *base_request.get_order_by(),
            where_many,
        );
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::AuditLog,
    )
    .await
}
