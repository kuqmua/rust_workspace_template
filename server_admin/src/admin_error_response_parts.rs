pub(crate) fn admin_error_response_parts(
    route_error_status: frontend_contract::route_error_status::RouteErrorStatus,
    option: Option<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>,
) -> axum::response::Response {
    let problem_status = frontend_contract::api_problem_status::ApiProblemStatus::try_from(
        u16::from(route_error_status.transport_status()),
    )
    .unwrap_or_else(|_error| {
        frontend_contract::api_problem_status::ApiProblemStatus::from(
            frontend_contract::known_http_status::KnownHttpStatus::InternalServerError,
        )
    });
    let mut response = axum::response::IntoResponse::into_response(
        frontend_contract::api_problem_error::ApiProblemError::from_status(problem_status),
    );
    if let Some(diagnostic) = option {
        let _previous_diagnostic = response.extensions_mut().insert(diagnostic);
    }
    response
}
