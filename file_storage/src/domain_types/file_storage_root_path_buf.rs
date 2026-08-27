use super::{FileStoragePathError, MAXIMUM_PATH_BYTES};

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
