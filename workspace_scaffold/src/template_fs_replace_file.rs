pub(crate) fn replace_file(
    path: crate::domain_types::ScaffoldPathRef<'_>,
    replacements: crate::domain_types::ReplacementsRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    let Ok(contents) = super::template_fs_read_bounded_text::read_bounded_text(path) else {
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
