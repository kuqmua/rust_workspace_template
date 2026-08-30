#[must_use]
pub fn identifier_file_storage_relative_path(
    identifier: &crate::storage_path_segment::StoragePathSegment,
    unique_file_identifier: &crate::storage_path_segment::StoragePathSegment,
    file_name: &crate::multipart_file_name::MultipartFileName,
) -> crate::runtime_storage_relative_path_buf::RuntimeStorageRelativePathBuf {
    let extension = std::path::Path::new(file_name.as_ref())
        .extension()
        .and_then(|value| value.to_str());
    let stored_file_name = extension.map_or_else(
        || unique_file_identifier.as_ref().to_owned(),
        |value| format!("{}.{value}", unique_file_identifier.as_ref()),
    );
    crate::runtime_storage_relative_path_buf::RuntimeStorageRelativePathBuf::from(
        std::path::Path::new(identifier.as_ref()).join(stored_file_name),
    )
}
