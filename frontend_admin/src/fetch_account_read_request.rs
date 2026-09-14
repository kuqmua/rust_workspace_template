#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_account_read_request<Request, Response>(
    request: &Request,
    admin_route: server_admin_contract::admin_route::AdminRoute,
) -> Result<Response, crate::admin_table_load_error::AdminTableLoadError>
where
    Request: serde::Serialize,
    Response: serde::de::DeserializeOwned,
{
    let url = crate::admin_api_url::admin_api_url(admin_route)?;
    let body = serde_json::to_vec(request)
        .map_err(crate::std_rc_serde_json_error::StdRcSerdeJsonError::from)?;
    let transport_request = frontend_contract::transport_request::TransportRequest::new(
        frontend_contract::transport_body::TransportBody::try_from(body)?,
        frontend_contract::transport_path::TransportPath::try_from(url.as_ref().to_owned())
            .map_err(crate::admin_table_load_error::AdminTableLoadError::ReadPath)?,
        admin_route.contract(),
    );
    crate::fetch_json_request::fetch_json_request(&url, Some(&transport_request)).await
}
