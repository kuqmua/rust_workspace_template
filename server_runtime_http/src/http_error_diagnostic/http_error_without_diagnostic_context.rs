#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{}", constants_str::HTTP_ERROR_WITHOUT_DIAGNOSTIC_CONTEXT)]
pub(super) struct HttpErrorWithoutDiagnosticContext;
