pub(super) fn validate_existing_file_text(
    path: super::WrittenFilePathRef<'_>,
    maximum_bytes: super::GeneratedFileMaximumBytes,
) -> std::io::Result<()> {
    server_runtime_http::domain_types::read_bounded_file(
        server_runtime_http::domain_types::PathRef::from(path.as_ref()),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(maximum_bytes.0),
    )
    .and_then(server_runtime_http::domain_types::BoundedText::try_from)
    .map(|_text| ())
    .map_err(std::io::Error::other)
}
