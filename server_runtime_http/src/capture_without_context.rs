#[track_caller]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)]
pub(in crate::domain_types) fn capture_without_context(
    telemetry: super::HttpErrorTelemetry,
) -> super::HttpErrorDiagnostic {
    super::HttpErrorDiagnostic::capture(telemetry, &super::HttpErrorWithoutDiagnosticContext)
}
