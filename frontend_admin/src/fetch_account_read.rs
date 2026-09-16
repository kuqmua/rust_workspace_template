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
    let query = crate::admin_table_query::admin_table_query(admin_csr_query)?;
    crate::fetch_account_read_request::fetch_account_read_request(
        &Request::try_from(&query)?,
        admin_route,
    )
    .await
}
