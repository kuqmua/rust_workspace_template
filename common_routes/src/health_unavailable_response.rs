#[allow(clippy::single_call_fn)] // shared response construction remains isolated from the health error representation
pub(super) fn health_unavailable_response() -> axum::response::Response {
    axum::response::IntoResponse::into_response(
        frontend_contract::domain_types::ApiProblemError::ServiceUnavailable,
    )
}
