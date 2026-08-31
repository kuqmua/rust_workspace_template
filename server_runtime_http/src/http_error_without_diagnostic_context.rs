#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(super) enum HttpErrorWithoutDiagnosticContext {
    #[error("{}", constants_str::HTTP_ERROR_WITHOUT_DIAGNOSTIC_CONTEXT)]
    Missing,
}
