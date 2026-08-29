#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::test_fixtures::INVALID_API_URL_PATH_SEGMENT)]
pub enum ApiUrlBuildError {
    InvalidPathSegment,
}
