#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminDeleteRolesPayloadExampleError {
    #[error(
        "{}",
        constants_str::ADMIN_DELETE_ROLES_PAYLOAD_EXAMPLE_ROLE_IDENTIFIER_ERROR
    )]
    RoleIdentifier(
        #[source]
        server_observability::observed_error::ObservedError<
            server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
        >,
    ),
}

impl axum::response::IntoResponse for AdminDeleteRolesPayloadExampleError {
    fn into_response(self) -> axum::response::Response {
        crate::admin_observed_internal_error_response::admin_observed_internal_error_response(
            self,
            server_observability::observed_error_code::ObservedErrorCode::from(
                constants_str::ADMIN_OBSERVED_ERROR_DELETE_ROLES_PAYLOAD_EXAMPLE,
            ),
        )
    }
}
