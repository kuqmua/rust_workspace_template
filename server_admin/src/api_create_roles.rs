#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_create_roles(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_roles_request::AdminCreateRolesRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminCreateRolesError,
> {
    crate::role_mutations_create_many::role_mutations_create_many(
        admin_auth_request,
        axum_admin_json.into_inner(),
    )
    .await
    .map(|identifiers| {
        crate::axum_admin_response::AxumAdminResponse::from(
            axum::response::IntoResponse::into_response((
                http::StatusCode::CREATED,
                axum::Json(identifiers),
            )),
        )
    })
    .map_err(crate::application_auth::AdminCreateRolesError::from)
}
