#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_json<Response>(
    admin_csr_api_url: &crate::admin_csr_api_url::AdminCsrApiUrl,
) -> Result<Response, crate::admin_table_load_error::AdminTableLoadError>
where
    Response: serde::de::DeserializeOwned,
{
    crate::fetch_json_request::fetch_json_request(admin_csr_api_url, None).await
}
