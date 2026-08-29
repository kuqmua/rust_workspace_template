#[tokio::test]
async fn stale_staging_cleanup_is_bounded_and_removes_regular_files() {
    let root_path = std::env::temp_dir().join(constants_str::catalog::TEST_STALE_STAGING_DIRECTORY);
    match tokio::fs::remove_dir_all(&root_path).await {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("e0757d39 {error}"),
    }
    let storage = crate::safe_file_storage::SafeFileStorage::new(
        crate::file_storage_root_path_buf::FileStorageRootPathBuf::try_from(root_path.clone()).expect("0a4c0bfd stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold"),
    );
    storage.prepare().await.expect(
        "73802bd5 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold",
    );
    let operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(
        String::from(constants_str::catalog::TEST_STALE_STAGING_OPERATION_ID),
    )
    .expect(
        "d374ce69 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold",
    );
    storage
        .stage_upload(
            &operation_id,
            &crate::std_file_bytes::StdFileBytes::try_from(vec![1u8]).expect("a9899d14 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold"),
        )
        .await
        .expect("df4e565c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
    let second_operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(
        String::from(constants_str::catalog::TEST_STALE_STAGING_SECOND_OPERATION_ID),
    )
    .expect(
        "de441c7a stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold",
    );
    storage
        .stage_upload(
            &second_operation_id,
            &crate::std_file_bytes::StdFileBytes::try_from(vec![2u8]).expect("941a849c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold"),
        )
        .await
        .expect("ce87151d stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
    let stale_before = std::time::UNIX_EPOCH
        .checked_add(std::time::Duration::from_hours(1_139_568u64))
        .expect("c81a56d9 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
    let limit = crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit::try_from(
        constants_usize::ONE,
    )
    .expect(
        "c35f98c6 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold",
    );
    let report = storage
        .cleanup_stale_staging(
            crate::file_storage_staging_area::FileStorageStagingArea::Upload,
            crate::stale_staging_cleanup_cfg::StaleStagingCleanupCfg::new(stale_before.into(), limit, limit),
        )
        .await
        .expect("eb46d89c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
    assert_eq!(
        report,
        crate::stale_staging_cleanup_report::StaleStagingCleanupReport {
            removed: crate::std_stale_staging_entry_count::StdStaleStagingEntryCount::from(
                constants_usize::ONE
            ),
            scanned: crate::std_stale_staging_entry_count::StdStaleStagingEntryCount::from(
                constants_usize::ONE
            ),
        }
    );
    let mut remaining_entries = tokio::fs::read_dir(
        root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY),
    )
    .await
    .expect(
        "acdbf8da stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold",
    );
    assert!(
        remaining_entries
            .next_entry()
            .await
            .expect("3c5c9b70 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold")
            .is_some()
    );
    assert!(
        remaining_entries
            .next_entry()
            .await
            .expect("406536b7 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold")
            .is_none()
    );
    tokio::fs::remove_dir_all(root_path).await.expect(
        "9cf8105c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold",
    );
}

#[test]
fn relative_paths_and_operation_ids_reject_traversal() {
    assert_eq!(
        crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
            std::path::PathBuf::from(constants_str::TEST_PATH_TRAVERSAL)
        ),
        Err(crate::file_storage_path_error::FileStoragePathError::RelativePathInvalid),
    );
    assert_eq!(
        crate::std_storage_operation_id::StdStorageOperationId::try_from(String::from(
            constants_str::TEST_PATH_TRAVERSAL,
        )),
        Err(crate::file_storage_path_error::FileStoragePathError::OperationIdInvalid),
    );
}

#[test]
fn storage_paths_reject_values_above_maximum_length() {
    let relative = constants_str::TEST_JWT_SECRET_CHARACTER_A
        .repeat(crate::domain_types::MAXIMUM_PATH_BYTES.saturating_add(constants_usize::ONE));
    let mut absolute = std::path::MAIN_SEPARATOR.to_string();
    absolute.push_str(relative.as_str());
    assert_eq!(
        crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
            std::path::PathBuf::from(relative)
        ),
        Err(crate::file_storage_path_error::FileStoragePathError::PathTooLong)
    );
    assert_eq!(
        crate::file_storage_root_path_buf::FileStorageRootPathBuf::try_from(
            std::path::PathBuf::from(absolute)
        ),
        Err(crate::file_storage_path_error::FileStoragePathError::PathTooLong)
    );
}

#[test]
fn disk_cache_budget_evicts_oldest_entries_first() {
    let old_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::test_fixtures::TEST_DISK_CACHE_OLD_PATH),
    )
    .expect("0dc17257 disk_cache_budget_evicts_oldest_entries_first invariant must hold");
    let new_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::test_fixtures::TEST_DISK_CACHE_NEW_PATH),
    )
    .expect("38c1eca1 disk_cache_budget_evicts_oldest_entries_first invariant must hold");
    let entries = [
        crate::disk_cache_entry::DiskCacheEntry::new(
            old_path.clone(),
            4u64.into(),
            std::time::UNIX_EPOCH.into(),
        ),
        crate::disk_cache_entry::DiskCacheEntry::new(
            new_path,
            4u64.into(),
            (std::time::UNIX_EPOCH + std::time::Duration::from_secs(1u64)).into(),
        ),
    ];
    let plan = crate::plan_disk_cache_eviction::plan_disk_cache_eviction(
        &entries,
        10u64.into(),
        4u64.into(),
    )
    .expect("1bc67951 disk_cache_budget_evicts_oldest_entries_first invariant must hold");
    assert_eq!(plan.as_ref(), &[old_path]);
}

