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
    assert!(
        storage
            .cleanup_stale_staging(
                crate::file_storage_staging_area::FileStorageStagingArea::Delete,
                crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration::new(
                    std::time::UNIX_EPOCH.into(),
                    limit,
                    limit
                ),
            )
            .await
            .is_ok_and(|cleanup_report| cleanup_report
                == crate::stale_staging_cleanup_report::StaleStagingCleanupReport::default())
    );
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
    assert!(
        storage
            .cleanup_stale_staging(
                crate::file_storage_staging_area::FileStorageStagingArea::Upload,
                crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration::new(
                    stale_before.into(),
                    limit,
                    limit
                ),
            )
            .await
            .is_ok_and(|cleanup_report| cleanup_report.scanned().get() == 1usize
                && cleanup_report.removed().get() == 1usize)
    );
    assert!(
        storage
            .cleanup_stale_staging(
                crate::file_storage_staging_area::FileStorageStagingArea::Upload,
                crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration::new(
                    stale_before.into(),
                    limit,
                    limit
                ),
            )
            .await
            .is_ok_and(|cleanup_report| cleanup_report
                == crate::stale_staging_cleanup_report::StaleStagingCleanupReport::default())
    );
    let timestamped_path = root_path
        .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
        .join(operation_id.as_ref());
    assert!(matches!(
        tokio::fs::write(&timestamped_path, [3u8]).await,
        Ok(())
    ));
    let fixture_file_result = tokio::fs::OpenOptions::new()
        .write(true)
        .open(&timestamped_path)
        .await;
    assert!(fixture_file_result.as_ref().err().is_none());
    let Ok(fixture_file) = fixture_file_result else {
        return;
    };
    let standard_fixture_file = fixture_file.into_std().await;
    let fixture_modified = std::time::UNIX_EPOCH + std::time::Duration::from_hours(24u64);
    let timestamp_task = tokio::task::spawn_blocking(move || {
        standard_fixture_file.set_times(std::fs::FileTimes::new().set_modified(fixture_modified))
    });
    assert!(
        timestamp_task
            .await
            .is_ok_and(|timestamp_result| matches!(timestamp_result, Ok(())))
    );
    assert!(
        storage
            .cleanup_stale_staging(
                crate::file_storage_staging_area::FileStorageStagingArea::Upload,
                crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration::new(
                    std::time::UNIX_EPOCH.into(),
                    limit,
                    limit
                ),
            )
            .await
            .is_ok_and(|newer_report| newer_report.scanned().get() == 1usize
                && newer_report.removed().get() == 0usize)
    );
    assert!(
        tokio::fs::read(&timestamped_path)
            .await
            .is_ok_and(|newer_bytes| newer_bytes == [3u8])
    );
    assert!(
        storage
            .cleanup_stale_staging(
                crate::file_storage_staging_area::FileStorageStagingArea::Upload,
                crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration::new(
                    fixture_modified.into(),
                    limit,
                    limit
                ),
            )
            .await
            .is_ok_and(|cutoff_report| cutoff_report.scanned().get() == 1usize
                && cutoff_report.removed().get() == 1usize)
    );
    assert!(!timestamped_path.exists());
    #[cfg(unix)]
    let () = {
        let upload_directory = root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY);
        let retained_directory = upload_directory.join(constants_str::X);
        let target_file = root_path.join(constants_str::X);
        let retained_link = upload_directory.join(constants_str::A);
        assert!(matches!(
            tokio::fs::create_dir(&retained_directory).await,
            Ok(())
        ));
        assert!(matches!(
            tokio::fs::write(&target_file, [6u8]).await,
            Ok(())
        ));
        assert!(matches!(
            tokio::fs::symlink(&target_file, &retained_link).await,
            Ok(())
        ));
        let entry_limit_result =
            crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit::try_from(2usize);
        assert!(
            entry_limit_result
                .as_ref()
                .is_ok_and(|entry_limit| entry_limit.get() == 2usize)
        );
        let Ok(two_entry_limit) = entry_limit_result else {
            return;
        };
        assert!(storage.cleanup_stale_staging(
            crate::file_storage_staging_area::FileStorageStagingArea::Upload,
            crate::stale_staging_cleanup_configuration::StaleStagingCleanupConfiguration::new(stale_before.into(), two_entry_limit, two_entry_limit),
        ).await.is_ok_and(|skipped_report| skipped_report.scanned().get() == 2usize && skipped_report.removed().get() == 0usize));
        assert!(retained_directory.is_dir());
        assert!(retained_link.is_symlink());
        assert!(
            tokio::fs::read(&target_file)
                .await
                .is_ok_and(|target_bytes| target_bytes == [6u8])
        );
    };
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
    assert!(
        [
            storage.commit_upload(&operation_id, &relative_path).await,
            storage.rollback_upload(&operation_id).await,
            storage.rollback_delete(&operation_id, &relative_path).await,
            storage.commit_delete(&operation_id).await,
            storage.stage_delete(&operation_id, &relative_path).await,
        ]
        .into_iter()
        .all(|result| matches!(
            result,
            Err(crate::file_storage_error::FileStorageError::Io(_))
        ))
    );
    assert!(!root_path.join(relative_path.as_ref()).exists());
    assert!(
        !root_path
            .join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(operation_id.as_ref())
            .exists()
    );
    assert!(
        !root_path
            .join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
            .join(operation_id.as_ref())
            .exists()
    );
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
    assert!(matches!(
        tokio::fs::write(root_path.join(relative_path.as_ref()), [9u8]).await,
        Ok(())
    ));
    assert!(matches!(
        storage.rollback_delete(&operation_id, &relative_path).await,
        Err(crate::file_storage_error::FileStorageError::DestinationExists)
    ));
    assert!(
        tokio::fs::read(root_path.join(relative_path.as_ref()))
            .await
            .is_ok_and(|destination_bytes| destination_bytes == [9u8])
    );
    assert!(
        tokio::fs::read(
            root_path
                .join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
                .join(operation_id.as_ref())
        )
        .await
        .is_ok_and(|staged_bytes| staged_bytes == [1u8, 2u8, 3u8])
    );
    assert!(matches!(
        tokio::fs::remove_file(root_path.join(relative_path.as_ref())).await,
        Ok(())
    ));
    #[cfg(unix)]
    let () = {
        let staged_original = root_path
            .join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
            .join(operation_id.as_ref());
        let destination_link = root_path.join(relative_path.as_ref());
        assert!(matches!(
            tokio::fs::symlink(&staged_original, &destination_link).await,
            Ok(())
        ));
        assert!(matches!(
            storage.rollback_delete(&operation_id, &relative_path).await,
            Err(crate::file_storage_error::FileStorageError::Symlink)
        ));
        assert!(destination_link.is_symlink());
        assert!(
            tokio::fs::read(&staged_original)
                .await
                .is_ok_and(|original_bytes| original_bytes == [1u8, 2u8, 3u8])
        );
        assert!(matches!(
            tokio::fs::remove_file(&destination_link).await,
            Ok(())
        ));
    };
    let blocked_parent = root_path
        .join(relative_path.as_ref())
        .with_file_name(constants_str::EMPTY)
        .components()
        .collect::<std::path::PathBuf>();
    assert!(matches!(
        tokio::fs::remove_dir(&blocked_parent).await,
        Ok(())
    ));
    assert!(matches!(
        tokio::fs::write(&blocked_parent, [8u8]).await,
        Ok(())
    ));
    assert!(matches!(
        storage.rollback_delete(&operation_id, &relative_path).await,
        Err(crate::file_storage_error::FileStorageError::Symlink)
    ));
    assert!(
        tokio::fs::read(&blocked_parent)
            .await
            .is_ok_and(|parent_bytes| parent_bytes == [8u8])
    );
    assert!(
        tokio::fs::read(
            root_path
                .join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
                .join(operation_id.as_ref())
        )
        .await
        .is_ok_and(|retained_bytes| retained_bytes == [1u8, 2u8, 3u8])
    );
    assert!(matches!(
        tokio::fs::remove_file(&blocked_parent).await,
        Ok(())
    ));
    assert!(matches!(
        tokio::fs::create_dir(&blocked_parent).await,
        Ok(())
    ));
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
    let assert_delete_reservation_absent = async || {
        assert!(
            tokio::fs::symlink_metadata(
                root_path
                    .join(constants_str::FILE_DELETE_STAGING_DIRECTORY)
                    .join(failed_operation_id.as_ref()),
            )
            .await
            .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
        );
    };
    assert!(matches!(
        storage
            .stage_delete(&failed_operation_id, &failed_relative_path)
            .await,
        Err(crate::file_storage_error::FileStorageError::SourceNotRegular)
    ));
    assert_delete_reservation_absent().await;
    assert!(
        tokio::fs::symlink_metadata(root_path.join(failed_relative_path.as_ref()))
            .await
            .is_ok_and(|metadata| metadata.is_dir())
    );
    let assert_failed_replace = async |atomic_replace_durability: crate::atomic_replace_durability::AtomicReplaceDurability| {
        assert!(matches!(storage.atomic_replace(&failed_operation_id, &failed_relative_path, &replacement_bytes, atomic_replace_durability).await, Err(crate::file_storage_error::FileStorageError::SourceNotRegular)));
        assert!(!root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY).join(failed_operation_id.as_ref()).exists());
        assert!(root_path.join(failed_relative_path.as_ref()).is_dir());
    };
    assert_failed_replace(crate::atomic_replace_durability::AtomicReplaceDurability::Flush).await;
    assert_failed_replace(crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll).await;
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
        let outside_file = outside_path.join(constants_str::TEST_DISK_CACHE_OLD_PATH);
        assert!(matches!(
            tokio::fs::write(&outside_file, [9u8]).await,
            Ok(())
        ));
        let destination_link = root_path.join(failed_relative_path.as_ref());
        assert!(matches!(
            tokio::fs::symlink(&outside_file, &destination_link).await,
            Ok(())
        ));
        assert!(matches!(
            storage
                .stage_delete(&failed_operation_id, &failed_relative_path)
                .await,
            Err(crate::file_storage_error::FileStorageError::Symlink)
        ));
        assert_delete_reservation_absent().await;
        assert!(
            tokio::fs::symlink_metadata(&destination_link)
                .await
                .is_ok_and(|metadata| metadata.file_type().is_symlink())
        );
        assert!(
            tokio::fs::read(&outside_file)
                .await
                .is_ok_and(|file_bytes| file_bytes == [9u8])
        );
        let assert_staged_destination_symlink_rejected = async |file_storage_staging_area: crate::file_storage_staging_area::FileStorageStagingArea| {
            let staged_path = root_path.join(file_storage_staging_area.directory_name().get())
                .join(failed_operation_id.as_ref());
            match file_storage_staging_area {
                crate::file_storage_staging_area::FileStorageStagingArea::Upload => {
                    assert!(matches!(storage.stage_upload(&failed_operation_id, &replacement_bytes).await, Ok(())));
                }
                crate::file_storage_staging_area::FileStorageStagingArea::Delete => {
                    assert!(matches!(tokio::fs::write(&staged_path, replacement_bytes.as_ref()).await, Ok(())));
                }
            }
            let result = match file_storage_staging_area {
                crate::file_storage_staging_area::FileStorageStagingArea::Upload => {
                    storage.commit_upload(&failed_operation_id, &failed_relative_path).await
                }
                crate::file_storage_staging_area::FileStorageStagingArea::Delete => {
                    storage.rollback_delete(&failed_operation_id, &failed_relative_path).await
                }
            };
            assert!(matches!(result, Err(crate::file_storage_error::FileStorageError::Symlink)));
            assert!(tokio::fs::read(&staged_path).await.is_ok_and(|file_bytes| file_bytes.as_slice() == replacement_bytes.as_ref()));
            assert!(tokio::fs::symlink_metadata(&destination_link).await.is_ok_and(|metadata| metadata.file_type().is_symlink()));
            assert!(tokio::fs::read(&outside_file).await.is_ok_and(|file_bytes| file_bytes == [9u8]));
            let cleanup = match file_storage_staging_area {
                crate::file_storage_staging_area::FileStorageStagingArea::Upload => storage.rollback_upload(&failed_operation_id).await,
                crate::file_storage_staging_area::FileStorageStagingArea::Delete => storage.commit_delete(&failed_operation_id).await,
            };
            assert!(matches!(cleanup, Ok(())));
            assert!(tokio::fs::symlink_metadata(&staged_path).await.is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound));
        };
        assert_staged_destination_symlink_rejected(
            crate::file_storage_staging_area::FileStorageStagingArea::Upload,
        )
        .await;
        assert_staged_destination_symlink_rejected(
            crate::file_storage_staging_area::FileStorageStagingArea::Delete,
        )
        .await;
        let assert_destination_symlink_rejected = async |atomic_replace_durability: crate::atomic_replace_durability::AtomicReplaceDurability| {
            assert!(matches!(
                storage.atomic_replace(&failed_operation_id, &failed_relative_path, &replacement_bytes, atomic_replace_durability).await,
                Err(crate::file_storage_error::FileStorageError::Symlink)
            ));
            assert!(tokio::fs::symlink_metadata(&destination_link).await.is_ok_and(|metadata| metadata.file_type().is_symlink()));
            assert!(tokio::fs::symlink_metadata(root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY).join(failed_operation_id.as_ref())).await.is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound));
            assert!(tokio::fs::read(&outside_file).await.is_ok_and(|file_bytes| file_bytes == [9u8]));
        };
        assert_destination_symlink_rejected(
            crate::atomic_replace_durability::AtomicReplaceDurability::Flush,
        )
        .await;
        assert_destination_symlink_rejected(
            crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll,
        )
        .await;
        assert!(matches!(
            tokio::fs::remove_file(&destination_link).await,
            Ok(())
        ));
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
        assert!(matches!(
            storage.prepare().await,
            Err(crate::file_storage_error::FileStorageError::Symlink)
        ));
        assert!(!outside_path.join(failed_operation_id.as_ref()).exists());
        tokio::fs::remove_dir_all(outside_path)
            .await
            .expect(constants_str::DIAGNOSTIC_7352F192);
    };
    let displaced_root =
        root_path.with_extension(constants_str::TEST_FILE_STORAGE_REPLACEMENT_OPERATION_ID);
    match tokio::fs::remove_dir_all(&displaced_root).await {
        Ok(()) => {}
        Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
    }
    assert!(matches!(
        tokio::fs::rename(&root_path, &displaced_root).await,
        Ok(())
    ));
    assert!(
        [
            storage.stage_upload(&operation_id, &bytes).await,
            storage.commit_upload(&operation_id, &relative_path).await,
            storage.rollback_upload(&operation_id).await,
            storage.stage_delete(&operation_id, &relative_path).await,
            storage.rollback_delete(&operation_id, &relative_path).await,
            storage.commit_delete(&operation_id).await,
            storage
                .atomic_replace(
                    &operation_id,
                    &relative_path,
                    &bytes,
                    crate::atomic_replace_durability::AtomicReplaceDurability::Flush,
                )
                .await,
            storage
                .atomic_replace(
                    &operation_id,
                    &relative_path,
                    &bytes,
                    crate::atomic_replace_durability::AtomicReplaceDurability::SyncAll,
                )
                .await,
        ]
        .into_iter()
        .all(|result| result.is_err_and(|error| {
            matches!(&error, crate::file_storage_error::FileStorageError::Io(_))
                && std::error::Error::source(&error).is_some()
        }))
    );
    assert!(
        tokio::fs::symlink_metadata(&root_path)
            .await
            .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
    );
    assert!(
        tokio::fs::read(displaced_root.join(relative_path.as_ref()))
            .await
            .is_ok_and(|preserved_bytes| preserved_bytes == [4u8, 5u8])
    );
    assert!(matches!(tokio::fs::write(&root_path, [7u8]).await, Ok(())));
    assert!(matches!(
        storage.rollback_upload(&operation_id).await,
        Err(crate::file_storage_error::FileStorageError::Symlink)
    ));
    assert!(
        tokio::fs::read(&root_path)
            .await
            .is_ok_and(|root_bytes| root_bytes == [7u8])
    );
    assert!(
        tokio::fs::read(displaced_root.join(relative_path.as_ref()))
            .await
            .is_ok_and(|displaced_bytes| displaced_bytes == [4u8, 5u8])
    );
    assert!(matches!(tokio::fs::remove_file(&root_path).await, Ok(())));
    #[cfg(unix)]
    let () = {
        assert!(matches!(
            tokio::fs::symlink(&displaced_root, &root_path).await,
            Ok(())
        ));
        assert!(matches!(
            storage.rollback_upload(&operation_id).await,
            Err(crate::file_storage_error::FileStorageError::Symlink)
        ));
        assert!(root_path.is_symlink());
        assert!(
            tokio::fs::read(displaced_root.join(relative_path.as_ref()))
                .await
                .is_ok_and(|preserved_bytes| preserved_bytes == [4u8, 5u8])
        );
        assert!(matches!(tokio::fs::remove_file(&root_path).await, Ok(())));
    };
    assert!(matches!(
        tokio::fs::rename(&displaced_root, &root_path).await,
        Ok(())
    ));
    tokio::fs::remove_dir_all(root_path)
        .await
        .expect(constants_str::DIAGNOSTIC_9A69203B);
}

