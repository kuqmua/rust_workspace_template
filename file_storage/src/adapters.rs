#[allow(clippy::arbitrary_source_item_ordering)] // transactional API is grouped as prepare, stage, commit, and rollback operations
impl crate::domain_types::SafeFileStorage {
    pub async fn cleanup_stale_staging(
        &self,
        area: crate::domain_types::FileStorageStagingArea,
        cfg: crate::domain_types::StaleStagingCleanupCfg,
    ) -> Result<crate::domain_types::StaleStagingCleanupReport, crate::domain_types::FileStorageError>
    {
        let directory = self.root().get().join(area.directory_name().get());
        self.ensure_directory_not_symlink(directory.as_path().into())
            .await?;
        let mut entries = tokio::fs::read_dir(directory)
            .await
            .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
        let mut report = crate::domain_types::StaleStagingCleanupReport::default();
        while report.scanned().get() < cfg.maximum_scanned().get()
            && report.removed().get() < cfg.maximum_removed().get()
        {
            let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?
            else {
                break;
            };
            report.record_scanned();
            let file_type = entry
                .file_type()
                .await
                .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
            if file_type.is_dir() || file_type.is_symlink() {
                continue;
            }
            let metadata = entry
                .metadata()
                .await
                .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
            let Ok(modified) = metadata.modified() else {
                continue;
            };
            if modified > cfg.stale_before().get() {
                continue;
            }
            match tokio::fs::remove_file(entry.path()).await {
                Ok(()) => report.record_removed(),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(crate::domain_types::FileStorageError::Io(error.into())),
            }
        }
        Ok(report)
    }
    pub async fn atomic_replace(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
        destination: &crate::domain_types::StorageRelativePathBuf,
        bytes: &crate::domain_types::StdFileBytes,
        durability: crate::domain_types::AtomicReplaceDurability,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        self.stage_upload(operation_id, bytes).await?;
        let staging_path = self
            .root()
            .get()
            .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(operation_id.as_ref());
        if durability == crate::domain_types::AtomicReplaceDurability::SyncAll {
            let file = tokio::fs::OpenOptions::new()
                .write(true)
                .open(&staging_path)
                .await
                .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
            file.sync_all()
                .await
                .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
        }
        self.ensure_destination_parent(destination).await?;
        let destination_path = self.root().get().join(destination.as_ref());
        match tokio::fs::symlink_metadata(&destination_path).await {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(crate::domain_types::FileStorageError::Symlink);
            }
            Ok(metadata) if !metadata.is_file() => {
                return Err(crate::domain_types::FileStorageError::SourceNotRegular);
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(crate::domain_types::FileStorageError::Io(error.into())),
        }
        if let Err(replace) = tokio::fs::rename(&staging_path, destination_path).await {
            return match tokio::fs::remove_file(staging_path).await {
                Ok(()) => Err(crate::domain_types::FileStorageError::Io(replace.into())),
                Err(cleanup) => Err(
                    crate::domain_types::FileStorageError::AtomicReplaceAndCleanup {
                        cleanup: cleanup.into(),
                        replace: replace.into(),
                    },
                ),
            };
        }
        Ok(())
    }

    pub async fn prepare(&self) -> Result<(), crate::domain_types::FileStorageError> {
        tokio::fs::create_dir_all(self.root().get())
            .await
            .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
        self.ensure_directory_not_symlink(self.root()).await?;
        self.prepare_staging_directory(constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into())
            .await?;
        self.prepare_staging_directory(constants_str::FILE_DELETE_STAGING_DIRECTORY.into())
            .await
    }

