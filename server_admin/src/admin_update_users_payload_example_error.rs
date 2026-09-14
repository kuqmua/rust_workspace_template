#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminUpdateUsersPayloadExampleError {
    #[error(
        "{}",
        constants_str::ADMIN_UPDATE_USERS_PAYLOAD_EXAMPLE_COLLECTION_ERROR
    )]
    Collection(
        #[source]
        server_observability::observed_error::ObservedError<
            server_admin_contract::admin_collection_error::AdminCollectionError,
        >,
    ),
    #[error(
        "{}",
        constants_str::ADMIN_UPDATE_USERS_PAYLOAD_EXAMPLE_USER_IDENTIFIER_ERROR
    )]
    UserIdentifier(
        #[source]
        server_observability::observed_error::ObservedError<
            server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
        >,
    ),
}

impl axum::response::IntoResponse for AdminUpdateUsersPayloadExampleError {
    fn into_response(self) -> axum::response::Response {
        crate::admin_observed_internal_error_response::admin_observed_internal_error_response(
            self,
            server_observability::observed_error_code::ObservedErrorCode::from(
                constants_str::ADMIN_OBSERVED_ERROR_UPDATE_USERS_PAYLOAD_EXAMPLE,
            ),
        )
    }
}
