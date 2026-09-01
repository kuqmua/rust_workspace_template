#[derive(generate_accessor::Getters, generate_constructor::New)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct HttpErrorTelemetry {
    #[getters(copy)]
    #[constructor(order = 1)]
    error_code: crate::http_error_code::HttpErrorCode,
    #[getters(copy)]
    #[constructor(order = 0)]
    error_type: crate::http_error_type::HttpErrorType,
}