#[test]
fn test_disk_cache_budget_retains_entries_when_budget_is_sufficient() {
    assert!(
        crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
            std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_OLD_PATH),
        )
        .is_ok_and(|storage_relative_path_buf| {
            let entries = [crate::disk_cache_entry::DiskCacheEntry::new(
                storage_relative_path_buf,
                8u64.into(),
                std::time::UNIX_EPOCH.into(),
            )];
            [12u64, 16u64].into_iter().all(|maximum| {
                crate::plan_disk_cache_eviction::plan_disk_cache_eviction(
                    &entries,
                    maximum.into(),
                    4u64.into(),
                )
                .is_ok_and(|disk_cache_eviction_plan| disk_cache_eviction_plan.as_ref().is_empty())
            })
        })
    );
}

#[test]
fn test_disk_cache_budget_rejects_existing_overflow_and_prioritizes_incoming_limit() {
    let storage_path_result = crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
        std::path::PathBuf::from(constants_str::TEST_DISK_CACHE_OLD_PATH),
    );
    assert!(storage_path_result.is_ok());
    let Ok(storage_path) = storage_path_result else {
        return;
    };
    let entries = [
        crate::disk_cache_entry::DiskCacheEntry::new(
            storage_path.clone(),
            u64::MAX.into(),
            std::time::UNIX_EPOCH.into(),
        ),
        crate::disk_cache_entry::DiskCacheEntry::new(
            storage_path,
            1u64.into(),
            std::time::UNIX_EPOCH.into(),
        ),
    ];
    assert!(matches!(
        crate::plan_disk_cache_eviction::plan_disk_cache_eviction(
            &entries,
            u64::MAX.into(),
            0u64.into()
        ),
        Err(crate::disk_cache_budget_error::DiskCacheBudgetError::SizeOverflow)
    ));
    assert!(matches!(
        crate::plan_disk_cache_eviction::plan_disk_cache_eviction(
            &entries,
            1u64.into(),
            2u64.into()
        ),
        Err(crate::disk_cache_budget_error::DiskCacheBudgetError::IncomingTooLarge)
    ));
    assert!(matches!(
        crate::plan_disk_cache_eviction::plan_disk_cache_eviction(&[], 0u64.into(), 1u64.into()),
        Err(crate::disk_cache_budget_error::DiskCacheBudgetError::IncomingTooLarge)
    ));
}

