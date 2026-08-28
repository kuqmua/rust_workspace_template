pub(crate) fn read_bounded_content(
    path: crate::domain_types::InitPathRef<'_>,
    maximum_bytes: crate::domain_types::InitMaxBytes,
) -> Result<crate::domain_types::EnvContent, server_runtime_http::domain_types::BoundedReadError> {
    let bytes = server_runtime_http::domain_types::read_bounded_file(
        server_runtime_http::domain_types::PathRef::from(path.get()),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(maximum_bytes.get()),
    )?;
    server_runtime_http::domain_types::BoundedText::try_from(bytes)
        .map(crate::domain_types::EnvContent::from)
}
