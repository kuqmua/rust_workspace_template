#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum FileStorageError {
    #[error("{}", constants_str::FILE_STORAGE_ATOMIC_REPLACE_AND_CLEANUP_ERROR)]
    AtomicReplaceAndCleanup {
        cleanup: crate::file_storage_io_error::FileStorageIoError,
        #[source]
        replace: crate::file_storage_io_error::FileStorageIoError,
    },
    #[error("{}", constants_str::FILE_STORAGE_DESTINATION_EXISTS)]
    DestinationExists,
    #[error("{}", constants_str::FILE_STORAGE_IO_ERROR)]
    Io(#[source] crate::file_storage_io_error::FileStorageIoError),
    #[error("{}", constants_str::FILE_STORAGE_SOURCE_NOT_REGULAR)]
    SourceNotRegular,
    #[error("{}", constants_str::FILE_STORAGE_STAGING_ENTRY_EXISTS)]
    StagingEntryExists,
    #[error("{}", constants_str::FILE_STORAGE_PATH_IS_SYMLINK)]
    Symlink,
}

impl FileStorageError {
    pub(crate) fn with_failed_cleanup(
        self,
        file_storage_io_error: crate::file_storage_io_error::FileStorageIoError,
    ) -> Self {
        let replace = match self {
            Self::Io(replace) => replace,
            operation @ (Self::AtomicReplaceAndCleanup { .. }
            | Self::DestinationExists
            | Self::SourceNotRegular
            | Self::StagingEntryExists
            | Self::Symlink) => crate::file_storage_io_error::FileStorageIoError::from(
                std::io::Error::other(operation),
            ),
        };
        Self::AtomicReplaceAndCleanup {
            cleanup: file_storage_io_error,
            replace,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_failed_cleanup_retains_operation_and_cleanup_io_errors() {
        let operation =
            super::FileStorageError::Io(crate::file_storage_io_error::FileStorageIoError::from(
                std::io::Error::from(std::io::ErrorKind::BrokenPipe),
            ));
        let cleanup_error = crate::file_storage_io_error::FileStorageIoError::from(
            std::io::Error::from(std::io::ErrorKind::PermissionDenied),
        );
        let combined = operation.with_failed_cleanup(cleanup_error);
        assert!(matches!(
            &combined,
            super::FileStorageError::AtomicReplaceAndCleanup { cleanup, replace }
                if replace.to_string()
                    == std::io::Error::from(std::io::ErrorKind::BrokenPipe).to_string()
                    && cleanup.to_string()
                        == std::io::Error::from(std::io::ErrorKind::PermissionDenied).to_string()
        ));
        assert!(std::error::Error::source(&combined).is_some());
    }

    #[test]
    fn test_failed_cleanup_retains_non_io_operation_error() {
        let cleanup = crate::file_storage_io_error::FileStorageIoError::from(std::io::Error::from(
            std::io::ErrorKind::PermissionDenied,
        ));
        let combined = super::FileStorageError::Symlink.with_failed_cleanup(cleanup);
        assert!(matches!(
            &combined,
            super::FileStorageError::AtomicReplaceAndCleanup { replace, .. }
                if replace.to_string() == super::FileStorageError::Symlink.to_string()
        ));
    }
}
