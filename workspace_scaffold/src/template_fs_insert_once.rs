pub(crate) fn template_fs_insert_once(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    marker: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    replacement: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let contents =
        crate::template_fs_read_bounded_text::template_fs_read_bounded_text(scaffold_path_ref)?;
    if contents.as_ref().contains(replacement.get()) {
        return Ok(());
    }
    let updated = contents
        .as_ref()
        .replacen(marker.get(), replacement.get(), constants_usize::ONE);
    if updated == contents.as_ref() {
        return Err(crate::scaffold_error::ScaffoldError::Marker);
    }
    std::fs::write(scaffold_path_ref.get(), updated)?;
    Ok(())
}
