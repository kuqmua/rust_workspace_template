pub(crate) fn read_bounded_content(
    path: crate::init_path_ref::InitPathRef<'_>,
    maximum_bytes: crate::init_max_bytes::InitMaxBytes,
) -> Result<crate::env_content::EnvContent, server_runtime_http::bounded_read_error::BoundedReadError>
{
    let bytes = server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::path_ref::PathRef::from(path.get()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            maximum_bytes.get(),
        ),
    )?;
    server_runtime_http::bounded_text::BoundedText::try_from(bytes)
        .map(crate::env_content::EnvContent::from)
}
