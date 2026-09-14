#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_create_user_payload_example() -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError,
> {
    let display_name = server_admin_contract::admin_display_name::AdminDisplayName::try_from(
        constants_str::ADMIN_FIXTURE_ALPHA_DISPLAY_NAME.to_owned(),
    )
    .map_err(
        crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError::DisplayName,
    )?;
    let login = server_admin_contract::admin_login::AdminLogin::try_from(
        constants_str::ADMIN_FIXTURE_ALPHA_LOGIN.to_owned(),
    )
    .map_err(
        crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError::Login,
    )?;
    let password = server_admin_contract::admin_new_password::AdminNewPassword::try_from(
        constants_str::TEST_STRONG_PASSWORD.to_owned(),
    )
    .map_err(
        crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError::Password,
    )?;
    Ok(crate::json_response::json_response(
        server_admin_contract::admin_create_user_request::AdminCreateUserRequest::new(
            display_name,
            login,
            password,
            None,
        ),
    ))
}