#[test]
fn test_disk_cache_equal_timestamps_preserve_input_order_and_skip_zero_size_budget() {
    let path_results = [
        constants_str::TEST_DISK_CACHE_OLD_PATH,
        constants_str::TEST_DISK_CACHE_NEW_PATH,
    ]
    .map(|path| {
        crate::storage_relative_path_buf::StorageRelativePathBuf::try_from(
            std::path::PathBuf::from(path),
        )
    });
    assert!(path_results.iter().all(Result::is_ok));
    let [Ok(old_path), Ok(new_path)] = path_results else {
        return;
    };
    let entries = [
        crate::disk_cache_entry::DiskCacheEntry::new(
            old_path.clone(),
            0u64.into(),
            std::time::UNIX_EPOCH.into(),
        ),
        crate::disk_cache_entry::DiskCacheEntry::new(
            new_path.clone(),
            4u64.into(),
            std::time::UNIX_EPOCH.into(),
        ),
        crate::disk_cache_entry::DiskCacheEntry::new(
            old_path.clone(),
            4u64.into(),
            std::time::UNIX_EPOCH.into(),
        ),
    ];
    assert!(
        crate::plan_disk_cache_eviction::plan_disk_cache_eviction(
            &entries,
            8u64.into(),
            4u64.into()
        )
        .is_ok_and(|plan| plan.as_ref() == [old_path, new_path])
    );
    assert!(
        crate::plan_disk_cache_eviction::plan_disk_cache_eviction(&[], 0u64.into(), 0u64.into())
            .is_ok_and(|plan| plan.as_ref().is_empty())
    );
}

