#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum FileStoragePathError {
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_FILE_TOO_LARGE)]
    FileTooLarge,
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_OPERATION_ID_INVALID)]
    OperationIdInvalid,
    #[error("{}", constants_str::catalog::FILE_STORAGE_PATH_TOO_LONG)]
    PathTooLong,
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_RELATIVE_PATH_INVALID)]
    RelativePathInvalid,
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_ROOT_MUST_BE_ABSOLUTE)]
    RootMustBeAbsolute,
}
