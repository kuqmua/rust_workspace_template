pub(crate) fn template_fs_replace_file(
    path: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    replacements: crate::replacements_ref::ReplacementsRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let Ok(contents) = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(path)
    else {
        return Ok(());
    };
    let updated_contents = replacements
        .get()
        .iter()
        .fold(contents.as_ref().to_owned(), |value, (from, to)| {
            value.replace(from, to.as_str())
        });
    std::fs::write(path.get(), updated_contents)?;
    Ok(())
}