#[test]
fn test_stale_staging_entry_limit_accepts_exact_bounds_and_rejects_outside_values() {
    assert!([1usize, 10_000usize].into_iter().all(|value| {
        crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit::try_from(value)
            .is_ok_and(|limit| limit.get() == value)
    }));
    assert!([0usize, 10_001usize].into_iter().all(|value| matches!(crate::std_stale_staging_entry_limit::StdStaleStagingEntryLimit::try_from(value), Err(crate::stale_staging_cleanup_configuration_error::StaleStagingCleanupConfigurationError::InvalidLimit))));
}

#[test]
fn test_storage_operation_identifiers_preserve_exact_length_and_reject_invalid_tokens() {
    assert!(
        [
            constants_str::X.to_owned(),
            constants_str::TEST_FILE_STORAGE_OPERATION_ID.to_owned(),
            constants_str::X.repeat(crate::domain_types::MAXIMUM_OPERATION_ID_BYTES),
        ]
        .into_iter()
        .all(|text| {
            crate::std_storage_operation_id::StdStorageOperationId::try_from(text.clone())
                .is_ok_and(|identifier| identifier.as_ref() == text)
        })
    );
    assert!(
        [
            constants_str::EMPTY.to_owned(),
            constants_str::SPACE.to_owned(),
            constants_str::SLASH.to_owned(),
            constants_str::TEST_PATH_TRAVERSAL.to_owned(),
            constants_str::X
                .repeat(crate::domain_types::MAXIMUM_OPERATION_ID_BYTES.saturating_add(1usize)),
        ]
        .into_iter()
        .all(|text| matches!(
            crate::std_storage_operation_id::StdStorageOperationId::try_from(text),
            Err(crate::file_storage_path_error::FileStoragePathError::OperationIdInvalid)
        ))
    );
}

