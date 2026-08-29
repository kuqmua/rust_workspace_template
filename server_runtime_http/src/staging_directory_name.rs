pub fn staging_directory_name(
    action: crate::file_staging_action::FileStagingAction,
) -> Result<
    crate::file_staging_directory_name::FileStagingDirectoryName,
    crate::multipart_value_error::MultipartValueError,
> {
    crate::file_staging_directory_name::FileStagingDirectoryName::try_from(String::from(
        match action {
            crate::file_staging_action::FileStagingAction::Delete => {
                constants_str::FILE_DELETE_STAGING_DIRECTORY
            }
            crate::file_staging_action::FileStagingAction::Upload => {
                constants_str::FILE_UPLOAD_STAGING_DIRECTORY
            }
        },
    ))
}
