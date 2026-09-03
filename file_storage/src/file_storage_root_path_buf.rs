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
    fn try_from(path_buf: std::path::PathBuf) -> Result<Self, Self::Error> {
        if path_buf.as_os_str().as_encoded_bytes().len() > crate::domain_types::MAXIMUM_PATH_BYTES {
            return Err(crate::file_storage_path_error::FileStoragePathError::PathTooLong);
        }
        if path_buf.is_absolute() {
            Ok(Self(path_buf))
        } else {
            Err(crate::file_storage_path_error::FileStoragePathError::RootMustBeAbsolute)
        }
    }
}
