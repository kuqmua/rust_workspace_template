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
        assert!(matches!(storage.prepare().await, Ok(())));
        let deletion_staging = root.join(constants_str::FILE_DELETE_STAGING_DIRECTORY);
        assert!(matches!(
            tokio::fs::remove_dir(&deletion_staging).await,
            Ok(())
        ));
        assert!(matches!(
            tokio::fs::write(&deletion_staging, constants_str::X).await,
            Ok(())
        ));
        assert!(matches!(
            storage.prepare().await,
            Err(crate::file_storage_error::FileStorageError::Symlink)
        ));
        assert!(
            tokio::fs::metadata(&deletion_staging)
                .await
                .is_ok_and(|metadata| metadata.is_file() && metadata.len() == 1u64)
        );
        tokio::fs::remove_dir_all(root)
            .await
            .expect(constants_str::DIAGNOSTIC_1FC58E0B);
    }
}
