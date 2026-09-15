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
