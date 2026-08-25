const MAXIMUM_FILE_BYTES: usize = 104_857_600usize;
const MAXIMUM_OPERATION_ID_BYTES: usize = 128usize;
const MAXIMUM_PATH_BYTES: usize = 4_096usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct FileStorageIoError(std::io::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct StoragePathRef<'value_lt>(&'value_lt std::path::Path);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct StorageDirectoryNameRef<'value_lt>(&'value_lt str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct FileStorageRootPathBuf(std::path::PathBuf);
impl TryFrom<std::path::PathBuf> for FileStorageRootPathBuf {
    type Error = FileStoragePathError;
    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        if value.as_os_str().as_encoded_bytes().len() > MAXIMUM_PATH_BYTES {
            return Err(FileStoragePathError::PathTooLong);
        }
        if value.is_absolute() {
            Ok(Self(value))
        } else {
            Err(FileStoragePathError::RootMustBeAbsolute)
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct StorageRelativePathBuf(std::path::PathBuf);
impl TryFrom<std::path::PathBuf> for StorageRelativePathBuf {
    type Error = FileStoragePathError;
    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        if value.as_os_str().as_encoded_bytes().len() > MAXIMUM_PATH_BYTES {
            return Err(FileStoragePathError::PathTooLong);
        }
        let valid = !value.as_os_str().is_empty()
            && value
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_)));
        if valid {
            Ok(Self(value))
        } else {
            Err(FileStoragePathError::RelativePathInvalid)
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct StdStorageOperationId(String);
impl TryFrom<String> for StdStorageOperationId {
    type Error = FileStoragePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > MAXIMUM_OPERATION_ID_BYTES {
            return Err(FileStoragePathError::OperationIdInvalid);
        }
        text_policy::domain_types::validate_url_safe_token_part(
            text_policy::domain_types::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::domain_types::UrlSafeTokenPartMaximumBytes::from(
                MAXIMUM_OPERATION_ID_BYTES,
            ),
        )
        .map_err(|_error| FileStoragePathError::OperationIdInvalid)?;
        Ok(Self(value))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct StdFileBytes(bounded_types::domain_types::vector::BoundedVec<u8, 0, MAXIMUM_FILE_BYTES>);
impl TryFrom<Vec<u8>> for StdFileBytes {
    type Error = FileStoragePathError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match bounded_types::domain_types::vector::BoundedVec::try_from(value) {
            Ok(bounded) => Ok(Self(bounded)),
            Err(_error) => Err(FileStoragePathError::FileTooLarge),
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum FileStoragePathError {
    #[error("{}", constants_str::FILE_STORAGE_FILE_TOO_LARGE)]
    FileTooLarge,
    #[error("{}", constants_str::FILE_STORAGE_OPERATION_ID_INVALID)]
    OperationIdInvalid,
    #[error("{}", constants_str::FILE_STORAGE_PATH_TOO_LONG)]
    PathTooLong,
    #[error("{}", constants_str::FILE_STORAGE_RELATIVE_PATH_INVALID)]
    RelativePathInvalid,
    #[error("{}", constants_str::FILE_STORAGE_ROOT_MUST_BE_ABSOLUTE)]
    RootMustBeAbsolute,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum FileStorageError {
    #[error("{}", constants_str::FILE_STORAGE_ATOMIC_REPLACE_AND_CLEANUP_ERROR)]
    AtomicReplaceAndCleanup {
        cleanup: FileStorageIoError,
        replace: FileStorageIoError,
    },
    #[error("{}", constants_str::FILE_STORAGE_DESTINATION_EXISTS)]
    DestinationExists,
    #[error("{}", constants_str::FILE_STORAGE_IO_ERROR)]
    Io(#[source] FileStorageIoError),
    #[error("{}", constants_str::FILE_STORAGE_SOURCE_NOT_REGULAR)]
    SourceNotRegular,
    #[error("{}", constants_str::FILE_STORAGE_STAGING_ENTRY_EXISTS)]
    StagingEntryExists,
    #[error("{}", constants_str::FILE_STORAGE_PATH_IS_SYMLINK)]
    Symlink,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SafeFileStorage {
    root: FileStorageRootPathBuf,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileStorageStagingArea {
    Delete,
    Upload,
}
impl FileStorageStagingArea {
    pub(crate) fn directory_name(self) -> StorageDirectoryNameRef<'static> {
        match self {
            Self::Delete => {
                StorageDirectoryNameRef::from(constants_str::FILE_DELETE_STAGING_DIRECTORY)
            }
            Self::Upload => {
                StorageDirectoryNameRef::from(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
            }
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::GetInner,
)]
pub struct StdStaleStagingEntryLimit(usize);
impl TryFrom<usize> for StdStaleStagingEntryLimit {
    type Error = StaleStagingCleanupCfgError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == constants_usize::ZERO || value > 10_000usize {
            Err(StaleStagingCleanupCfgError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct StaleBeforeSystemTime(std::time::SystemTime);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaleStagingCleanupCfg {
    maximum_removed: StdStaleStagingEntryLimit,
    maximum_scanned: StdStaleStagingEntryLimit,
    stale_before: StaleBeforeSystemTime,
}
impl StaleStagingCleanupCfg {
    pub(crate) const fn maximum_removed(self) -> StdStaleStagingEntryLimit {
        self.maximum_removed
    }

    pub(crate) const fn maximum_scanned(self) -> StdStaleStagingEntryLimit {
        self.maximum_scanned
    }

    #[must_use]
    pub const fn new(
        stale_before: StaleBeforeSystemTime,
        maximum_scanned: StdStaleStagingEntryLimit,
        maximum_removed: StdStaleStagingEntryLimit,
    ) -> Self {
        Self {
            maximum_removed,
            maximum_scanned,
            stale_before,
        }
    }

    pub(crate) const fn stale_before(self) -> StaleBeforeSystemTime {
        self.stale_before
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("stale staging cleanup limit must be between 1 and 10000")]
pub struct StaleStagingCleanupCfgError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
    newtype::IntoInnerFrom,
    newtype::Display,
)]
pub struct StdStaleStagingEntryCount(usize);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, Eq, PartialEq,
)]
pub struct StaleStagingCleanupReport {
    removed: StdStaleStagingEntryCount,
    scanned: StdStaleStagingEntryCount,
}
impl StaleStagingCleanupReport {
    pub(crate) const fn record_removed(&mut self) {
        self.removed.0 = self.removed.0.saturating_add(constants_usize::ONE);
    }

    pub(crate) const fn record_scanned(&mut self) {
        self.scanned.0 = self.scanned.0.saturating_add(constants_usize::ONE);
    }

    #[must_use]
    pub const fn removed(self) -> StdStaleStagingEntryCount {
        self.removed
    }
    #[must_use]
    pub const fn scanned(self) -> StdStaleStagingEntryCount {
        self.scanned
    }
}

#[allow(
    clippy::multiple_inherent_impl,
    reason = "domain constructor and path access stay separate from filesystem adapter operations"
)]
impl SafeFileStorage {
    #[must_use]
    pub const fn new(root: FileStorageRootPathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn root(&self) -> StoragePathRef<'_> {
        StoragePathRef::from(self.root.as_ref())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum AtomicReplaceDurability {
    Flush,
    SyncAll,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct StdDiskCacheSize(u64);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct DiskCacheModifiedAtSystemTime(std::time::SystemTime);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DiskCacheEntry {
    modified_at: DiskCacheModifiedAtSystemTime,
    path: StorageRelativePathBuf,
    size: StdDiskCacheSize,
}
impl DiskCacheEntry {
    #[must_use]
    pub const fn new(
        path: StorageRelativePathBuf,
        size: StdDiskCacheSize,
        modified_at: DiskCacheModifiedAtSystemTime,
    ) -> Self {
        Self {
            modified_at,
            path,
            size,
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct DiskCacheEvictionPlan(
    bounded_types::domain_types::vector::BoundedVec<StorageRelativePathBuf, 0, { usize::MAX }>,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum DiskCacheBudgetError {
    #[error("incoming cache entry exceeds the cache budget")]
    IncomingTooLarge,
    #[error("cache size calculation overflowed")]
    SizeOverflow,
}

pub fn plan_disk_cache_eviction(
    entries: &[DiskCacheEntry],
    maximum: StdDiskCacheSize,
    incoming: StdDiskCacheSize,
) -> Result<DiskCacheEvictionPlan, DiskCacheBudgetError> {
    if incoming.0 > maximum.0 {
        return Err(DiskCacheBudgetError::IncomingTooLarge);
    }
    let mut current = entries
        .iter()
        .try_fold(constants_u64::ZERO, |total, entry| {
            total
                .checked_add(entry.size.0)
                .ok_or(DiskCacheBudgetError::SizeOverflow)
        })?;
    let mut ordered = entries.iter().collect::<Vec<_>>();
    ordered.sort_unstable_by_key(|entry| entry.modified_at.0);
    let projected = current
        .checked_add(incoming.0)
        .ok_or(DiskCacheBudgetError::SizeOverflow)?;
    let required = projected.saturating_sub(maximum.0);
    let remove_capacity = ordered
        .iter()
        .scan(constants_u64::ZERO, |removed, entry| {
            if *removed >= required {
                None
            } else {
                *removed = removed.saturating_add(entry.size.0);
                Some(())
            }
        })
        .count();
    let mut remove = Vec::with_capacity(remove_capacity);
    let mut candidates = ordered.into_iter();
    while current
        .checked_add(incoming.0)
        .ok_or(DiskCacheBudgetError::SizeOverflow)?
        > maximum.0
    {
        let Some(entry) = candidates.next() else {
            break;
        };
        current = current.saturating_sub(entry.size.0);
        remove.push(entry.path.clone());
    }
    Ok(DiskCacheEvictionPlan::from(
        bounded_types::domain_types::vector::BoundedVec::from_max_iter(remove),
    ))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn stale_staging_cleanup_is_bounded_and_removes_regular_files() {
        let root_path = std::env::temp_dir().join(constants_str::TEST_STALE_STAGING_DIRECTORY);
        match tokio::fs::remove_dir_all(&root_path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("e0757d39 {error}"),
        }
        let storage = super::SafeFileStorage::new(
            super::FileStorageRootPathBuf::try_from(root_path.clone()).expect("0a4c0bfd stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold"),
        );
        storage.prepare().await.expect("73802bd5 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        let operation_id = super::StdStorageOperationId::try_from(String::from(
            constants_str::TEST_STALE_STAGING_OPERATION_ID,
        ))
        .expect("d374ce69 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        storage
            .stage_upload(
                &operation_id,
                &super::StdFileBytes::try_from(vec![1u8]).expect("a9899d14 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold"),
            )
            .await
            .expect("df4e565c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        let second_operation_id = super::StdStorageOperationId::try_from(String::from(
            constants_str::TEST_STALE_STAGING_SECOND_OPERATION_ID,
        ))
        .expect("de441c7a stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        storage
            .stage_upload(
                &second_operation_id,
                &super::StdFileBytes::try_from(vec![2u8]).expect("941a849c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold"),
            )
            .await
            .expect("ce87151d stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        let stale_before = std::time::UNIX_EPOCH
            .checked_add(std::time::Duration::from_hours(1_139_568u64))
            .expect("c81a56d9 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        let limit = super::StdStaleStagingEntryLimit::try_from(constants_usize::ONE).expect("c35f98c6 stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        let report = storage
            .cleanup_stale_staging(
                super::FileStorageStagingArea::Upload,
                super::StaleStagingCleanupCfg::new(stale_before.into(), limit, limit),
            )
            .await
            .expect("eb46d89c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
        assert_eq!(
            report,
            super::StaleStagingCleanupReport {
                removed: super::StdStaleStagingEntryCount::from(constants_usize::ONE),
                scanned: super::StdStaleStagingEntryCount::from(constants_usize::ONE),
            }
        );
        let mut remaining_entries =
            tokio::fs::read_dir(root_path.join(constants_str::FILE_UPLOAD_STAGING_DIRECTORY))
                .await
                .expect("acdbf8da stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
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
        tokio::fs::remove_dir_all(root_path)
            .await
            .expect("9cf8105c stale_staging_cleanup_is_bounded_and_removes_regular_files invariant must hold");
    }

    #[test]
    fn relative_paths_and_operation_ids_reject_traversal() {
        assert_eq!(
            super::StorageRelativePathBuf::try_from(std::path::PathBuf::from(
                constants_str::TEST_PATH_TRAVERSAL
            )),
            Err(super::FileStoragePathError::RelativePathInvalid),
        );
        assert_eq!(
            super::StdStorageOperationId::try_from(String::from(
                constants_str::TEST_PATH_TRAVERSAL,
            )),
            Err(super::FileStoragePathError::OperationIdInvalid),
        );
    }

    #[test]
    fn storage_paths_reject_values_above_maximum_length() {
        let relative = constants_str::TEST_JWT_SECRET_CHARACTER_A
            .repeat(super::MAXIMUM_PATH_BYTES.saturating_add(constants_usize::ONE));
        let mut absolute = std::path::MAIN_SEPARATOR.to_string();
        absolute.push_str(relative.as_str());
        assert_eq!(
            super::StorageRelativePathBuf::try_from(std::path::PathBuf::from(relative)),
            Err(super::FileStoragePathError::PathTooLong)
        );
        assert_eq!(
            super::FileStorageRootPathBuf::try_from(std::path::PathBuf::from(absolute)),
            Err(super::FileStoragePathError::PathTooLong)
        );
    }

    #[test]
    fn disk_cache_budget_evicts_oldest_entries_first() {
        let old_path = super::StorageRelativePathBuf::try_from(std::path::PathBuf::from(
            constants_str::TEST_DISK_CACHE_OLD_PATH,
        ))
        .expect("0dc17257 disk_cache_budget_evicts_oldest_entries_first invariant must hold");
        let new_path = super::StorageRelativePathBuf::try_from(std::path::PathBuf::from(
            constants_str::TEST_DISK_CACHE_NEW_PATH,
        ))
        .expect("38c1eca1 disk_cache_budget_evicts_oldest_entries_first invariant must hold");
        let entries = [
            super::DiskCacheEntry::new(old_path.clone(), 4u64.into(), std::time::UNIX_EPOCH.into()),
            super::DiskCacheEntry::new(
                new_path,
                4u64.into(),
                (std::time::UNIX_EPOCH + std::time::Duration::from_secs(1u64)).into(),
            ),
        ];
        let plan = super::plan_disk_cache_eviction(&entries, 10u64.into(), 4u64.into())
            .expect("1bc67951 disk_cache_budget_evicts_oldest_entries_first invariant must hold");
        assert_eq!(plan.as_ref(), &[old_path]);
    }

    #[tokio::test]
    async fn staged_upload_delete_and_rollback_preserve_transaction_boundaries() {
        let root_path = std::env::temp_dir().join(constants_str::TEST_FILE_STORAGE_DIRECTORY);
        match tokio::fs::remove_dir_all(&root_path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("a61c720d {error}"),
        }
        let storage = super::SafeFileStorage::new(
            super::FileStorageRootPathBuf::try_from(root_path.clone()).expect("ec6f4321 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold"),
        );
        storage.prepare().await.expect("ab760e42 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
        let operation_id = super::StdStorageOperationId::try_from(String::from(
            constants_str::TEST_FILE_STORAGE_OPERATION_ID,
        ))
        .expect("ca3f4821 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
        let relative_path = super::StorageRelativePathBuf::try_from(std::path::PathBuf::from(
            constants_str::TEST_FILE_STORAGE_RELATIVE_PATH,
        ))
        .expect("85ed3042 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
        let bytes = super::StdFileBytes::try_from(vec![1u8, 2u8, 3u8]).expect("d7df0f1c staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
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
        let replacement_operation_id = super::StdStorageOperationId::try_from(String::from(
            constants_str::TEST_FILE_STORAGE_REPLACEMENT_OPERATION_ID,
        ))
        .expect("fb7e68b1 staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
        let replacement_bytes = super::StdFileBytes::try_from(vec![4u8, 5u8]).expect("23566f2b staged_upload_delete_and_rollback_preserve_transaction_boundaries invariant must hold");
        storage
            .atomic_replace(
                &replacement_operation_id,
                &relative_path,
                &replacement_bytes,
                super::AtomicReplaceDurability::Flush,
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
}
