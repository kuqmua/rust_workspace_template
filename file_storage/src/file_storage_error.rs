#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum FileStorageError {
    #[error(
        "{}",
        constants_str::test_fixtures::FILE_STORAGE_ATOMIC_REPLACE_AND_CLEANUP_ERROR
    )]
    AtomicReplaceAndCleanup {
        cleanup: crate::file_storage_io_error::FileStorageIoError,
        replace: crate::file_storage_io_error::FileStorageIoError,
    },
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_DESTINATION_EXISTS)]
    DestinationExists,
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_IO_ERROR)]
    Io(#[source] crate::file_storage_io_error::FileStorageIoError),
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_SOURCE_NOT_REGULAR)]
    SourceNotRegular,
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_STAGING_ENTRY_EXISTS)]
    StagingEntryExists,
    #[error("{}", constants_str::test_fixtures::FILE_STORAGE_PATH_IS_SYMLINK)]
    Symlink,
}
