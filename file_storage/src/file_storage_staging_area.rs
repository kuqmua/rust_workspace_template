use super::domain_types::StorageDirectoryNameRef;

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
