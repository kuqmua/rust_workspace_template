#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_account_read<Request, Response>(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
    admin_route: server_admin_contract::admin_route::AdminRoute,
) -> Result<
    Response,
    crate::admin_table_load_error::AdminTableLoadError,
>
where
    Request: serde::Serialize + for<'query> TryFrom<&'query server_admin_contract::admin_table_query::AdminTableQuery, Error = server_admin_contract::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError>,
    Response: serde::de::DeserializeOwned,
{
    let direction = match admin_csr_query.direction() {
        None => server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending,
        Some(direction)
            if direction.as_ref().as_str()
                == server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending
                    .as_ref() =>
        {
            server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending
        }
        Some(direction)
            if direction.as_ref().as_str()
                == server_admin_contract::admin_sort_direction::AdminSortDirection::Descending
                    .as_ref() =>
        {
            server_admin_contract::admin_sort_direction::AdminSortDirection::Descending
        }
        Some(_) => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    let query = server_admin_contract::admin_table_query::AdminTableQuery::new(
        admin_csr_query.search().clone(),
        admin_csr_query.sort().clone(),
        admin_csr_query.offset(),
        admin_csr_query.limit(),
        direction,
    );
    let request = Request::try_from(&query)?;
    let url = crate::admin_api_url::admin_api_url(admin_route)?;
    let body = serde_json::to_vec(&request)
        .map_err(crate::std_rc_serde_json_error::StdRcSerdeJsonError::from)?;
    let transport_request = frontend_contract::transport_request::TransportRequest::new(
        frontend_contract::transport_body::TransportBody::try_from(body)?,
        frontend_contract::transport_path::TransportPath::try_from(url.as_ref().to_owned())
            .map_err(crate::admin_table_load_error::AdminTableLoadError::ReadPath)?,
        admin_route.contract(),
    );
    crate::fetch_json_request::fetch_json_request(&url, Some(&transport_request)).await
}
