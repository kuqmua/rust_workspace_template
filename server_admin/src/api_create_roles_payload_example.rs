#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_create_roles_payload_example() -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::admin_create_roles_payload_example_error::AdminCreateRolesPayloadExampleError,
> {
    let name = server_admin_contract::admin_role_name::AdminRoleName::try_from(
        constants_str::ADMIN_FIXTURE_ROLE_NAME.to_owned(),
    )
    .map_err(
        crate::admin_create_roles_payload_example_error::AdminCreateRolesPayloadExampleError::Name,
    )?;
    let roles = server_admin_contract::admin_create_roles_request::AdminCreateRolesRequest::try_from(
        vec![server_admin_contract::admin_create_role_request::AdminCreateRoleRequest::new(name)],
    )
    .map_err(|admin_collection_error| {
        crate::admin_create_roles_payload_example_error::AdminCreateRolesPayloadExampleError::Collection(
            server_observability::observed_error::ObservedError::capture(
                admin_collection_error,
                server_observability::observed_error_code::ObservedErrorCode::from(
                    constants_str::ADMIN_OBSERVED_ERROR_CREATE_ROLES_PAYLOAD_EXAMPLE,
                ),
            ),
        )
    })?;
    Ok(crate::json_response::json_response(roles))
}
