// The owner module retains lint-sensitive semantics from the original implementation.
#[track_caller]
#[allow(
    clippy::single_call_fn,
    reason = "context-free capture remains shared with focused diagnostic tests"
)]
pub(crate) fn capture_without_context(
    telemetry: super::HttpErrorTelemetry,
) -> super::HttpErrorDiagnostic {
    super::HttpErrorDiagnostic::capture(telemetry, &super::HttpErrorWithoutDiagnosticContext)
}
