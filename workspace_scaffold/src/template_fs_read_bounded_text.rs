pub(crate) fn template_fs_read_bounded_text(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
) -> Result<
    crate::scaffold_text::ScaffoldText,
    server_runtime_http::bounded_read_error::BoundedReadError,
> {
    let bytes = server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(scaffold_path_ref.get()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            constants_usize::VALUE_16_777_216,
        ),
    )?;
    let text = server_runtime_http::bounded_text::BoundedText::try_from(bytes)?.into_inner();
    Ok(crate::scaffold_text::ScaffoldText::try_from(text)
        .unwrap_or_else(crate::scaffold_text::ScaffoldText::from))
}
