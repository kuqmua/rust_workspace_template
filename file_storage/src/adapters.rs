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
        self.run_capability_operation(move |root| {
            let directory_name = file_storage_staging_area.directory_name();
            Self::ensure_capability_directory(&root, directory_name)?;
            let mut entries = root
                .read_dir(directory_name.get())
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
            let mut report =
                crate::stale_staging_cleanup_report::StaleStagingCleanupReport::default();
            while report.scanned().get()
                < stale_staging_cleanup_configuration.maximum_scanned().get()
                && report.removed().get()
                    < stale_staging_cleanup_configuration.maximum_removed().get()
            {
                let Some(entry_result) = entries.next() else {
                    break;
                };
                let entry = entry_result.map_err(|error| {
                    crate::file_storage_error::FileStorageError::Io(error.into())
                })?;
                report.record_scanned();
                let file_type = entry.file_type().map_err(|error| {
                    crate::file_storage_error::FileStorageError::Io(error.into())
                })?;
                if file_type.is_dir() || file_type.is_symlink() {
                    continue;
                }
                let metadata = entry.metadata().map_err(|error| {
                    crate::file_storage_error::FileStorageError::Io(error.into())
                })?;
                let Ok(modified) = metadata.modified() else {
                    continue;
                };
                if modified.into_std() > stale_staging_cleanup_configuration.stale_before().get() {
                    continue;
                }
                match entry.remove_file() {
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
        })
        .await
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
        let operation_id = std_storage_operation_id.clone();
        let relative_path = storage_relative_path_buf.clone();
        self.run_capability_operation(move |root| {
            let staging_path = std::path::Path::new(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
                .join(operation_id.as_ref());
            let replace_result = (|| {
                Self::ensure_capability_directory(
                    &root,
                    constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into(),
                )?;
                if atomic_replace_durability
                    == crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll
                {
                    let file = root.open(&staging_path).map_err(|error| {
                        crate::file_storage_error::FileStorageError::Io(error.into())
                    })?;
                    file.sync_all().map_err(|error| {
                        crate::file_storage_error::FileStorageError::Io(error.into())
                    })?;
                }
                Self::ensure_capability_destination_parent(&root, &relative_path)?;
                match root.symlink_metadata(relative_path.as_ref()) {
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
                root.rename(&staging_path, &root, relative_path.as_ref())
                    .map_err(|error| {
                        crate::file_storage_error::FileStorageError::Io(error.into())
                    })?;
                if atomic_replace_durability
                    == crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll
                {
                    let destination_parent = relative_path
                        .as_ref()
                        .parent()
                        .unwrap_or_else(|| std::path::Path::new(constants_str::DOT));
                    let directory = root.open_dir(destination_parent).map_err(|error| {
                        crate::file_storage_error::FileStorageError::Io(error.into())
                    })?;
                    directory.into_std_file().sync_all().map_err(|error| {
                        crate::file_storage_error::FileStorageError::Io(error.into())
                    })?;
                }
                Ok(())
            })();
            if let Err(operation_error) = replace_result {
                return match root.remove_file(&staging_path) {
                    Ok(()) => Err(operation_error),
                    Err(cleanup) if cleanup.kind() == std::io::ErrorKind::NotFound => {
                        Err(operation_error)
                    }
                    Err(cleanup) => Err(crate::file_storage_error::FileStorageError::Io(
                        cleanup.into(),
                    )),
                };
            }
            Ok(())
        })
        .await
    }

    pub async fn prepare(&self) -> Result<(), crate::file_storage_error::FileStorageError> {
        tokio::fs::create_dir_all(self.root().get())
            .await
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        self.run_capability_operation(|root| {
            [
                constants_str::FILE_UPLOAD_STAGING_DIRECTORY,
                constants_str::FILE_DELETE_STAGING_DIRECTORY,
            ]
            .into_iter()
            .try_for_each(|directory_name| {
                match root.symlink_metadata(directory_name) {
                    Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
                    Ok(_metadata) => {
                        return Err(crate::file_storage_error::FileStorageError::Symlink);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        root.create_dir(directory_name).map_err(|create_error| {
                            crate::file_storage_error::FileStorageError::Io(create_error.into())
                        })?;
                        Self::ensure_capability_directory(&root, directory_name.into())?;
                    }
                    Err(error) => {
                        return Err(crate::file_storage_error::FileStorageError::Io(
                            error.into(),
                        ));
                    }
                }
                Ok(())
            })
        })
        .await
    }

    pub async fn stage_upload(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        std_file_bytes: &crate::std_file_bytes::StdFileBytes,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let operation_id = std_storage_operation_id.clone();
        let file_bytes = std_file_bytes.clone();
        self.run_capability_operation(move |root| {
            Self::ensure_capability_directory(
                &root,
                constants_str::FILE_UPLOAD_STAGING_DIRECTORY.into(),
            )?;
            let staging_path = std::path::Path::new(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
                .join(operation_id.as_ref());
            let mut options = cap_std::fs::OpenOptions::new();
            let _configured_options = options.create_new(true).write(true);
            let mut file = root.open_with(&staging_path, &options).map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    crate::file_storage_error::FileStorageError::StagingEntryExists
                } else {
                    crate::file_storage_error::FileStorageError::Io(error.into())
                }
            })?;
            let write_result = std::io::Write::write_all(&mut file, file_bytes.as_ref())
                .and_then(|()| std::io::Write::flush(&mut file))
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()));
            drop(file);
            if let Err(write_error) = write_result {
                return match root.remove_file(&staging_path) {
                    Ok(()) => Err(write_error),
                    Err(cleanup) if cleanup.kind() == std::io::ErrorKind::NotFound => {
                        Err(write_error)
                    }
                    Err(cleanup) => Err(crate::file_storage_error::FileStorageError::Io(
                        cleanup.into(),
                    )),
                };
            }
            Ok(())
        })
        .await
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
        let operation_id = std_storage_operation_id.clone();
        let relative_path = storage_relative_path_buf.clone();
        self.run_capability_operation(move |root| {
            Self::ensure_capability_destination_parent(&root, &relative_path)?;
            Self::ensure_capability_directory(
                &root,
                constants_str::FILE_DELETE_STAGING_DIRECTORY.into(),
            )?;
            let metadata = root
                .symlink_metadata(relative_path.as_ref())
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return Err(if metadata.file_type().is_symlink() {
                    crate::file_storage_error::FileStorageError::Symlink
                } else {
                    crate::file_storage_error::FileStorageError::SourceNotRegular
                });
            }
            let staged_path = std::path::Path::new(constants_str::FILE_DELETE_STAGING_DIRECTORY)
                .join(operation_id.as_ref());
            root.rename(relative_path.as_ref(), &root, staged_path)
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))
        })
        .await
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
        storage_directory_name_ref: crate::storage_directory_name_ref::StorageDirectoryNameRef<
            'static,
        >,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let operation_id = std_storage_operation_id.clone();
        let storage_directory_name = storage_directory_name_ref.get();
        self.run_capability_operation(move |root| {
            Self::ensure_capability_directory(&root, storage_directory_name.into())?;
            root.remove_file(
                std::path::Path::new(storage_directory_name).join(operation_id.as_ref()),
            )
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))
        })
        .await
    }

    async fn rename_staged(
        &self,
        std_storage_operation_id: &crate::std_storage_operation_id::StdStorageOperationId,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
        storage_directory_name_ref: crate::storage_directory_name_ref::StorageDirectoryNameRef<
            'static,
        >,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let operation_id = std_storage_operation_id.clone();
        let relative_path = storage_relative_path_buf.clone();
        let storage_directory_name = storage_directory_name_ref.get();
        self.run_capability_operation(move |root| {
            Self::ensure_capability_directory(&root, storage_directory_name.into())?;
            Self::ensure_capability_destination_parent(&root, &relative_path)?;
            match root.symlink_metadata(relative_path.as_ref()) {
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
            root.rename(
                std::path::Path::new(storage_directory_name).join(operation_id.as_ref()),
                &root,
                relative_path.as_ref(),
            )
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))
        })
        .await
    }

    fn ensure_capability_directory(
        root: &cap_std::fs::Dir,
        storage_directory_name_ref: crate::storage_directory_name_ref::StorageDirectoryNameRef<'_>,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let metadata = root
            .symlink_metadata(storage_directory_name_ref.get())
            .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            Ok(())
        } else {
            Err(crate::file_storage_error::FileStorageError::Symlink)
        }
    }

    fn ensure_capability_destination_parent(
        root: &cap_std::fs::Dir,
        storage_relative_path_buf: &crate::storage_relative_path_buf::StorageRelativePathBuf,
    ) -> Result<(), crate::file_storage_error::FileStorageError> {
        let mut current = std::path::PathBuf::new();
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
            match root.symlink_metadata(&current) {
                Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
                Ok(_metadata) => return Err(crate::file_storage_error::FileStorageError::Symlink),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    root.create_dir(&current).map_err(|create_error| {
                        crate::file_storage_error::FileStorageError::Io(create_error.into())
                    })?;
                    let metadata = root.symlink_metadata(&current).map_err(|metadata_error| {
                        crate::file_storage_error::FileStorageError::Io(metadata_error.into())
                    })?;
                    if !metadata.is_dir() || metadata.file_type().is_symlink() {
                        return Err(crate::file_storage_error::FileStorageError::Symlink);
                    }
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

    async fn run_capability_operation<T, F>(
        &self,
        operation: F,
    ) -> Result<T, crate::file_storage_error::FileStorageError>
    where
        T: Send + 'static,
        F: FnOnce(cap_std::fs::Dir) -> Result<T, crate::file_storage_error::FileStorageError>
            + Send
            + 'static,
    {
        let root_path = self.root().get().to_path_buf();
        tokio::task::spawn_blocking(move || {
            let metadata = std::fs::symlink_metadata(&root_path)
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err(crate::file_storage_error::FileStorageError::Symlink);
            }
            let root = cap_std::fs::Dir::open_ambient_dir(root_path, cap_std::ambient_authority())
                .map_err(|error| crate::file_storage_error::FileStorageError::Io(error.into()))?;
            operation(root)
        })
        .await
        .map_err(|error| {
            crate::file_storage_error::FileStorageError::Io(std::io::Error::other(error).into())
        })?
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
