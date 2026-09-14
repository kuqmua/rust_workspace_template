#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_delete_roles_payload_example() -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::admin_delete_roles_payload_example_error::AdminDeleteRolesPayloadExampleError,
> {
    let role_id = server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
        .map_err(|admin_id_try_from_i64_error| {
            crate::admin_delete_roles_payload_example_error::AdminDeleteRolesPayloadExampleError::RoleIdentifier(
                server_observability::observed_error::ObservedError::capture(
                    admin_id_try_from_i64_error,
                    server_observability::observed_error_code::ObservedErrorCode::from(
                        constants_str::ADMIN_OBSERVED_ERROR_DELETE_ROLES_PAYLOAD_EXAMPLE,
                    ),
                ),
            )
        })?;
    let filter =
        server_admin_contract::admin_role_filter::AdminRoleFilter::new(Some(role_id), None, None);
    Ok(crate::json_response::json_response(
        server_admin_contract::admin_delete_roles_request::AdminDeleteRolesRequest::new(filter),
    ))
}
