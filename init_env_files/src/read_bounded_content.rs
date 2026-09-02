pub(crate) fn read_bounded_content(
    init_path_ref: crate::init_path_ref::InitPathRef<'_>,
    init_max_bytes: crate::init_max_bytes::InitMaxBytes,
) -> Result<crate::env_content::EnvContent, server_runtime_http::bounded_read_error::BoundedReadError>
{
    let bytes = server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(init_path_ref.get()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            init_max_bytes.get(),
        ),
    )?;
    server_runtime_http::bounded_text::BoundedText::try_from(bytes)
        .map(crate::env_content::EnvContent::from)
}
