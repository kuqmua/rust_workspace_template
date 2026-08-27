pub(super) fn health_unavailable_response() -> axum::response::Response {
    axum::response::IntoResponse::into_response(
        frontend_contract::domain_types::ApiProblemError::ServiceUnavailable,
    )
}
