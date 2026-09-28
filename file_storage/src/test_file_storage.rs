#[tokio::test]
async fn test_stale_staging_cleanup_is_bounded_and_removes_regular_files() {
    let root_path = std::env::temp_dir().join(constants_str::TEST_STALE_STAGING_DIRECTORY);
    match tokio::fs::remove_dir_all(&root_path).await {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => std::panic::panic_any(constants_str::PANIC_E0757D39.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        )),
    }
    let storage = crate::safe_file_storage::SafeFileStorage::new(
        crate::file_storage_root_path_buf::FileStorageRootPathBuf::try_from(root_path.clone())
            .expect(constants_str::DIAGNOSTIC_0A4C0BFD),
    );
    storage
        .prepare()
        .await
        .expect(constants_str::DIAGNOSTIC_73802BD5);
    let operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(
        String::from(constants_str::TEST_STALE_STAGING_OPERATION_ID),
    )
    .expect(constants_str::DIAGNOSTIC_D374CE69);
    storage
        .stage_upload(
            &operation_id,
            &crate::std_file_bytes::StdFileBytes::try_from(vec![1u8])
                .expect(constants_str::DIAGNOSTIC_A9899D14),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_DF4E565C);
    let second_operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(
        String::from(constants_str::TEST_STALE_STAGING_SECOND_OPERATION_ID),
    )
    .expect(constants_str::DIAGNOSTIC_DE441C7A);
    storage
        .stage_upload(
            &second_operation_id,
            &crate::std_file_bytes::StdFileBytes::try_from(vec![2u8])
                .expect(constants_str::DIAGNOSTIC_941A849C),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_CE87151D);
    let stale_before = std::time::UNIX_EPOCH
        .checked_add(std::time::Duration::from_hours(1_139_568u64))
        .expect(constants_str::DIAGNOSTIC_C81A56D9);
    let limit = crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit::try_from(
        constants_usize::ONE,
    )
    .expect(constants_str::DIAGNOSTIC_C35F98C6);
    let report = storage
        .cleanup_stale_staging(
            crate::file_storage_staging_area::FileStorageStagingArea::Upload,
            crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration::new(
                stale_before.into(),
                limit,
                limit,
            ),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_EB46D89C);
    assert_eq!(
        report,
        crate::stale_staging_cleanup_report::StaleStagingCleanupReport::from((
            crate::std_stale_staging_entry_count::StdStaleStagingEntryCount::from(
                constants_usize::ONE
            ),
            crate::std_stale_staging_entry_count::StdStaleStagingEntryCount::from(
                constants_usize::ONE
            ),
        ))
    );
    let mut remaining_entries =
        tokio::fs::read_dir(root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY))
            .await
            .expect(constants_str::DIAGNOSTIC_ACDBF8DA);
    assert!(
        remaining_entries
            .next_entry()
            .await
            .expect(constants_str::DIAGNOSTIC_3C5C9B70)
            .is_some()
    );
    assert!(
        remaining_entries
            .next_entry()
            .await
            .expect(constants_str::DIAGNOSTIC_406536B7)
            .is_none()
    );
    tokio::fs::remove_dir_all(root_path)
        .await
        .expect(constants_str::DIAGNOSTIC_9CF8105C);
}

#[test]
fn test_relative_paths_reject_owned_staging_directories() {
    assert!(
        [
            constants_str::FILE_UPLOAD_STAGING_DIRECTORY,
            constants_str::FILE_DELETE_STAGING_DIRECTORY,
        ]
        .into_iter()
        .all(|directory| {
            let path = std::path::PathBuf::from(directory);
            crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(path.clone())
                == Err(crate::file_storage_path_error::FileStoragePathError::RelativePathInvalid)
                && crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
                    path.join(constants_str::TEST_FILE_STORAGE_OPERATION_ID),
                ) == Err(
                    crate::file_storage_path_error::FileStoragePathError::RelativePathInvalid,
                )
                && crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
                    std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_OLD_PATH)
                        .join(directory),
                )
                .is_ok()
        })
    );
}

#[test]
fn test_relative_paths_reject_nul_bytes() {
    let mut path_text = constants_str::TEST_DISK_CACHE_OLD_PATH.to_owned();
    path_text.push('\0');
    assert_eq!(
        crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
            std::path::PathBuf::from(path_text),
        ),
        Err(crate::file_storage_path_error::FileStoragePathError::RelativePathInvalid),
    );
}

#[test]
fn test_relative_paths_and_operation_ids_reject_traversal() {
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
fn test_storage_paths_reject_values_above_maximum_length() {
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
fn test_disk_cache_budget_evicts_oldest_entries_first() {
    let old_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_OLD_PATH),
    )
    .expect(constants_str::DIAGNOSTIC_0DC17257);
    let new_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_NEW_PATH),
    )
    .expect(constants_str::DIAGNOSTIC_38C1ECA1);
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
    .expect(constants_str::DIAGNOSTIC_1BC67951);
    assert_eq!(plan.as_ref(), &[old_path]);
}

