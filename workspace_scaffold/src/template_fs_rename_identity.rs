#[allow(
    clippy::single_call_fn,
    reason = "project command owns identity traversal"
)]
pub(crate) fn rename_identity(
    root: crate::domain_types::ScaffoldPathRef<'_>,
    project_name: crate::domain_types::ProjectNameRef<'_>,
    repository_url: crate::domain_types::RepositoryUrlRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    let replacements = [
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_REPOSITORY_URL,
            repository_url.get().to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_SNAKE,
            project_name.get().to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_KEBAB,
            crate::domain_types::naming_kebab_case::kebab_case(project_name)
                .as_ref()
                .to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_TITLE,
            crate::domain_types::naming_title_case::title_case(project_name)
                .as_ref()
                .to_owned(),
        ),
    ];
    let mut pending = vec![root.get().to_path_buf()];
    while let Some(path) = pending.pop() {
        if bool::from(super::template_fs_should_skip::should_skip(
            crate::domain_types::ScaffoldPathRef::from(path.as_path()),
        )) {
            continue;
        }
        if path.is_dir() {
            std::fs::read_dir(path)?.try_for_each(|entry| {
                pending.push(entry?.path());
                Ok::<(), std::io::Error>(())
            })?;
        } else {
            super::template_fs_replace_file::replace_file(
                crate::domain_types::ScaffoldPathRef::from(path.as_path()),
                crate::domain_types::ReplacementsRef::from(replacements.as_slice()),
            )?;
        }
    }
    Ok(())
}
