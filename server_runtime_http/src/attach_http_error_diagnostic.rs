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
    #[tokio::test]
    async fn test_http_error_diagnostic_attachment_replaces_metadata_and_preserves_response() {
        let previous_telemetry = crate::http_error_telemetry::HttpErrorTelemetry::new(
            crate::http_error_type::HttpErrorType::from(constants_str::X),
            crate::http_error_code::HttpErrorCode::from(constants_str::X),
        );
        let telemetry = crate::http_error_telemetry::HttpErrorTelemetry::new(
            crate::http_error_type::HttpErrorType::from(constants_str::VALUE_AF7C24A2),
            crate::http_error_code::HttpErrorCode::from(constants_str::VALUE_CF4DCEBB),
        );
        let mut original = axum::response::Response::new(axum::body::Body::from(constants_str::X));
        *original.status_mut() = http::StatusCode::CREATED;
        *original.version_mut() = http::Version::HTTP_2;
        let _previous_header = original.headers_mut().insert(
            http::header::RETRY_AFTER,
            http::HeaderValue::from_static(constants_str::VALUE_2),
        );
        let _previous_unrelated =
            original
                .extensions_mut()
                .insert(crate::http_error_type::HttpErrorType::from(
                    constants_str::X,
                ));
        let previous = crate::http_error_diagnostic::HttpErrorDiagnostic::capture(
            previous_telemetry,
            &crate::http_error_without_diagnostic_context::HttpErrorWithoutDiagnosticContext::Missing,
        );
        let _previous_diagnostic = original.extensions_mut().insert(previous);
        let diagnostic = crate::http_error_diagnostic::HttpErrorDiagnostic::capture(
            telemetry,
            &crate::http_error_without_diagnostic_context::HttpErrorWithoutDiagnosticContext::Missing,
        );
        let location = diagnostic.location().to_string();
        let response =
            crate::attach_http_error_diagnostic::attach_http_error_diagnostic(original, diagnostic);
        assert!(
            response
                .extensions()
                .get::<crate::http_error_diagnostic::HttpErrorDiagnostic>()
                .is_some_and(|attached| {
                    attached.telemetry().error_type().to_string() == constants_str::VALUE_AF7C24A2
                        && attached.telemetry().error_code().to_string()
                            == constants_str::VALUE_CF4DCEBB
                        && attached.location().to_string() == location
                })
        );
        assert_eq!(response.status(), http::StatusCode::CREATED);
        assert_eq!(response.version(), http::Version::HTTP_2);
        assert_eq!(
            response.headers().get(http::header::RETRY_AFTER),
            Some(&http::HeaderValue::from_static(constants_str::VALUE_2))
        );
        assert!(
            response
                .extensions()
                .get::<crate::http_error_type::HttpErrorType>()
                .is_some_and(|http_error_type| http_error_type.to_string() == constants_str::X)
        );
        assert!(
            axum::body::to_bytes(response.into_body(), constants_str::X.len())
                .await
                .is_ok_and(|bytes| bytes.as_ref() == constants_str::X.as_bytes())
        );
    }
}
