#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
)]
pub struct StorageRelativePathBuf(std::path::PathBuf);
impl TryFrom<std::path::PathBuf> for StorageRelativePathBuf {
    type Error = crate::file_storage_path_error::FileStoragePathError;
    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        if value.as_os_str().as_encoded_bytes().len() > crate::domain_types::MAXIMUM_PATH_BYTES {
            return Err(crate::file_storage_path_error::FileStoragePathError::PathTooLong);
        }
        let reserved = value.components().next().is_some_and(|component| {
            component.as_os_str()
                == std::ffi::OsStr::new(constants_str::FILE_UPLOAD_STAGING_DIRECTORY)
                || component.as_os_str()
                    == std::ffi::OsStr::new(constants_str::FILE_DELETE_STAGING_DIRECTORY)
        });
        let valid = !reserved
            && !value.as_os_str().is_empty()
            && !value.as_os_str().as_encoded_bytes().contains(&b'\0')
            && value
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_)));
        if valid {
            Ok(Self(value))
        } else {
            Err(crate::file_storage_path_error::FileStoragePathError::RelativePathInvalid)
        }
    }
}
