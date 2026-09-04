#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
)]
pub struct FileStorageRootPathBuf(std::path::PathBuf);
impl TryFrom<std::path::PathBuf> for FileStorageRootPathBuf {
    type Error = crate::file_storage_path_error::FileStoragePathError;
    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        if value.as_os_str().as_encoded_bytes().len() > crate::domain_types::MAXIMUM_PATH_BYTES {
            return Err(crate::file_storage_path_error::FileStoragePathError::PathTooLong);
        }
        if value.is_absolute() {
            Ok(Self(value))
        } else {
            Err(crate::file_storage_path_error::FileStoragePathError::RootMustBeAbsolute)
        }
    }
}
