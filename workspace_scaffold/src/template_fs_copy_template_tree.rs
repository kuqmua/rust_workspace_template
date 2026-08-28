pub(crate) fn template_fs_copy_template_tree(
    source: crate::ScaffoldPathRef<'_>,
    destination: crate::ScaffoldPathRef<'_>,
    replacements: crate::ReplacementsRef<'_>,
) -> Result<(), crate::ScaffoldError> {
    std::fs::create_dir_all(destination.get())?;
    std::fs::read_dir(source.get())?.try_for_each(|entry_result| {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.get().join(entry.file_name());
        if source_path.is_dir() {
            template_fs_copy_template_tree(
                crate::ScaffoldPathRef::from(source_path.as_path()),
                crate::ScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        } else {
            let _copied_bytes = std::fs::copy(source_path, destination_path.as_path())?;
            super::template_fs_replace_file::template_fs_replace_file(
                crate::ScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        }
    })?;
    Ok(())
}
