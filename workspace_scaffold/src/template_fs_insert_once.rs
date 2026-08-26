pub(crate) fn template_fs_insert_once(
    path: crate::domain_types::ScaffoldPathRef<'_>,
    marker: crate::domain_types::ScaffoldTextRef<'_>,
    replacement: crate::domain_types::ScaffoldTextRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    let contents = super::template_fs_read_bounded_text::template_fs_read_bounded_text(path)?;
    if contents.as_ref().contains(replacement.get()) {
        return Ok(());
    }
    let updated = contents
        .as_ref()
        .replacen(marker.get(), replacement.get(), constants_usize::ONE);
    if updated == contents.as_ref() {
        return Err(crate::domain_types::ScaffoldError::Marker);
    }
    std::fs::write(path.get(), updated)?;
    Ok(())
}
