pub(crate) fn template_fs_write_text(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    scaffold_text_ref: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    std::fs::write(scaffold_path_ref.get(), scaffold_text_ref.get())?;
    Ok(())
}
