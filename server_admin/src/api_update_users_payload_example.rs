#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_users_payload_example() -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::admin_update_users_payload_example_error::AdminUpdateUsersPayloadExampleError,
> {
    let user_id = server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
        .map_err(|admin_id_try_from_i64_error| {
            crate::admin_update_users_payload_example_error::AdminUpdateUsersPayloadExampleError::UserIdentifier(
                server_observability::observed_error::ObservedError::capture(
                    admin_id_try_from_i64_error,
                    server_observability::observed_error_code::ObservedErrorCode::from(
                        constants_str::ADMIN_OBSERVED_ERROR_UPDATE_USERS_PAYLOAD_EXAMPLE,
                    ),
                ),
            )
        })?;
    let changes = server_admin_contract::admin_update_user_request::AdminUpdateUserRequest::new(
        None,
        None,
        None,
        None,
        None,
        Some(server_admin_contract::admin_bool::AdminBool::from(false)),
    );
    let filter = server_admin_contract::admin_user_filter::AdminUserFilter::new(
        Some(user_id),
        None,
        None,
        None,
    );
    let updates = server_admin_contract::admin_user_updates::AdminUserUpdates::try_from(vec![
        server_admin_contract::admin_user_update::AdminUserUpdate::new(changes, filter),
    ])
    .map_err(|admin_collection_error| {
        crate::admin_update_users_payload_example_error::AdminUpdateUsersPayloadExampleError::Collection(
            server_observability::observed_error::ObservedError::capture(
                admin_collection_error,
                server_observability::observed_error_code::ObservedErrorCode::from(
                    constants_str::ADMIN_OBSERVED_ERROR_UPDATE_USERS_PAYLOAD_EXAMPLE,
                ),
            ),
        )
    })?;
    Ok(crate::json_response::json_response(
        server_admin_contract::admin_update_users_request::AdminUpdateUsersRequest::new(updates),
    ))
}
