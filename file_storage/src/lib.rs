const MAXIMUM_FILE_BYTES: usize = 104_857_600usize;
const MAXIMUM_OPERATION_ID_BYTES: usize = 128usize;

#[derive(Debug)]
pub struct StdFileStorageIoError(std::io::Error);
impl From<std::io::Error> for StdFileStorageIoError {
    fn from(value: std::io::Error) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for StdFileStorageIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for StdFileStorageIoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct StdStoragePathRef<'value_lt>(&'value_lt std::path::Path);
impl<'value_lt> From<&'value_lt std::path::Path> for StdStoragePathRef<'value_lt> {
    fn from(value: &'value_lt std::path::Path) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug)]
struct StorageDirectoryNameRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for StorageDirectoryNameRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdFileStorageRoot(std::path::PathBuf);
impl TryFrom<std::path::PathBuf> for StdFileStorageRoot {
    type Error = FileStoragePathError;
    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        if value.is_absolute() {
            Ok(Self(value))
        } else {
            Err(FileStoragePathError::RootMustBeAbsolute)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdStorageRelativePath(std::path::PathBuf);
impl TryFrom<std::path::PathBuf> for StdStorageRelativePath {
    type Error = FileStoragePathError;
    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdStorageOperationId(String);
impl TryFrom<String> for StdStorageOperationId {
    type Error = FileStoragePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > MAXIMUM_OPERATION_ID_BYTES {
            return Err(FileStoragePathError::OperationIdInvalid);
        }
        text_policy::validate_url_safe_token_part(
            text_policy::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::UrlSafeTokenPartMaximumBytes::from(MAXIMUM_OPERATION_ID_BYTES),
        )
        .map_err(|_error| FileStoragePathError::OperationIdInvalid)?;
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdFileBytes(Vec<u8>);
impl TryFrom<Vec<u8>> for StdFileBytes {
    type Error = FileStoragePathError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() <= MAXIMUM_FILE_BYTES {
            Ok(Self(value))
        } else {
            Err(FileStoragePathError::FileTooLarge)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FileStoragePathError {
    #[error("{}", str_constants::FILE_STORAGE_FILE_TOO_LARGE)]
    FileTooLarge,
    #[error("{}", str_constants::FILE_STORAGE_OPERATION_ID_INVALID)]
    OperationIdInvalid,
    #[error("{}", str_constants::FILE_STORAGE_RELATIVE_PATH_INVALID)]
    RelativePathInvalid,
    #[error("{}", str_constants::FILE_STORAGE_ROOT_MUST_BE_ABSOLUTE)]
    RootMustBeAbsolute,
}

#[derive(Debug, thiserror::Error)]
pub enum FileStorageError {
    #[error("{}", str_constants::FILE_STORAGE_ATOMIC_REPLACE_AND_CLEANUP_ERROR)]
    AtomicReplaceAndCleanup {
        cleanup: StdFileStorageIoError,
        replace: StdFileStorageIoError,
    },
    #[error("{}", str_constants::FILE_STORAGE_DESTINATION_EXISTS)]
    DestinationExists,
    #[error("{}", str_constants::FILE_STORAGE_IO_ERROR)]
    Io(#[source] StdFileStorageIoError),
    #[error("{}", str_constants::FILE_STORAGE_SOURCE_NOT_REGULAR)]
    SourceNotRegular,
    #[error("{}", str_constants::FILE_STORAGE_STAGING_ENTRY_EXISTS)]
    StagingEntryExists,
    #[error("{}", str_constants::FILE_STORAGE_PATH_IS_SYMLINK)]
    Symlink,
}

#[derive(Clone, Debug)]
pub struct SafeFileStorage {
    root: StdFileStorageRoot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileStorageStagingArea {
    Delete,
    Upload,
}
impl FileStorageStagingArea {
    const fn directory_name(self) -> StorageDirectoryNameRef<'static> {
        match self {
            Self::Delete => StorageDirectoryNameRef(str_constants::FILE_DELETE_STAGING_DIRECTORY),
            Self::Upload => StorageDirectoryNameRef(str_constants::FILE_UPLOAD_STAGING_DIRECTORY),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdStaleStagingEntryLimit(usize);
impl TryFrom<usize> for StdStaleStagingEntryLimit {
    type Error = StaleStagingCleanupCfgError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0usize || value > 10_000usize {
            Err(StaleStagingCleanupCfgError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdStaleBefore(std::time::SystemTime);
impl From<std::time::SystemTime> for StdStaleBefore {
    fn from(value: std::time::SystemTime) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaleStagingCleanupCfg {
    maximum_removed: StdStaleStagingEntryLimit,
    maximum_scanned: StdStaleStagingEntryLimit,
    stale_before: StdStaleBefore,
}
impl StaleStagingCleanupCfg {
    #[must_use]
    pub const fn new(
        stale_before: StdStaleBefore,
        maximum_scanned: StdStaleStagingEntryLimit,
        maximum_removed: StdStaleStagingEntryLimit,
    ) -> Self {
        Self {
            maximum_removed,
            maximum_scanned,
            stale_before,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("stale staging cleanup limit must be between 1 and 10000")]
pub struct StaleStagingCleanupCfgError;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StdStaleStagingEntryCount(usize);
impl From<usize> for StdStaleStagingEntryCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<StdStaleStagingEntryCount> for usize {
    fn from(value: StdStaleStagingEntryCount) -> Self {
        value.0
    }
}
impl std::fmt::Display for StdStaleStagingEntryCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StaleStagingCleanupReport {
    removed: StdStaleStagingEntryCount,
    scanned: StdStaleStagingEntryCount,
}
impl StaleStagingCleanupReport {
    #[must_use]
    pub const fn removed(self) -> StdStaleStagingEntryCount {
        self.removed
    }
    #[must_use]
    pub const fn scanned(self) -> StdStaleStagingEntryCount {
        self.scanned
    }
}
#[allow(clippy::arbitrary_source_item_ordering)] // transactional API is grouped as prepare, stage, commit, and rollback operations
impl SafeFileStorage {
    pub async fn cleanup_stale_staging(
        &self,
        area: FileStorageStagingArea,
        cfg: StaleStagingCleanupCfg,
    ) -> Result<StaleStagingCleanupReport, FileStorageError> {
        let directory = self.root.0.join(area.directory_name().0);
        self.ensure_directory_not_symlink(directory.as_path().into())
            .await?;
        let mut entries = tokio::fs::read_dir(directory)
            .await
            .map_err(|error| FileStorageError::Io(error.into()))?;
        let mut report = StaleStagingCleanupReport::default();
        while report.scanned.0 < cfg.maximum_scanned.0 && report.removed.0 < cfg.maximum_removed.0 {
            let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|error| FileStorageError::Io(error.into()))?
            else {
                break;
            };
            report.scanned.0 = report.scanned.0.saturating_add(1usize);
            let file_type = entry
                .file_type()
                .await
                .map_err(|error| FileStorageError::Io(error.into()))?;
            if file_type.is_dir() || file_type.is_symlink() {
                continue;
            }
            let metadata = entry
                .metadata()
                .await
                .map_err(|error| FileStorageError::Io(error.into()))?;
            let Ok(modified) = metadata.modified() else {
                continue;
            };
            if modified > cfg.stale_before.0 {
                continue;
            }
            match tokio::fs::remove_file(entry.path()).await {
                Ok(()) => report.removed.0 = report.removed.0.saturating_add(1usize),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(FileStorageError::Io(error.into())),
            }
        }
        Ok(report)
    }
    pub async fn atomic_replace(
        &self,
        operation_id: &StdStorageOperationId,
        destination: &StdStorageRelativePath,
        bytes: &StdFileBytes,
        durability: AtomicReplaceDurability,
    ) -> Result<(), FileStorageError> {
        self.stage_upload(operation_id, bytes).await?;
        let staging_path = self
            .root
            .0
            .join(str_constants::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(operation_id.0.as_str());
        if durability == AtomicReplaceDurability::SyncAll {
            let file = tokio::fs::OpenOptions::new()
                .write(true)
                .open(&staging_path)
                .await
                .map_err(|error| FileStorageError::Io(error.into()))?;
            file.sync_all()
                .await
                .map_err(|error| FileStorageError::Io(error.into()))?;
        }
        self.ensure_destination_parent(destination).await?;
        let destination_path = self.root.0.join(&destination.0);
        match tokio::fs::symlink_metadata(&destination_path).await {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(FileStorageError::Symlink);
            }
            Ok(metadata) if !metadata.is_file() => {
                return Err(FileStorageError::SourceNotRegular);
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(FileStorageError::Io(error.into())),
        }
        if let Err(replace) = tokio::fs::rename(&staging_path, destination_path).await {
            return match tokio::fs::remove_file(staging_path).await {
                Ok(()) => Err(FileStorageError::Io(replace.into())),
                Err(cleanup) => Err(FileStorageError::AtomicReplaceAndCleanup {
                    cleanup: cleanup.into(),
                    replace: replace.into(),
                }),
            };
        }
        Ok(())
    }

    #[must_use]
    pub const fn new(root: StdFileStorageRoot) -> Self {
        Self { root }
    }

    pub async fn prepare(&self) -> Result<(), FileStorageError> {
        tokio::fs::create_dir_all(&self.root.0)
            .await
            .map_err(|error| FileStorageError::Io(error.into()))?;
        self.ensure_directory_not_symlink(self.root.0.as_path().into())
            .await?;
        self.prepare_staging_directory(str_constants::FILE_UPLOAD_STAGING_DIRECTORY.into())
            .await?;
        self.prepare_staging_directory(str_constants::FILE_DELETE_STAGING_DIRECTORY.into())
            .await
    }

    pub async fn stage_upload(
        &self,
        operation_id: &StdStorageOperationId,
        bytes: &StdFileBytes,
    ) -> Result<(), FileStorageError> {
        let staging_path = self
            .root
            .0
            .join(str_constants::FILE_UPLOAD_STAGING_DIRECTORY)
            .join(operation_id.0.as_str());
        let mut file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(staging_path)
            .await
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    FileStorageError::StagingEntryExists
                } else {
                    FileStorageError::Io(error.into())
                }
            })?;
        tokio::io::AsyncWriteExt::write_all(&mut file, &bytes.0)
            .await
            .map_err(|error| FileStorageError::Io(error.into()))?;
        tokio::io::AsyncWriteExt::flush(&mut file)
            .await
            .map_err(|error| FileStorageError::Io(error.into()))
    }

    pub async fn commit_upload(
        &self,
        operation_id: &StdStorageOperationId,
        destination: &StdStorageRelativePath,
    ) -> Result<(), FileStorageError> {
        let destination_path = self.root.0.join(&destination.0);
        self.ensure_destination_parent(destination).await?;
        self.ensure_destination_absent(destination_path.as_path().into())
            .await?;
        tokio::fs::rename(
            self.root
                .0
                .join(str_constants::FILE_UPLOAD_STAGING_DIRECTORY)
                .join(operation_id.0.as_str()),
            destination_path,
        )
        .await
        .map_err(|error| FileStorageError::Io(error.into()))
    }

    pub async fn rollback_upload(
        &self,
        operation_id: &StdStorageOperationId,
    ) -> Result<(), FileStorageError> {
        tokio::fs::remove_file(
            self.root
                .0
                .join(str_constants::FILE_UPLOAD_STAGING_DIRECTORY)
                .join(operation_id.0.as_str()),
        )
        .await
        .map_err(|error| FileStorageError::Io(error.into()))
    }

    pub async fn stage_delete(
        &self,
        operation_id: &StdStorageOperationId,
        source: &StdStorageRelativePath,
    ) -> Result<(), FileStorageError> {
        let source_path = self.root.0.join(&source.0);
        self.ensure_regular_file(source_path.as_path().into())
            .await?;
        tokio::fs::rename(
            source_path,
            self.root
                .0
                .join(str_constants::FILE_DELETE_STAGING_DIRECTORY)
                .join(operation_id.0.as_str()),
        )
        .await
        .map_err(|error| FileStorageError::Io(error.into()))
    }

    pub async fn rollback_delete(
        &self,
        operation_id: &StdStorageOperationId,
        destination: &StdStorageRelativePath,
    ) -> Result<(), FileStorageError> {
        let destination_path = self.root.0.join(&destination.0);
        self.ensure_destination_parent(destination).await?;
        self.ensure_destination_absent(destination_path.as_path().into())
            .await?;
        tokio::fs::rename(
            self.root
                .0
                .join(str_constants::FILE_DELETE_STAGING_DIRECTORY)
                .join(operation_id.0.as_str()),
            destination_path,
        )
        .await
        .map_err(|error| FileStorageError::Io(error.into()))
    }

    pub async fn commit_delete(
        &self,
        operation_id: &StdStorageOperationId,
    ) -> Result<(), FileStorageError> {
        tokio::fs::remove_file(
            self.root
                .0
                .join(str_constants::FILE_DELETE_STAGING_DIRECTORY)
                .join(operation_id.0.as_str()),
        )
        .await
        .map_err(|error| FileStorageError::Io(error.into()))
    }

    async fn prepare_staging_directory(
        &self,
        directory_name: StorageDirectoryNameRef<'_>,
    ) -> Result<(), FileStorageError> {
        let path = self.root.0.join(directory_name.0);
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(|error| FileStorageError::Io(error.into()))?;
        self.ensure_directory_not_symlink(path.as_path().into())
            .await
    }

    async fn ensure_directory_not_symlink(
        &self,
        path: StdStoragePathRef<'_>,
    ) -> Result<(), FileStorageError> {
        let metadata = tokio::fs::symlink_metadata(path.0)
            .await
            .map_err(|error| FileStorageError::Io(error.into()))?;
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            Ok(())
        } else {
            Err(FileStorageError::Symlink)
        }
    }

    async fn ensure_destination_absent(
        &self,
        path: StdStoragePathRef<'_>,
    ) -> Result<(), FileStorageError> {
        match tokio::fs::symlink_metadata(path.0).await {
            Ok(metadata) if metadata.file_type().is_symlink() => Err(FileStorageError::Symlink),
            Ok(_metadata) => Err(FileStorageError::DestinationExists),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(FileStorageError::Io(error.into())),
        }
    }

    async fn ensure_destination_parent(
        &self,
        relative_path: &StdStorageRelativePath,
    ) -> Result<(), FileStorageError> {
        let mut current = self.root.0.clone();
        let mut components = relative_path
            .0
            .parent()
            .into_iter()
            .flat_map(std::path::Path::components);
        #[allow(clippy::while_let_on_iterator)]
        // repository policy forbids for loops and each component requires awaited filesystem validation
        while let Some(component) = components.next() {
            current.push(component.as_os_str());
            match tokio::fs::symlink_metadata(&current).await {
                Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
                Ok(_metadata) => return Err(FileStorageError::Symlink),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    tokio::fs::create_dir(&current)
                        .await
                        .map_err(|create_error| FileStorageError::Io(create_error.into()))?;
                    self.ensure_directory_not_symlink(current.as_path().into())
                        .await?;
                }
                Err(error) => return Err(FileStorageError::Io(error.into())),
            }
        }
        Ok(())
    }

    async fn ensure_regular_file(
        &self,
        path: StdStoragePathRef<'_>,
    ) -> Result<(), FileStorageError> {
        let metadata = tokio::fs::symlink_metadata(path.0)
            .await
            .map_err(|error| FileStorageError::Io(error.into()))?;
        if metadata.is_file() && !metadata.file_type().is_symlink() {
            Ok(())
        } else if metadata.file_type().is_symlink() {
            Err(FileStorageError::Symlink)
        } else {
            Err(FileStorageError::SourceNotRegular)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AtomicReplaceDurability {
    Flush,
    SyncAll,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdDiskCacheSize(u64);
impl From<u64> for StdDiskCacheSize {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdDiskCacheModifiedAt(std::time::SystemTime);
impl From<std::time::SystemTime> for StdDiskCacheModifiedAt {
    fn from(value: std::time::SystemTime) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiskCacheEntry {
    modified_at: StdDiskCacheModifiedAt,
    path: StdStorageRelativePath,
    size: StdDiskCacheSize,
}
impl DiskCacheEntry {
    #[must_use]
    pub const fn new(
        path: StdStorageRelativePath,
        size: StdDiskCacheSize,
        modified_at: StdDiskCacheModifiedAt,
    ) -> Self {
        Self {
            modified_at,
            path,
            size,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiskCacheEvictionPlan(Vec<StdStorageRelativePath>);
impl AsRef<[StdStorageRelativePath]> for DiskCacheEvictionPlan {
    fn as_ref(&self) -> &[StdStorageRelativePath] {
        self.0.as_slice()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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
    let mut current = entries.iter().try_fold(0u64, |total, entry| {
        total
            .checked_add(entry.size.0)
            .ok_or(DiskCacheBudgetError::SizeOverflow)
    })?;
    let mut ordered = entries.iter().collect::<Vec<_>>();
    ordered.sort_unstable_by_key(|entry| entry.modified_at.0);
    let mut remove = Vec::new();
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
    Ok(DiskCacheEvictionPlan(remove))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn stale_staging_cleanup_is_bounded_and_removes_regular_files() {
        let root_path = std::env::temp_dir().join(str_constants::TEST_STALE_STAGING_DIRECTORY);
        match tokio::fs::remove_dir_all(&root_path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("e0757d39 {error}"),
        }
        let storage = super::SafeFileStorage::new(
            super::StdFileStorageRoot::try_from(root_path.clone()).expect("0a4c0bfd"),
        );
        storage.prepare().await.expect("73802bd5");
        let operation_id = super::StdStorageOperationId::try_from(String::from(
            str_constants::TEST_STALE_STAGING_OPERATION_ID,
        ))
        .expect("d374ce69");
        storage
            .stage_upload(
                &operation_id,
                &super::StdFileBytes::try_from(vec![1u8]).expect("a9899d14"),
            )
            .await
            .expect("df4e565c");
        let second_operation_id = super::StdStorageOperationId::try_from(String::from(
            str_constants::TEST_STALE_STAGING_SECOND_OPERATION_ID,
        ))
        .expect("de441c7a");
        storage
            .stage_upload(
                &second_operation_id,
                &super::StdFileBytes::try_from(vec![2u8]).expect("941a849c"),
            )
            .await
            .expect("ce87151d");
        let stale_before = std::time::UNIX_EPOCH
            .checked_add(std::time::Duration::from_hours(1_139_568u64))
            .expect("c81a56d9");
        let limit = super::StdStaleStagingEntryLimit::try_from(1usize).expect("c35f98c6");
        let report = storage
            .cleanup_stale_staging(
                super::FileStorageStagingArea::Upload,
                super::StaleStagingCleanupCfg::new(stale_before.into(), limit, limit),
            )
            .await
            .expect("eb46d89c");
        assert_eq!(
            report,
            super::StaleStagingCleanupReport {
                removed: super::StdStaleStagingEntryCount::from(1usize),
                scanned: super::StdStaleStagingEntryCount::from(1usize),
            }
        );
        let mut remaining_entries =
            tokio::fs::read_dir(root_path.join(str_constants::FILE_UPLOAD_STAGING_DIRECTORY))
                .await
                .expect("acdbf8da");
        assert!(
            remaining_entries
                .next_entry()
                .await
                .expect("3c5c9b70")
                .is_some()
        );
        assert!(
            remaining_entries
                .next_entry()
                .await
                .expect("406536b7")
                .is_none()
        );
        tokio::fs::remove_dir_all(root_path)
            .await
            .expect("9cf8105c");
    }

    #[test]
    fn relative_paths_and_operation_ids_reject_traversal() {
        assert_eq!(
            super::StdStorageRelativePath::try_from(std::path::PathBuf::from(
                str_constants::TEST_PATH_TRAVERSAL
            )),
            Err(super::FileStoragePathError::RelativePathInvalid),
        );
        assert_eq!(
            super::StdStorageOperationId::try_from(String::from(
                str_constants::TEST_PATH_TRAVERSAL,
            )),
            Err(super::FileStoragePathError::OperationIdInvalid),
        );
    }

    #[test]
    fn disk_cache_budget_evicts_oldest_entries_first() {
        let old_path = super::StdStorageRelativePath::try_from(std::path::PathBuf::from(
            str_constants::TEST_DISK_CACHE_OLD_PATH,
        ))
        .expect("0dc17257");
        let new_path = super::StdStorageRelativePath::try_from(std::path::PathBuf::from(
            str_constants::TEST_DISK_CACHE_NEW_PATH,
        ))
        .expect("38c1eca1");
        let entries = [
            super::DiskCacheEntry::new(old_path.clone(), 4u64.into(), std::time::UNIX_EPOCH.into()),
            super::DiskCacheEntry::new(
                new_path,
                4u64.into(),
                (std::time::UNIX_EPOCH + std::time::Duration::from_secs(1u64)).into(),
            ),
        ];
        let plan =
            super::plan_disk_cache_eviction(&entries, 10u64.into(), 4u64.into()).expect("1bc67951");
        assert_eq!(plan.as_ref(), &[old_path]);
    }

    #[tokio::test]
    async fn staged_upload_delete_and_rollback_preserve_transaction_boundaries() {
        let root_path = std::env::temp_dir().join(str_constants::TEST_FILE_STORAGE_DIRECTORY);
        match tokio::fs::remove_dir_all(&root_path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("a61c720d {error}"),
        }
        let storage = super::SafeFileStorage::new(
            super::StdFileStorageRoot::try_from(root_path.clone()).expect("ec6f4321"),
        );
        storage.prepare().await.expect("ab760e42");
        let operation_id = super::StdStorageOperationId::try_from(String::from(
            str_constants::TEST_FILE_STORAGE_OPERATION_ID,
        ))
        .expect("ca3f4821");
        let relative_path = super::StdStorageRelativePath::try_from(std::path::PathBuf::from(
            str_constants::TEST_FILE_STORAGE_RELATIVE_PATH,
        ))
        .expect("85ed3042");
        let bytes = super::StdFileBytes::try_from(vec![1u8, 2u8, 3u8]).expect("d7df0f1c");
        storage
            .stage_upload(&operation_id, &bytes)
            .await
            .expect("94c1083e");
        storage
            .commit_upload(&operation_id, &relative_path)
            .await
            .expect("217f53e4");
        let _metadata_after_upload = tokio::fs::metadata(root_path.join(&relative_path.0))
            .await
            .expect("a28e410c");
        storage
            .stage_delete(&operation_id, &relative_path)
            .await
            .expect("40761d28");
        storage
            .rollback_delete(&operation_id, &relative_path)
            .await
            .expect("1cd05291");
        let _metadata_after_delete_rollback = tokio::fs::metadata(root_path.join(&relative_path.0))
            .await
            .expect("3c48b27d");
        let replacement_operation_id = super::StdStorageOperationId::try_from(String::from(
            str_constants::TEST_FILE_STORAGE_REPLACEMENT_OPERATION_ID,
        ))
        .expect("fb7e68b1");
        let replacement_bytes = super::StdFileBytes::try_from(vec![4u8, 5u8]).expect("23566f2b");
        storage
            .atomic_replace(
                &replacement_operation_id,
                &relative_path,
                &replacement_bytes,
                super::AtomicReplaceDurability::Flush,
            )
            .await
            .expect("a1ea86b8");
        assert_eq!(
            tokio::fs::read(root_path.join(&relative_path.0))
                .await
                .expect("571084e8"),
            vec![4u8, 5u8],
        );
        tokio::fs::remove_dir_all(root_path)
            .await
            .expect("9a69203b");
    }
}
