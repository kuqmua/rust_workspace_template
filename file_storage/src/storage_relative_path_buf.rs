#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::domain_types::{FileStoragePathError, MAXIMUM_PATH_BYTES};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct StorageRelativePathBuf(pub(super) std::path::PathBuf);
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
