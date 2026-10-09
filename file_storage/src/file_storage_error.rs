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
        [
            super::FileStorageError::Symlink,
            super::FileStorageError::DestinationExists,
            super::FileStorageError::SourceNotRegular,
            super::FileStorageError::StagingEntryExists,
        ]
        .into_iter()
        .fold((), |(), operation| {
            let expected = operation.to_string();
            let cleanup_error = crate::file_storage_io_error::FileStorageIoError::from(
                std::io::Error::from(std::io::ErrorKind::PermissionDenied),
            );
            let combined = operation.with_failed_cleanup(cleanup_error);
            assert!(matches!(
                &combined,
                super::FileStorageError::AtomicReplaceAndCleanup { replace, cleanup }
                    if replace.to_string() == expected
                        && cleanup.to_string()
                            == std::io::Error::from(std::io::ErrorKind::PermissionDenied).to_string()
            ));
            assert!(std::error::Error::source(&combined).is_some());
        });
    }
    #[test]
    fn test_failed_cleanup_retains_previously_combined_error_and_original_source() {
        let original = super::FileStorageError::AtomicReplaceAndCleanup {
            replace: crate::file_storage_io_error::FileStorageIoError::from(std::io::Error::from(
                std::io::ErrorKind::BrokenPipe,
            )),
            cleanup: crate::file_storage_io_error::FileStorageIoError::from(std::io::Error::from(
                std::io::ErrorKind::PermissionDenied,
            )),
        };
        let original_debug = format!("{original:?}");
        let combined =
            original.with_failed_cleanup(crate::file_storage_io_error::FileStorageIoError::from(
                std::io::Error::from(std::io::ErrorKind::NotFound),
            ));
        let combined_debug = format!("{combined:?}");
        assert!(
            matches!(&combined, super::FileStorageError::AtomicReplaceAndCleanup { cleanup, .. }
            if cleanup.to_string() == std::io::Error::from(std::io::ErrorKind::NotFound).to_string()
                && combined_debug.contains(&original_debug))
        );
        assert!(
            std::error::Error::source(&combined).is_some_and(|replacement| {
                replacement
                    .downcast_ref::<crate::file_storage_io_error::FileStorageIoError>()
                    .is_some()
                    && std::error::Error::source(replacement).is_some_and(|original_source| {
                        original_source
                            .downcast_ref::<crate::file_storage_io_error::FileStorageIoError>()
                            .is_some()
                            && original_source.to_string()
                                == std::io::Error::from(std::io::ErrorKind::BrokenPipe).to_string()
                    })
            })
        );
    }
}
