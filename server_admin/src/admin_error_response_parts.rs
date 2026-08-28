pub(crate) fn admin_error_response_parts(
    route_error_status: frontend_contract::RouteErrorStatus,
    optional_diagnostic: Option<server_runtime_http::domain_types::HttpErrorDiagnostic>,
) -> axum::response::Response {
    let problem_status = frontend_contract::ApiProblemStatus::try_from(u16::from(
        route_error_status.transport_status(),
    ))
    .unwrap_or_else(|_error| {
        frontend_contract::ApiProblemStatus::from(
            frontend_contract::KnownHttpStatus::InternalServerError,
        )
    });
    let mut response = axum::response::IntoResponse::into_response(
        frontend_contract::ApiProblemError::from_status(problem_status),
    );
    if let Some(diagnostic) = optional_diagnostic {
        let _previous_diagnostic = response.extensions_mut().insert(diagnostic);
    }
    response
}
