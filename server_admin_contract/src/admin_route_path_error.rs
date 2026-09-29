#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum AdminRoutePathError {
    #[error("{}", constants_str::ADMINISTRATOR_ROUTE_PATH_IS_TOO_LONG)]
    TooLong,
    #[error("{}", constants_str::ADMINISTRATOR_ROUTE_PATH_IS_TOO_LONG)]
    ParameterizedPath(
        #[from] frontend_contract::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError,
    ),
}
