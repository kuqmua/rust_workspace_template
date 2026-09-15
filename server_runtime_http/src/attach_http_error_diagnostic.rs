#[must_use]
pub fn attach_http_error_diagnostic(
    mut response: axum::response::Response,
    http_error_diagnostic: crate::http_error_diagnostic::HttpErrorDiagnostic,
) -> axum::response::Response {
    let _previous = response.extensions_mut().insert(http_error_diagnostic);
    response
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_attaches_http_error_diagnostic() {
        let http_error_telemetry = crate::http_error_telemetry::HttpErrorTelemetry::new(
            crate::http_error_type::HttpErrorType::from(constants_str::VALUE_AF7C24A2),
            crate::http_error_code::HttpErrorCode::from(constants_str::VALUE_CF4DCEBB),
        );
        let http_error_diagnostic = crate::http_error_diagnostic::HttpErrorDiagnostic::capture(
            http_error_telemetry,
            &crate::http_error_without_diagnostic_context::HttpErrorWithoutDiagnosticContext::Missing,
        );
        let response = super::attach_http_error_diagnostic(
            axum::response::Response::new(axum::body::Body::empty()),
            http_error_diagnostic,
        );
        assert!(
            response
                .extensions()
                .get::<crate::http_error_diagnostic::HttpErrorDiagnostic>()
                .is_some()
        );
    }
}
