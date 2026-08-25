#[allow(
    clippy::single_call_fn,
    reason = "identity traversal owns ignored directory policy"
)]
fn should_skip(path: crate::domain_types::ScaffoldPathRef<'_>) -> crate::domain_types::ShouldSkip {
    crate::domain_types::ShouldSkip::from(path.get().components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some(
                constants_str::GIT
                    | constants_str::TARGET
                    | constants_str::WORKSPACE_SCAFFOLD_NODE_MODULES
            )
        )
    }))
}

pub(crate) fn read_bounded_text(
    path: crate::domain_types::ScaffoldPathRef<'_>,
) -> Result<crate::domain_types::ScaffoldText, crate::domain_types::ServerRuntimeBoundedReadError> {
    let bytes = server_runtime_http::domain_types::read_bounded_file(
        server_runtime_http::domain_types::PathRef::from(path.get()),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(
            constants_usize::VALUE_16_777_216,
        ),
    )
    .map_err(crate::domain_types::ServerRuntimeBoundedReadError::from)?;
    let text = server_runtime_http::domain_types::BoundedText::try_from(bytes)
        .map_err(crate::domain_types::ServerRuntimeBoundedReadError::from)?
        .into_inner();
    Ok(crate::domain_types::ScaffoldText::try_from(text)
        .unwrap_or_else(crate::domain_types::ScaffoldText::from))
}

pub(crate) fn write_text(
    path: crate::domain_types::ScaffoldPathRef<'_>,
    text: crate::domain_types::ScaffoldTextRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    std::fs::write(path.get(), text.get())?;
    Ok(())
}

#[allow(
    clippy::single_call_fn,
    reason = "filesystem adapter owns the raw single-file copy operation"
)]
pub(crate) fn copy_file(
    source: crate::domain_types::ScaffoldPathRef<'_>,
    destination: crate::domain_types::ScaffoldPathRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    let _copied_bytes = std::fs::copy(source.get(), destination.get())?;
    Ok(())
}

pub(crate) fn replace_file(
    path: crate::domain_types::ScaffoldPathRef<'_>,
    replacements: crate::domain_types::ReplacementsRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    let Ok(contents) = read_bounded_text(path) else {
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
            crate::domain_types::naming::kebab_case(project_name)
                .as_ref()
                .to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_TITLE,
            crate::domain_types::naming::title_case(project_name)
                .as_ref()
                .to_owned(),
        ),
    ];
    let mut pending = vec![root.get().to_path_buf()];
    while let Some(path) = pending.pop() {
        if bool::from(should_skip(crate::domain_types::ScaffoldPathRef::from(
            path.as_path(),
        ))) {
            continue;
        }
        if path.is_dir() {
            std::fs::read_dir(path)?.try_for_each(|entry| {
                pending.push(entry?.path());
                Ok::<(), std::io::Error>(())
            })?;
        } else {
            replace_file(
                crate::domain_types::ScaffoldPathRef::from(path.as_path()),
                crate::domain_types::ReplacementsRef::from(replacements.as_slice()),
            )?;
        }
    }
    Ok(())
}

pub(crate) fn copy_template_tree(
    source: crate::domain_types::ScaffoldPathRef<'_>,
    destination: crate::domain_types::ScaffoldPathRef<'_>,
    replacements: crate::domain_types::ReplacementsRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    std::fs::create_dir_all(destination.get())?;
    std::fs::read_dir(source.get())?.try_for_each(|entry_result| {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.get().join(entry.file_name());
        if source_path.is_dir() {
            copy_template_tree(
                crate::domain_types::ScaffoldPathRef::from(source_path.as_path()),
                crate::domain_types::ScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        } else {
            let _copied_bytes = std::fs::copy(source_path, destination_path.as_path())?;
            replace_file(
                crate::domain_types::ScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        }
    })?;
    Ok(())
}

pub(crate) fn insert_once(
    path: crate::domain_types::ScaffoldPathRef<'_>,
    marker: crate::domain_types::ScaffoldTextRef<'_>,
    replacement: crate::domain_types::ScaffoldTextRef<'_>,
) -> Result<(), crate::domain_types::ScaffoldError> {
    let contents = read_bounded_text(path)?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn ignored_template_directories_are_explicit() {
        assert!(bool::from(super::should_skip(
            crate::domain_types::ScaffoldPathRef::from(std::path::Path::new("target/generated"))
        )));
        assert!(!bool::from(super::should_skip(
            crate::domain_types::ScaffoldPathRef::from(std::path::Path::new("server/src"))
        )));
    }
}
