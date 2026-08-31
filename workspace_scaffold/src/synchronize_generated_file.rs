pub(super) fn synchronize_generated_file(
    path: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    begin: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    end: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    generated: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    write_changes: crate::should_write::ShouldWrite,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let source = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(path)?;
    let (prefix, after_begin) = source
        .as_ref()
        .split_once(begin.get())
        .ok_or(crate::scaffold_error::ScaffoldError::Marker)?;
    let (_previous, suffix) = after_begin
        .split_once(end.get())
        .ok_or(crate::scaffold_error::ScaffoldError::Marker)?;
    let expected = crate::scaffold_text::ScaffoldText::try_from(format!(
        "{prefix}{}{generated}{}{suffix}",
        begin.get(),
        end.get(),
        generated = generated.get()
    ))
    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?;
    if expected.as_ref() == source.as_ref() {
        return Ok(());
    }
    if bool::from(write_changes) {
        crate::template_fs_write_text::template_fs_write_text(
            path,
            crate::scaffold_text_ref::ScaffoldTextRef::from(expected.as_ref()),
        )
    } else {
        Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment)
    }
}
