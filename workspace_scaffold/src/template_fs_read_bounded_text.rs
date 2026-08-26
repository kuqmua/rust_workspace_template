pub(crate) fn template_fs_read_bounded_text(
    path: crate::domain_types::ScaffoldPathRef<'_>,
) -> Result<crate::domain_types::ScaffoldText, crate::domain_types::ServerRuntimeBoundedReadError> {
    let bytes = server_runtime_http::domain_types::read_bounded_file(
        server_runtime_http::domain_types::PathRef::from(path.get()),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(
            constants_usize::VALUE_16_777_216,
        ),
    )
    .map_err(crate::domain_types::ServerRuntimeBoundedReadError::from)?;
    let text = server_runtime_http::domain_types::BoundedText::try_from(bytes)
        .map_err(crate::domain_types::ServerRuntimeBoundedReadError::from)?
        .into_inner();
    Ok(crate::domain_types::ScaffoldText::try_from(text)
        .unwrap_or_else(crate::domain_types::ScaffoldText::from))
}
