pub(crate) fn template_fs_replace_file(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    replacements_ref: crate::replacements_ref::ReplacementsRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let Ok(contents) =
        crate::template_fs_read_bounded_text::template_fs_read_bounded_text(scaffold_path_ref)
    else {
        return Ok(());
    };
    let updated_contents = replacements_ref
        .get()
        .iter()
        .fold(contents.as_ref().to_owned(), |value, (from, to)| {
            value.replace(from, to.as_str())
        });
    std::fs::write(scaffold_path_ref.get(), updated_contents)?;
    Ok(())
}
