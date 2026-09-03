#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "adapters requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]
impl crate::safe_file_storage::SafeFileStorage {
    pub async fn cleanup_stale_staging(
        &self,
        file_storage_staging_area: crate::file_storage_staging_area::FileStorageStagingArea,
        stale_staging_cleanup_configuration: crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration,
    ) -> Result<
        crate::stale_staging_cleanup_report::StaleStagingCleanupReport,
        crate::file_storage_error::FileStorageError,
    > {
        let directory = self
            .root()
            .get()
            .join(file_storage_staging_area.directory_name().get());
        self.ensure_directory_not_symlink(directory.as_path().into())
            .await?;
        let mut entries = tokio::fs::read_dir(directory)
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        let mut report = crate::stale_staging_cleanup_report::StaleStagingCleanupReport::default();
        while report.scanned().get() < stale_staging_cleanup_configuration.maximum_scanned().get()
            && report.removed().get() < stale_staging_cleanup_configuration.maximum_removed().get()
        {
            let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?
            else {
                break;
            };
            report.record_scanned();
            let file_type = entry
                .file_type()
                .await
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
            if file_type.is_dir() || file_type.is_symlink() {
                continue;
            }
            let metadata = entry
                .metadata()
                .await
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
            let Ok(modified) = metadata.modified() else {
                continue;
            };
            if modified > stale_staging_cleanup_configuration.stale_before().get() {
                continue;
            }
            match tokio::fs::remove_file(entry.path()).await {
                Ok(()) => report.record_removed(),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(crate::file_storage_error::FileStorageError::Io(
                        error.into(),
                    ));
                }
            }
        }
        Ok(report)
    }
    pub async fn atomic_replace(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
        std_file_bytes: &crate::std_file_bytes::StdFileBytes,
        atomic_replace_durability: crate::atomic_replace_durability::AtomicReplaceDurability,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        self.stage_upload(std_storage_operation_id, std_file_bytes)
            .await?;
        let staging_path = self
            .root()
            .get()
            .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(std_storage_operation_id.as_ref());
        if atomic_replace_durability
            == crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll
        {
            let file = tokio::fs::OpenOptions::new()
                .write(true)
                .open(&staging_path)
                .await
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
            file.sync_all()
                .await
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        }
        self.ensure_destination_parent(storage_relative_path_buf)
            .await?;
        let destination_path = self.root().get().join(storage_relative_path_buf.as_ref());
        match tokio::fs::symlink_metadata(&destination_path).await {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(crate::file_storage_error::FileStorageError::Symlink);
            }
            Ok(metadata) if !metadata.is_file() => {
                return Err(crate::file_storage_error::FileStorageError::SourceNotRegular);
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(crate::file_storage_error::FileStorageError::Io(
                    error.into(),
                ));
            }
        }
        if let Err(replace) = tokio::fs::rename(&staging_path, destination_path).await {
            return match tokio::fs::remove_file(staging_path).await {
                Ok(()) => Err(crate::file_storage_error::FileStorageError::Io(
                    replace.into(),
                )),
                Err(cleanup) => Err(
                    crate::file_storage_error::FileStorageError::AtomicReplaceAndCleanup {
                        cleanup: cleanup.into(),
                        replace: replace.into(),
                    },
                ),
            };
        }
        Ok(())
    }

    pub async fn prepare(&self) -> Result<(), crate::file_storage_error::FileStorageError> {
        tokio::fs::create_dir_all(self.root().get())
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        self.ensure_directory_not_symlink(self.root()).await?;
        self.prepare_staging_directory(constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into())
            .await?;
        self.prepare_staging_directory(constants_str::FILE_DELETE_STAGING_DIRECTORY.into())
            .await
    }

    pub async fn stage_upload(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        std_file_bytes: &crate::std_file_bytes::StdFileBytes,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let staging_path = self
            .root()
            .get()
            .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(std_storage_operation_id.as_ref());
        let mut file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(staging_path)
            .await
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    crate::file_storage_error::FileStorageError::StagingEntryExists
                } else {
                    crate::file_storage_error::FileStorageError::Io(error.into())
                }
            })?;
        tokio::io::AsyncWriteExt::write_all(&mut file, std_file_bytes.as_ref())
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        tokio::io::AsyncWriteExt::flush(&mut file)
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))
    }

    pub async fn commit_upload(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        self.rename_staged(
            std_storage_operation_id,
            storage_relative_path_buf,
            constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into(),
        )
        .await
    }

    pub async fn rollback_upload(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        self.remove_staged(
            std_storage_operation_id,
            constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into(),
        )
        .await
    }

    pub async fn stage_delete(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let source_path = self.root().get().join(storage_relative_path_buf.as_ref());
        let metadata = tokio::fs::symlink_metadata(source_path.as_path())
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(if metadata.file_type().is_symlink() {
                crate::file_storage_error::FileStorageError::Symlink
            } else {
                crate::file_storage_error::FileStorageError::SourceNotRegular
            });
        }
        tokio::fs::rename(
            source_path,
            self.root()
                .get()
                .join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
                .join(std_storage_operation_id.as_ref()),
        )
        .await
        .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))
    }

    pub async fn rollback_delete(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        self.rename_staged(
            std_storage_operation_id,
            storage_relative_path_buf,
            constants_str::FILE_DELETE_STAGING_DIRECTORY.into(),
        )
        .await
    }

    pub async fn commit_delete(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        self.remove_staged(
            std_storage_operation_id,
            constants_str::FILE_DELETE_STAGING_DIRECTORY.into(),
        )
        .await
    }

    async fn remove_staged(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        storage_directory_name_ref: crate::storage_directory_name_ref::StorageDirectoryNameRef<'_>,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        tokio::fs::remove_file(
            self.root()
                .get()
                .join(storage_directory_name_ref.get())
                .join(std_storage_operation_id.as_ref()),
        )
        .await
        .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))
    }

    async fn rename_staged(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
        storage_directory_name_ref: crate::storage_directory_name_ref::StorageDirectoryNameRef<'_>,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let destination_path = self.root().get().join(storage_relative_path_buf.as_ref());
        self.ensure_destination_parent(storage_relative_path_buf)
            .await?;
        match tokio::fs::symlink_metadata(destination_path.as_path()).await {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(crate::file_storage_error::FileStorageError::Symlink);
            }
            Ok(_metadata) => {
                return Err(crate::file_storage_error::FileStorageError::DestinationExists);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(crate::file_storage_error::FileStorageError::Io(
                    error.into(),
                ));
            }
        }
        tokio::fs::rename(
            self.root()
                .get()
                .join(storage_directory_name_ref.get())
                .join(std_storage_operation_id.as_ref()),
            destination_path,
        )
        .await
        .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))
    }

    async fn prepare_staging_directory(
        &self,
        storage_directory_name_ref: crate::storage_directory_name_ref::StorageDirectoryNameRef<'_>,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let path = self.root().get().join(storage_directory_name_ref.get());
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        self.ensure_directory_not_symlink(path.as_path().into())
            .await
    }

    async fn ensure_directory_not_symlink(
        &self,
        storage_path_ref: crate::storage_path_ref::StoragePathRef<'_>,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let metadata = tokio::fs::symlink_metadata(storage_path_ref.get())
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            Ok(())
        } else {
            Err(crate::file_storage_error::FileStorageError::Symlink)
        }
    }

    async fn ensure_destination_parent(
        &self,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let mut current = self.root().get().to_path_buf();
        let mut components = storage_relative_path_buf
            .as_ref()
            .parent()
            .into_iter()
            .flat_map(std::path::Path::components);

        #[allow(
            clippy::while_let_on_iterator,
            reason = "adapters requires this localized allowance for generated or framework-constrained code verified by focused tests"
        )]
        while let Some(component) = components.next() {
            current.push(component.as_os_str());
            match tokio::fs::symlink_metadata(&current).await {
                Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
                Ok(_metadata) => return Err(crate::file_storage_error::FileStorageError::Symlink),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    tokio::fs::create_dir(&current)
                        .await
                        .map_err(|create_error| {
                            crate::file_storage_error::FileStorageError::Io(create_error.into())
                        })?;
                    self.ensure_directory_not_symlink(current.as_path().into())
                        .await?;
                }
                Err(error) => {
                    return Err(crate::file_storage_error::FileStorageError::Io(
                        error.into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_prepare_creates_owned_staging_directories() {
        let root =
            std::env::temp_dir().join(format!("file-storage-adapter-test-{}", std::process::id()));
        let storage = crate::safe_file_storage::SafeFileStorage::new(
            crate::file_storage_root_path_buf::FileStorageRootPathBuf::try_from(root.clone())
                .expect(constants_str::DIAGNOSTIC_F2BA8084),
        );
        storage
            .prepare()
            .await
            .expect(constants_str::DIAGNOSTIC_EF6BFE8C);
        assert!(
            root.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
                .is_dir()
        );
        assert!(
            root.join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
                .is_dir()
        );
        tokio::fs::remove_dir_all(root)
            .await
            .expect(constants_str::DIAGNOSTIC_1FC58E0B);
    }
}