#[test]
fn test_storage_root_paths_reject_empty_and_relative_paths() {
    assert!(
        [
            constants_str::EMPTY,
            constants_str::TEST_FILE_STORAGE_RELATIVE_PATH,
            constants_str::TEST_PATH_TRAVERSAL
        ]
        .into_iter()
        .all(|text| {
            matches!(
                crate::file_storage_root_path_buf::FileStorageRootPathBuf::try_from(
                    std::path::PathBuf::from(text)
                ),
                Err(crate::file_storage_path_error::FileStoragePathError::RootMustBeAbsolute)
            )
        })
    );
}

#[test]
fn test_file_bytes_preserve_empty_and_exact_limit_and_reject_oversized_payloads() {
    let maximum = crate::domain_types::MAXIMUM_FILE_BYTES;
    assert!(
        [0usize, maximum, maximum + 1usize]
            .into_iter()
            .all(|length| {
                match crate::std_file_bytes::StdFileBytes::try_from(vec![7u8; length]) {
                    Ok(std_file_bytes) => {
                        length <= maximum
                            && std_file_bytes.as_ref().len() == length
                            && std_file_bytes.as_ref().first().copied()
                                == (length > 0usize).then_some(7u8)
                            && std_file_bytes.as_ref().last().copied()
                                == (length > 0usize).then_some(7u8)
                    }
                    Err(file_storage_path_error) => {
                        length > maximum
                            && matches!(
                                file_storage_path_error,
                                crate::file_storage_path_error::FileStoragePathError::FileTooLarge
                            )
                    }
                }
            })
    );
}
