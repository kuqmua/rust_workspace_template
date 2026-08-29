#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct HttpErrorTelemetry {
    error_code: crate::http_error_code::HttpErrorCode,
    error_type: crate::http_error_type::HttpErrorType,
}

impl HttpErrorTelemetry {
    pub(crate) const fn error_code(self) -> crate::http_error_code::HttpErrorCode {
        self.error_code
    }

    pub(crate) const fn error_type(self) -> crate::http_error_type::HttpErrorType {
        self.error_type
    }

    #[must_use]
    pub const fn new(
        error_type: crate::http_error_type::HttpErrorType,
        error_code: crate::http_error_code::HttpErrorCode,
    ) -> Self {
        Self {
            error_code,
            error_type,
        }
    }
}
