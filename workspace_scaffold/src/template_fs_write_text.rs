pub(crate) fn template_fs_write_text(
    path: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    text: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    std::fs::write(path.get(), text.get())?;
    Ok(())
}
