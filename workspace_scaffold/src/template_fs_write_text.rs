pub(crate) fn template_fs_write_text(
    path: crate::ScaffoldPathRef<'_>,
    text: crate::ScaffoldTextRef<'_>,
) -> Result<(), crate::ScaffoldError> {
    std::fs::write(path.get(), text.get())?;
    Ok(())
}
