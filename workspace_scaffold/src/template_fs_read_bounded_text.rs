pub(crate) fn template_fs_read_bounded_text(
    path: crate::ScaffoldPathRef<'_>,
) -> Result<crate::ScaffoldText, server_runtime_http::domain_types::BoundedReadError> {
    let bytes = server_runtime_http::domain_types::read_bounded_file(
        server_runtime_http::domain_types::PathRef::from(path.get()),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(
            constants_usize::VALUE_16_777_216,
        ),
    )?;
    let text = server_runtime_http::domain_types::BoundedText::try_from(bytes)?.into_inner();
    Ok(crate::ScaffoldText::try_from(text).unwrap_or_else(crate::ScaffoldText::from))
}