#[test]
fn test_disk_cache_budget_evicts_when_projected_size_overflows() {
    let result = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_OLD_PATH),
    )
    .map(|old_path| {
        let entries = [crate::disk_cache_entry::DiskCacheEntry::new(
            old_path.clone(),
            u64::MAX.into(),
            std::time::UNIX_EPOCH.into(),
        )];
        crate::plan_disk_cache_eviction::plan_disk_cache_eviction(
            &entries,
            u64::MAX.into(),
            1u64.into(),
        )
        .map(|plan| (old_path, plan))
    });
    assert!(
        matches!(result, Ok(Ok((old_path, plan))) if plan.as_ref() == std::slice::from_ref(&old_path))
    );
}

#[tokio::test]
async fn test_staged_upload_delete_and_rollback_preserve_transaction_boundaries() {
    let root_path = std::env::temp_dir().join(constants_str::TEST_FILE_STORAGE_DIRECTORY);
    match tokio::fs::remove_dir_all(&root_path).await {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => std::panic::panic_any(constants_str::PANIC_A61C720D.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        )),
    }
    let storage = crate::safe_file_storage::SafeFileStorage::new(
        crate::file_storage_root_path_buf::FileStorageRootPathBuf::try_from(root_path.clone())
            .expect(constants_str::DIAGNOSTIC_EC6F4321),
    );
    storage
        .prepare()
        .await
        .expect(constants_str::DIAGNOSTIC_AB760E42);
    let operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(
        String::from(constants_str::TEST_FILE_STORAGE_OPERATION_ID),
    )
    .expect(constants_str::DIAGNOSTIC_CA3F4821);
    let relative_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::TEST_FILE_STORAGE_RELATIVE_PATH),
    )
    .expect(constants_str::DIAGNOSTIC_85ED3042);
    let bytes = crate::std_file_bytes::StdFileBytes::try_from(vec![1u8, 2u8, 3u8])
        .expect(constants_str::DIAGNOSTIC_D7DF0F1C);
    storage
        .stage_upload(&operation_id, &bytes)
        .await
        .expect(constants_str::DIAGNOSTIC_94C1083E);
    storage
        .commit_upload(&operation_id, &relative_path)
        .await
        .expect(constants_str::DIAGNOSTIC_217F53E4);
    let _metadata_after_upload = tokio::fs::metadata(root_path.join(relative_path.as_ref()))
        .await
        .expect(constants_str::DIAGNOSTIC_A28E410C);
    storage
        .stage_delete(&operation_id, &relative_path)
        .await
        .expect(constants_str::DIAGNOSTIC_40761D28);
    storage
        .rollback_delete(&operation_id, &relative_path)
        .await
        .expect(constants_str::DIAGNOSTIC_1CD05291);
    let _metadata_after_delete_rollback =
        tokio::fs::metadata(root_path.join(relative_path.as_ref()))
            .await
            .expect(constants_str::DIAGNOSTIC_3C48B27D);
    let replacement_operation_id =
        crate::std_storage_operation_id::StdStorageOperationId::try_from(String::from(
            constants_str::TEST_FILE_STORAGE_REPLACEMENT_OPERATION_ID,
        ))
        .expect(constants_str::DIAGNOSTIC_FB7E68B1);
    let replacement_bytes = crate::std_file_bytes::StdFileBytes::try_from(vec![4u8, 5u8])
        .expect(constants_str::DIAGNOSTIC_23566F2B);
    storage
        .atomic_replace(
            &replacement_operation_id,
            &relative_path,
            &replacement_bytes,
            crate::atomic_replace_durability::AtomicReplaceDurability::Flush,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_A1EA86B8);
    assert_eq!(
        tokio::fs::read(root_path.join(relative_path.as_ref()))
            .await
            .expect(constants_str::DIAGNOSTIC_571084E8),
        [4u8, 5u8],
    );
    let root_relative_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_NEW_PATH),
    )
    .expect(constants_str::DIAGNOSTIC_E474698D);
    storage
        .atomic_replace(
            &replacement_operation_id,
            &root_relative_path,
            &replacement_bytes,
            crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_29BE576B);
    assert_eq!(
        tokio::fs::read(root_path.join(root_relative_path.as_ref()))
            .await
            .expect(constants_str::DIAGNOSTIC_3CED8A3A),
        [4u8, 5u8],
    );
    storage
        .atomic_replace(
            &replacement_operation_id,
            &relative_path,
            &replacement_bytes,
            crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_1DC48FC8);
    tokio::fs::write(root_path.join(root_relative_path.as_ref()), [6u8])
        .await
        .expect(constants_str::DIAGNOSTIC_C4DC1206);
    storage
        .stage_delete(&operation_id, &relative_path)
        .await
        .expect(constants_str::DIAGNOSTIC_E12EE4FE);
    assert!(matches!(
        storage
            .stage_delete(&operation_id, &root_relative_path)
            .await,
        Err(crate::file_storage_error::FileStorageError::StagingEntryExists)
    ));
    assert!(root_path.join(root_relative_path.as_ref()).exists());
    storage
        .rollback_delete(&operation_id, &relative_path)
        .await
        .expect(constants_str::DIAGNOSTIC_1067673A);
    assert_eq!(
        tokio::fs::read(root_path.join(relative_path.as_ref()))
            .await
            .expect(constants_str::DIAGNOSTIC_23CFF317),
        [4u8, 5u8],
    );
    assert_eq!(
        tokio::fs::read(root_path.join(root_relative_path.as_ref()))
            .await
            .expect(constants_str::DIAGNOSTIC_3502EA03),
        [6u8],
    );
    let failed_relative_path = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_OLD_PATH),
    )
    .expect(constants_str::DIAGNOSTIC_49FAEC3B);
    tokio::fs::create_dir(root_path.join(failed_relative_path.as_ref()))
        .await
        .expect(constants_str::DIAGNOSTIC_5734F460);
    let failed_operation_id = crate::std_storage_operation_id::StdStorageOperationId::try_from(
        String::from(constants_str::TEST_STALE_STAGING_SECOND_OPERATION_ID),
    )
    .expect(constants_str::DIAGNOSTIC_55012796);
    assert!(
        storage
            .atomic_replace(
                &failed_operation_id,
                &failed_relative_path,
                &replacement_bytes,
                crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll,
            )
            .await
            .is_err()
    );
    assert!(
        !root_path
            .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(failed_operation_id.as_ref())
            .exists()
    );
    assert!(matches!(
        tokio::fs::remove_dir(root_path.join(failed_relative_path.as_ref())).await,
        Ok(())
    ));
    let assert_directory_rejected = async |file_storage_staging_area: crate::file_storage_staging_area::FileStorageStagingArea| {
        let staged_path = root_path
            .join(file_storage_staging_area.directory_name().get())
            .join(failed_operation_id.as_ref());
        assert!(matches!(tokio::fs::create_dir(&staged_path).await, Ok(())));
        let result = match file_storage_staging_area {
            crate::file_storage_staging_area::FileStorageStagingArea::Upload => {
                storage.commit_upload(&failed_operation_id, &failed_relative_path).await
            }
            crate::file_storage_staging_area::FileStorageStagingArea::Delete => {
                storage.rollback_delete(&failed_operation_id, &failed_relative_path).await
            }
        };
        assert!(matches!(result, Err(crate::file_storage_error::FileStorageError::SourceNotRegular)));
        assert!(staged_path.is_dir());
        assert!(!root_path.join(failed_relative_path.as_ref()).exists());
        assert!(matches!(tokio::fs::remove_dir(staged_path).await, Ok(())));
    };
    assert_directory_rejected(crate::file_storage_staging_area::FileStorageStagingArea::Upload)
        .await;
    assert_directory_rejected(crate::file_storage_staging_area::FileStorageStagingArea::Delete)
        .await;
    #[cfg(unix)]
    let () = {
        let outside_path = root_path.with_extension(constants_str::TEST_FILE_STORAGE_OPERATION_ID);
        match tokio::fs::remove_dir_all(&outside_path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => std::panic::panic_any(error),
        }
        tokio::fs::create_dir(&outside_path)
            .await
            .expect(constants_str::DIAGNOSTIC_E394BE93);
        let staged_link = root_path
            .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(failed_operation_id.as_ref());
        assert!(matches!(
            tokio::fs::symlink(&outside_path, &staged_link).await,
            Ok(())
        ));
        assert!(matches!(
            storage
                .commit_upload(&failed_operation_id, &failed_relative_path)
                .await,
            Err(crate::file_storage_error::FileStorageError::Symlink)
        ));
        assert!(staged_link.is_symlink());
        assert!(!root_path.join(failed_relative_path.as_ref()).exists());
        assert!(matches!(tokio::fs::remove_file(staged_link).await, Ok(())));
        tokio::fs::remove_dir(root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY))
            .await
            .expect(constants_str::DIAGNOSTIC_9AE571ED);
        std::os::unix::fs::symlink(
            &outside_path,
            root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY),
        )
        .expect(constants_str::DIAGNOSTIC_5574E967);
        assert!(matches!(
            storage
                .stage_upload(&failed_operation_id, &replacement_bytes)
                .await,
            Err(crate::file_storage_error::FileStorageError::Symlink)
        ));
        assert!(!outside_path.join(failed_operation_id.as_ref()).exists());
        tokio::fs::remove_dir_all(outside_path)
            .await
            .expect(constants_str::DIAGNOSTIC_7352F192);
    };
    tokio::fs::remove_dir_all(root_path)
        .await
        .expect(constants_str::DIAGNOSTIC_9A69203B);
}
