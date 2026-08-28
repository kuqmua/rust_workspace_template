#[must_use]
pub fn identifier_file_storage_relative_path(
    identifier: &super::StoragePathSegment,
    unique_file_identifier: &super::StoragePathSegment,
    file_name: &super::MultipartFileName,
) -> super::StorageRelativePathBuf {
    let extension = std::path::Path::new(file_name.as_ref())
        .extension()
        .and_then(|value| value.to_str());
    let stored_file_name = extension.map_or_else(
        || unique_file_identifier.as_ref().to_owned(),
        |value| format!("{}.{value}", unique_file_identifier.as_ref()),
    );
    super::StorageRelativePathBuf::from(
        std::path::Path::new(identifier.as_ref()).join(stored_file_name),
    )
}
