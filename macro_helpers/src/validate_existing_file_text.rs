pub(super) fn validate_existing_file_text(
    path: crate::written_file_path_ref::WrittenFilePathRef<'_>,
    maximum_bytes: crate::generated_file_maximum_bytes::GeneratedFileMaximumBytes,
) -> std::io::Result<()> {
    server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(path.as_ref()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            maximum_bytes.0,
        ),
    )
    .and_then(server_runtime_http::bounded_text::BoundedText::try_from)
    .map(|_text| ())
    .map_err(std::io::Error::other)
}
