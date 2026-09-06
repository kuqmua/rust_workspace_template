#[allow(
    clippy::future_not_send,
    reason = "browser session recovery executes on the browser thread"
)]
pub(crate) async fn with_admin_session<Output, Operation>(
    mut operation: Operation,
) -> Result<Output, crate::admin_table_load_error::AdminTableLoadError>
where
    Operation: AsyncFnMut() -> Result<Output, crate::admin_table_load_error::AdminTableLoadError>,
{
    match operation().await {
        Ok(output) => return Ok(output),
        Err(error) if !bool::from(error.requires_session_refresh()) => return Err(error),
        Err(_authentication_failure) => {}
    }
    let refresh_url = crate::admin_api_url::admin_api_url(
        server_admin_contract::admin_route::AdminRoute::Refresh,
    )?;
    let refreshed = crate::send_admin_request::send_admin_request(
        crate::admin_mutation_method::AdminMutationMethod::Post,
        &refresh_url,
        &server_admin_contract::admin_no_body::AdminNoBody,
    )
    .await;
    match operation().await {
        Ok(output) => Ok(output),
        Err(error) => match refreshed {
            Ok(()) => Err(error),
            Err(refresh_error) => Err(refresh_error),
        },
    }
}
