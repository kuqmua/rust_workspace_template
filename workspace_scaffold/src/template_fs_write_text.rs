pub(crate) fn template_fs_write_text(
    path: crate::domain_types::ScaffoldPathRef<'_>,
    text: crate::domain_types::ScaffoldTextRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    std::fs::write(path.get(), text.get())?;
    Ok(())
}