    pub async fn stage_upload(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
        bytes: &crate::domain_types::StdFileBytes,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        let staging_path = self
            .root()
            .get()
            .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(operation_id.as_ref());
        let mut file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(staging_path)
            .await
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    crate::domain_types::FileStorageError::StagingEntryExists
                } else {
                    crate::domain_types::FileStorageError::Io(error.into())
                }
            })?;
        tokio::io::AsyncWriteExt::write_all(&mut file, bytes.as_ref())
            .await
            .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
        tokio::io::AsyncWriteExt::flush(&mut file)
            .await
            .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))
    }

    pub async fn commit_upload(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
        destination: &crate::domain_types::StorageRelativePathBuf,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        self.rename_staged(
            operation_id,
            destination,
            constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into(),
        )
        .await
    }

    pub async fn rollback_upload(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        self.remove_staged(
            operation_id,
            constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into(),
        )
        .await
    }

    pub async fn stage_delete(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
        source: &crate::domain_types::StorageRelativePathBuf,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        let source_path = self.root().get().join(source.as_ref());
        self.ensure_regular_file(source_path.as_path().into())
            .await?;
        tokio::fs::rename(
            source_path,
            self.root()
                .get()
                .join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
                .join(operation_id.as_ref()),
        )
        .await
        .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))
    }

    pub async fn rollback_delete(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
        destination: &crate::domain_types::StorageRelativePathBuf,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        self.rename_staged(
            operation_id,
            destination,
            constants_str::FILE_DELETE_STAGING_DIRECTORY.into(),
        )
        .await
    }

    pub async fn commit_delete(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        self.remove_staged(
            operation_id,
            constants_str::FILE_DELETE_STAGING_DIRECTORY.into(),
        )
        .await
    }

    async fn remove_staged(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
        directory_name: crate::domain_types::StorageDirectoryNameRef<'_>,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        tokio::fs::remove_file(
            self.root()
                .get()
                .join(directory_name.get())
                .join(operation_id.as_ref()),
        )
        .await
        .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))
    }

    async fn rename_staged(
        &self,
        operation_id: &crate::domain_types::StdStorageOperationId,
        destination: &crate::domain_types::StorageRelativePathBuf,
        directory_name: crate::domain_types::StorageDirectoryNameRef<'_>,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        let destination_path = self.root().get().join(destination.as_ref());
        self.ensure_destination_parent(destination).await?;
        self.ensure_destination_absent(destination_path.as_path().into())
            .await?;
        tokio::fs::rename(
            self.root()
                .get()
                .join(directory_name.get())
                .join(operation_id.as_ref()),
            destination_path,
        )
        .await
        .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))
    }

    async fn prepare_staging_directory(
        &self,
        directory_name: crate::domain_types::StorageDirectoryNameRef<'_>,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        let path = self.root().get().join(directory_name.get());
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
        self.ensure_directory_not_symlink(path.as_path().into())
            .await
    }

    async fn ensure_directory_not_symlink(
        &self,
        path: crate::domain_types::StoragePathRef<'_>,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        let metadata = tokio::fs::symlink_metadata(path.get())
            .await
            .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            Ok(())
        } else {
            Err(crate::domain_types::FileStorageError::Symlink)
        }
    }

    async fn ensure_destination_absent(
        &self,
        path: crate::domain_types::StoragePathRef<'_>,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        match tokio::fs::symlink_metadata(path.get()).await {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                Err(crate::domain_types::FileStorageError::Symlink)
            }
            Ok(_metadata) => Err(crate::domain_types::FileStorageError::DestinationExists),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(crate::domain_types::FileStorageError::Io(error.into())),
        }
    }

    async fn ensure_destination_parent(
        &self,
        relative_path: &crate::domain_types::StorageRelativePathBuf,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        let mut current = self.root().get().to_path_buf();
        let mut components = relative_path
            .as_ref()
            .parent()
            .into_iter()
            .flat_map(std::path::Path::components);
        #[allow(clippy::while_let_on_iterator)]
        // repository policy forbids for loops and each component requires awaited filesystem validation
        while let Some(component) = components.next() {
            current.push(component.as_os_str());
            match tokio::fs::symlink_metadata(&current).await {
                Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
                Ok(_metadata) => return Err(crate::domain_types::FileStorageError::Symlink),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    tokio::fs::create_dir(&current)
                        .await
                        .map_err(|create_error| {
                            crate::domain_types::FileStorageError::Io(create_error.into())
                        })?;
                    self.ensure_directory_not_symlink(current.as_path().into())
                        .await?;
                }
                Err(error) => return Err(crate::domain_types::FileStorageError::Io(error.into())),
            }
        }
        Ok(())
    }

    async fn ensure_regular_file(
        &self,
        path: crate::domain_types::StoragePathRef<'_>,
    ) -> Result<(), crate::domain_types::FileStorageError> {
        let metadata = tokio::fs::symlink_metadata(path.get())
            .await
            .map_err(|error| crate::domain_types::FileStorageError::Io(error.into()))?;
        if metadata.is_file() && !metadata.file_type().is_symlink() {
            Ok(())
        } else if metadata.file_type().is_symlink() {
            Err(crate::domain_types::FileStorageError::Symlink)
        } else {
            Err(crate::domain_types::FileStorageError::SourceNotRegular)
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn prepare_creates_owned_staging_directories() {
        let root =
            std::env::temp_dir().join(format!("file-storage-adapter-test-{}", std::process::id()));
        let storage = crate::domain_types::SafeFileStorage::new(
            crate::domain_types::FileStorageRootPathBuf::try_from(root.clone())
                .expect("f2ba8084 adapter test root must be valid"),
        );
        storage
            .prepare()
            .await
            .expect("ef6bfe8c staging directories must be created");
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
            .expect("1fc58e0b adapter test directory must be removable");
    }
}
