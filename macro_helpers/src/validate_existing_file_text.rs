pub(super) fn validate_existing_file_text(
    written_file_path_ref: crate::written_file_path_ref::WrittenFilePathRef<'_>,
    generated_file_maximum_bytes: crate::generated_file_maximum_bytes::GeneratedFileMaximumBytes,
) -> std::io::Result<()> {
    server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(written_file_path_ref.as_ref()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            usize::from(generated_file_maximum_bytes),
        ),
    )
    .and_then(server_runtime_http::bounded_text::BoundedText::try_from)
    .map(|_text| ())
    .map_err(std::io::Error::other)
}
