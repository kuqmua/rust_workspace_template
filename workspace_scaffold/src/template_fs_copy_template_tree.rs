pub(crate) fn template_fs_copy_template_tree(
    source: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    destination: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    replacements_ref: crate::replacements_ref::ReplacementsRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    if std::fs::symlink_metadata(source.get())?
        .file_type()
        .is_symlink()
    {
        return Err(crate::scaffold_error::ScaffoldError::Catalog);
    }
    std::fs::create_dir_all(destination.get())?;
    std::fs::read_dir(source.get())?.try_for_each(|entry_result| {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.get().join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        }
        if file_type.is_dir() {
            template_fs_copy_template_tree(
                crate::scaffold_path_ref::ScaffoldPathRef::from(source_path.as_path()),
                crate::scaffold_path_ref::ScaffoldPathRef::from(destination_path.as_path()),
                replacements_ref,
            )
        } else {
            let _copied_bytes = std::fs::copy(source_path, destination_path.as_path())?;
            crate::template_fs_replace_file::template_fs_replace_file(
                crate::scaffold_path_ref::ScaffoldPathRef::from(destination_path.as_path()),
                replacements_ref,
            )
        }
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    #[test]
    fn test_template_copy_rejects_symlinked_entries() {
        let root = std::env::temp_dir()
            .join(module_path!())
            .join(std::process::id().to_string());
        let source = root.join(constants_str::CARGO_TOML);
        let destination = root.join(constants_str::TARGET);
        std::fs::create_dir_all(source.as_path()).expect(constants_str::DIAGNOSTIC_B1D5D04B);
        let external = root.join(constants_str::ENV_EXAMPLE);
        std::fs::write(external.as_path(), constants_str::EMPTY)
            .expect(constants_str::DIAGNOSTIC_EB140001);
        let symlink = source.join(constants_str::CARGO_TOML);
        std::os::unix::fs::symlink(external.as_path(), symlink.as_path())
            .expect(constants_str::DIAGNOSTIC_4A439A85);
        let file_link_result =
            crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
                crate::scaffold_path_ref::ScaffoldPathRef::from(source.as_path()),
                crate::scaffold_path_ref::ScaffoldPathRef::from(destination.as_path()),
                crate::replacements_ref::ReplacementsRef::from(&[][..]),
            );
        assert!(matches!(
            file_link_result,
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
        assert!(!destination.join(constants_str::CARGO_TOML).exists());
        std::fs::remove_file(symlink.as_path()).expect(constants_str::DIAGNOSTIC_B26E5384);
        std::os::unix::fs::symlink(source.as_path(), symlink.as_path())
            .expect(constants_str::DIAGNOSTIC_11511FE6);
        let directory_link_result =
            crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
                crate::scaffold_path_ref::ScaffoldPathRef::from(source.as_path()),
                crate::scaffold_path_ref::ScaffoldPathRef::from(destination.as_path()),
                crate::replacements_ref::ReplacementsRef::from(&[][..]),
            );
        assert!(matches!(
            directory_link_result,
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
        std::fs::remove_dir_all(root).expect(constants_str::DIAGNOSTIC_8C0F3911);
    }
}
