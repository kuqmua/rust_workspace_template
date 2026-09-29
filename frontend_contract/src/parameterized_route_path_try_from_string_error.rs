#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum ParameterizedRoutePathTryFromStringError {
    #[error("{}", constants_str::PARAMETERIZED_ROUTE_PATH_TOO_LONG)]
    TooLong,
}
