#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileStorageStagingArea {
    Delete,
    Upload,
}
impl FileStorageStagingArea {
    pub(crate) fn directory_name(
        self,
    ) -> crate::storage_directory_name_ref::StorageDirectoryNameRef<'static> {
        match self {
            Self::Delete => crate::storage_directory_name_ref::StorageDirectoryNameRef::from(
                constants_str::FILE_DELETE_STAGING_DIRECTORY,
            ),
            Self::Upload => crate::storage_directory_name_ref::StorageDirectoryNameRef::from(
                constants_str::FILE_UPLOAD_STAGING_DIRECTORY,
            ),
        }
    }
}
