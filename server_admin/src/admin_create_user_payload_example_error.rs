#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminCreateUserPayloadExampleError {
    #[error(
        "{}",
        constants_str::ADMIN_CREATE_USER_PAYLOAD_EXAMPLE_DISPLAY_NAME_ERROR
    )]
    DisplayName(server_admin_contract::admin_display_name::AdminDisplayNameTryFromStringError),
    #[error("{}", constants_str::ADMIN_CREATE_USER_PAYLOAD_EXAMPLE_LOGIN_ERROR)]
    Login(server_admin_contract::admin_login::AdminLoginTryFromStringError),
    #[error("{}", constants_str::ADMIN_CREATE_USER_PAYLOAD_EXAMPLE_PASSWORD_ERROR)]
    Password(server_admin_contract::admin_new_password::AdminNewPasswordTryFromStringError),
}

impl axum::response::IntoResponse for AdminCreateUserPayloadExampleError {
    fn into_response(self) -> axum::response::Response {
        crate::admin_observed_internal_error_response::admin_observed_internal_error_response(
            self,
            server_observability::observed_error_code::ObservedErrorCode::from(
                constants_str::ADMIN_OBSERVED_ERROR_CREATE_USER_PAYLOAD_EXAMPLE,
            ),
        )
    }
}
