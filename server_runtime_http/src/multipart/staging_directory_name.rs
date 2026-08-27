pub fn staging_directory_name(
    action: super::FileStagingAction,
) -> Result<super::FileStagingDirectoryName, super::MultipartValueError> {
    super::FileStagingDirectoryName::try_from(String::from(match action {
        super::FileStagingAction::Delete => constants_str::FILE_DELETE_STAGING_DIRECTORY,
        super::FileStagingAction::Upload => constants_str::FILE_UPLOAD_STAGING_DIRECTORY,
    }))
}
