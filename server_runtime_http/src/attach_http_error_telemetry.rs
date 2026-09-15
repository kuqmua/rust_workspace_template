#[must_use]
pub fn attach_http_error_telemetry(
    mut response: axum::response::Response,
    http_error_telemetry: crate::http_error_telemetry::HttpErrorTelemetry,
) -> axum::response::Response {
    let _previous = response.extensions_mut().insert(http_error_telemetry);
    response
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_attaches_http_error_telemetry() {
        let response = super::attach_http_error_telemetry(
            axum::response::Response::new(axum::body::Body::empty()),
            crate::http_error_telemetry::HttpErrorTelemetry::new(
                crate::http_error_type::HttpErrorType::from(constants_str::VALUE_AF7C24A2),
                crate::http_error_code::HttpErrorCode::from(constants_str::VALUE_CF4DCEBB),
            ),
        );
        assert!(
            response
                .extensions()
                .get::<crate::http_error_telemetry::HttpErrorTelemetry>()
                .is_some()
        );
    }
}
