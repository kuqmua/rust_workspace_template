#[derive(proc_macro_getters::Getters, proc_macro_new::New)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct HttpErrorTelemetry {
    #[getters(copy)]
    #[constructor(order = 1)]
    error_code: crate::http_error_code::HttpErrorCode,
    #[getters(copy)]
    #[constructor(order = 0)]
    error_type: crate::http_error_type::HttpErrorType,
}
