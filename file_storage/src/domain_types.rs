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
mod tests;
