pub(crate) fn admin_error_response_parts(
    route_error_status: frontend_contract::route_error_status::RouteErrorStatus,
    http_error_diagnostic: Option<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>,
) -> axum::response::Response {
    let problem_status = frontend_contract::api_problem_status::ApiProblemStatus::try_from(
        u16::from(route_error_status.transport_status()),
    )
    .unwrap_or_else(|_error| {
        frontend_contract::api_problem_status::ApiProblemStatus::from(
            frontend_contract::known_http_status::KnownHttpStatus::InternalServerError,
        )
    });
    let response = axum::response::IntoResponse::into_response(
        frontend_contract::api_problem_error::ApiProblemError::from_status(problem_status),
    );
    if let Some(http_error_diagnostic) = http_error_diagnostic {
        server_runtime_http::attach_http_error_diagnostic::attach_http_error_diagnostic(
            response,
            http_error_diagnostic,
        )
    } else {
        response
    }
}
