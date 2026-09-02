#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum HttpErrorClass {
    Authentication,
    Conflict,
    Forbidden,
    Internal,
    NotFound,
    PayloadTooLarge,
    RateLimited,
    ServiceUnavailable,
    Timeout,
    UnexpectedSuccess,
    Validation,
}
