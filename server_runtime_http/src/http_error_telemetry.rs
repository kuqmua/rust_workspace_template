#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct HttpErrorTelemetry {
    error_code: super::HttpErrorCode,
    error_type: super::HttpErrorType,
}

impl HttpErrorTelemetry {
    pub(crate) const fn error_code(self) -> super::HttpErrorCode {
        self.error_code
    }

    pub(crate) const fn error_type(self) -> super::HttpErrorType {
        self.error_type
    }

    #[must_use]
    pub const fn new(error_type: super::HttpErrorType, error_code: super::HttpErrorCode) -> Self {
        Self {
            error_code,
            error_type,
        }
    }
}
