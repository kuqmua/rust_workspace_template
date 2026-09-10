pub(crate) async fn dispatch_filtered_update<Request, Error>(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<Request>,
) -> Result<crate::axum_admin_response::AxumAdminResponse, Error>
where
    Request: crate::admin_filtered_update_operation::AdminFilteredUpdateOperation + Send,
    Error: From<crate::admin_error::AdminError>,
{
    crate::admin_filtered_update_operation::AdminFilteredUpdateOperation::apply(
        axum_admin_json.into_inner(),
        admin_auth_request,
    )
    .await
    .map_err(Error::from)
}
