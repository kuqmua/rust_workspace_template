#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_users_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let query = crate::admin_table_query::admin_table_query(admin_csr_query)?;
    let filter_query = match admin_csr_query.user_id() {
        Some(user_id) => {
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
                        user_id.to_string(),
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
        Some(field)
            if matches!(
                field.as_ref(),
                constants_str::LOGIN | constants_str::DISPLAY_NAME
            ) =>
        {
            frontend_contract::input_kind::InputKind::Text
        }
        Some(field) if field.as_ref() == constants_str::IS_BANNED => {
            frontend_contract::input_kind::InputKind::Checkbox
        }
        Some(field) if field.as_ref() == stringify!(must_change_password) => {
            frontend_contract::input_kind::InputKind::Checkbox
        }
        Some(field)
            if matches!(
                field.as_ref(),
                constants_str::CREATED_AT | constants_str::UPDATED_AT
            ) =>
        {
            frontend_contract::input_kind::InputKind::DateTime
        }
        Some(_) => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    let where_many = server_admin_contract::admin_where_many::AdminWhereMany::try_from_filter(
        &filter_query,
        input_kind,
    )
    .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?;
    let default_query = server_admin_contract::admin_table_query::AdminTableQuery::default();
    let base_request =
        server_admin_contract::admin_users_read_request::AdminUsersReadRequest::try_from(
            if admin_csr_query.user_id().is_some() {
                &default_query
            } else {
                &query
            },
        )?;
    let request = server_admin_contract::admin_users_read_request::AdminUsersReadRequest::new(
        base_request.get_search().cloned(),
        base_request.get_pagination().clone(),
        base_request.get_select().clone(),
        base_request.get_order_by().clone(),
        where_many,
    );
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::Users,
    )
    .await
}
