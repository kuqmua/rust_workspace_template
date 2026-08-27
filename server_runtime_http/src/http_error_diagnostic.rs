#[path = "http_error_diagnostic/capture_without_context.rs"]
mod capture_without_context;
#[path = "http_error_diagnostic/http_error_code.rs"]
mod http_error_code;
#[path = "http_error_diagnostic/http_error_diagnostic.rs"]
mod http_error_diagnostic;
#[path = "http_error_diagnostic/http_error_telemetry.rs"]
mod http_error_telemetry;
#[path = "http_error_diagnostic/http_error_type.rs"]
mod http_error_type;
#[path = "http_error_diagnostic/http_error_without_diagnostic_context.rs"]
mod http_error_without_diagnostic_context;
#[path = "http_error_diagnostic/std_http_error_backtrace.rs"]
mod std_http_error_backtrace;
#[path = "http_error_diagnostic/std_http_error_chain.rs"]
mod std_http_error_chain;
#[path = "http_error_diagnostic/tracing_http_span_trace.rs"]
mod tracing_http_span_trace;

pub(super) use capture_without_context::capture_without_context;
pub use http_error_code::HttpErrorCode;
pub use http_error_diagnostic::HttpErrorDiagnostic;
pub use http_error_telemetry::HttpErrorTelemetry;
pub use http_error_type::HttpErrorType;
use http_error_without_diagnostic_context::HttpErrorWithoutDiagnosticContext;
pub(super) use std_http_error_backtrace::StdHttpErrorBacktrace;
pub(super) use std_http_error_chain::StdHttpErrorChain;
pub(super) use tracing_http_span_trace::TracingHttpSpanTrace;

#[cfg(test)]
mod tests {
    #[test]
    fn fallback_diagnostic_keeps_telemetry() {
        let telemetry = super::HttpErrorTelemetry::new(
            super::HttpErrorType::from(constants_str::VALUE_AF7C24A2),
            super::HttpErrorCode::from(constants_str::VALUE_CF4DCEBB),
        );
        let diagnostic = super::capture_without_context(telemetry);
        assert_eq!(
            diagnostic.telemetry().error_code().to_string(),
            "test_failure"
        );
        assert_eq!(
            diagnostic.telemetry().error_type().to_string(),
            "test.error"
        );
    }
}
