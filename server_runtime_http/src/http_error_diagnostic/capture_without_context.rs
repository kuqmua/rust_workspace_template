#[track_caller]
#[allow(clippy::single_call_fn)]
pub(in crate::domain_types) fn capture_without_context(
    telemetry: super::HttpErrorTelemetry,
) -> super::HttpErrorDiagnostic {
    super::HttpErrorDiagnostic::capture(telemetry, &super::HttpErrorWithoutDiagnosticContext)
}
