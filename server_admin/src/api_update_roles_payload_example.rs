#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_roles")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_roles_payload_example() -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::admin_update_roles_payload_example_error::AdminUpdateRolesPayloadExampleError,
> {
    let role_id = server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
        .map_err(|admin_id_try_from_i64_error| {
            crate::admin_update_roles_payload_example_error::AdminUpdateRolesPayloadExampleError::RoleIdentifier(
                server_observability::observed_error::ObservedError::capture(
                    admin_id_try_from_i64_error,
                    server_observability::observed_error_code::ObservedErrorCode::from(
                        constants_str::ADMIN_OBSERVED_ERROR_UPDATE_ROLES_PAYLOAD_EXAMPLE,
                    ),
                ),
            )
        })?;
    let name = server_admin_contract::admin_role_name::AdminRoleName::try_from(
        constants_str::ADMIN_FIXTURE_ROLE_NAME.to_owned(),
    )
    .map_err(
        crate::admin_update_roles_payload_example_error::AdminUpdateRolesPayloadExampleError::Name,
    )?;
    let changes = server_admin_contract::admin_update_role_request::AdminUpdateRoleRequest::new(
        Some(name),
        None,
    );
    let filter =
        server_admin_contract::admin_role_filter::AdminRoleFilter::new(Some(role_id), None, None);
    let updates = server_admin_contract::admin_role_updates::AdminRoleUpdates::try_from(vec![
        server_admin_contract::admin_role_update::AdminRoleUpdate::new(changes, filter),
    ])
    .map_err(|admin_collection_error| {
        crate::admin_update_roles_payload_example_error::AdminUpdateRolesPayloadExampleError::Collection(
            server_observability::observed_error::ObservedError::capture(
                admin_collection_error,
                server_observability::observed_error_code::ObservedErrorCode::from(
                    constants_str::ADMIN_OBSERVED_ERROR_UPDATE_ROLES_PAYLOAD_EXAMPLE,
                ),
            ),
        )
    })?;
    Ok(crate::json_response::json_response(
        server_admin_contract::admin_update_roles_request::AdminUpdateRolesRequest::new(updates),
    ))
}