#[tokio::test]
async fn staged_upload_delete_and_rollback_preserve_transaction_boundaries() {
    let root_path =
        std::env::temp_dir().join(constants_str::test_fixtures::TEST_FILE_STORAGE_DIRECTORY);
    match tokio::fs::remove_dir_all(&root_path).await {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("a61c720d {error}"),
    }
    let storage = crate::safe_file_storage::SafeFileStorage::new(
        crate::file_storage_root_path_buf::FileStorageRootPathBuf::try_from(root_path.clone()).expect("ec6f4321 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold"),
    );
    storage.prepare().await.expect("ab760e42 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    let operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(String::from(
        constants_str::test_fixtures::TEST_FILE_STORAGE_OPERATION_ID,
    ))
    .expect("ca3f4821 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    let relative_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(std::path::PathBuf::from(
        constants_str::test_fixtures::TEST_FILE_STORAGE_RELATIVE_PATH,
    ))
    .expect("85ed3042 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    let bytes = crate::std_file_bytes::StdFileBytes::try_from(vec![1u8, 2u8, 3u8]).expect("d7df0f1c staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    storage
        .stage_upload(&operation_id, &bytes)
        .await
        .expect("94c1083e staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    storage
        .commit_upload(&operation_id, &relative_path)
        .await
        .expect("217f53e4 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    let _metadata_after_upload = tokio::fs::metadata(root_path.join(&relative_path.0))
        .await
        .expect("a28e410c staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    storage
        .stage_delete(&operation_id, &relative_path)
        .await
        .expect("40761d28 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    storage
        .rollback_delete(&operation_id, &relative_path)
        .await
        .expect("1cd05291 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    let _metadata_after_delete_rollback = tokio::fs::metadata(root_path.join(&relative_path.0))
        .await
        .expect("3c48b27d staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    let replacement_operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(String::from(
        constants_str::test_fixtures::TEST_FILE_STORAGE_REPLACEMENT_OPERATION_ID,
    ))
    .expect("fb7e68b1 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    let replacement_bytes = crate::std_file_bytes::StdFileBytes::try_from(vec![4u8, 5u8]).expect("23566f2b staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    storage
        .atomic_replace(
            &replacement_operation_id,
            &relative_path,
            &replacement_bytes,
            crate::atomic_replace_durability::AtomicReplaceDurability::Flush,
        )
        .await
        .expect("a1ea86b8 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
    assert_eq!(
        tokio::fs::read(root_path.join(&relative_path.0))
            .await
            .expect("571084e8 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold"),
        vec![4u8, 5u8],
    );
    tokio::fs::remove_dir_all(root_path)
        .await
        .expect("9a69203b staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
}
