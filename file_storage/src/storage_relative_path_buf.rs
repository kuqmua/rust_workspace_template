#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
)]
pub struct StorageRelativePathBuf(std::path::PathBuf);
impl TryFrom<std::path::PathBuf> for StorageRelativePathBuf {
    type Error = crate::file_storage_path_error::FileStoragePathError;
    fn try_from(path_buf: std::path::PathBuf) -> Result<Self, Self::Error> {
        if path_buf.as_os_str().as_encoded_bytes().len() > crate::domain_types::MAXIMUM_PATH_BYTES {
            return Err(crate::file_storage_path_error::FileStoragePathError::PathTooLong);
        }
        let valid = !path_buf.as_os_str().is_empty()
            && path_buf
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_)));
        if valid {
            Ok(Self(path_buf))
        } else {
            Err(crate::file_storage_path_error::FileStoragePathError::RelativePathInvalid)
        }
    }
}
