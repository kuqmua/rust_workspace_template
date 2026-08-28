pub(crate) fn template_fs_replace_file(
    path: crate::ScaffoldPathRef<'_>,
    replacements: crate::ReplacementsRef<'_>,
) -> Result<(), crate::ScaffoldError> {
    let Ok(contents) = super::template_fs_read_bounded_text::template_fs_read_bounded_text(path)
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
