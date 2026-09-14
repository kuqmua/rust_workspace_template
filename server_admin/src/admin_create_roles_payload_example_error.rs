#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminCreateRolesPayloadExampleError {
    #[error(
        "{}",
        constants_str::ADMIN_CREATE_ROLES_PAYLOAD_EXAMPLE_COLLECTION_ERROR
    )]
    Collection(
        #[source]
        server_observability::observed_error::ObservedError<
            server_admin_contract::admin_collection_error::AdminCollectionError,
        >,
    ),
    #[error("{}", constants_str::ADMIN_CREATE_ROLES_PAYLOAD_EXAMPLE_NAME_ERROR)]
    Name(server_admin_contract::admin_role_name::AdminRoleNameTryFromStringError),
}

impl axum::response::IntoResponse for AdminCreateRolesPayloadExampleError {
    fn into_response(self) -> axum::response::Response {
        crate::admin_observed_internal_error_response::admin_observed_internal_error_response(
            self,
            server_observability::observed_error_code::ObservedErrorCode::from(
                constants_str::ADMIN_OBSERVED_ERROR_CREATE_ROLES_PAYLOAD_EXAMPLE,
            ),
        )
    }
}
