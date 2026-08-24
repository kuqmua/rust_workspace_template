#[allow(
    clippy::single_call_fn,
    reason = "identity traversal owns ignored directory policy"
)]
fn should_skip(path: super::StdScaffoldPathRef<'_>) -> super::ShouldSkip {
    super::ShouldSkip::from(path.0.components().any(|component| {
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

pub(super) fn read_bounded_text(
    path: super::StdScaffoldPathRef<'_>,
) -> Result<super::ScaffoldText, super::ServerRuntimeBoundedReadError> {
    let bytes = server_runtime_http::read_bounded_file(
        server_runtime_http::StdPathRef::from(path.0),
        server_runtime_http::BoundedReadMaximumBytes::from(constants_usize::VALUE_16_777_216),
    )
    .map_err(super::ServerRuntimeBoundedReadError::from)?;
    let text = server_runtime_http::BoundedText::try_from(bytes)
        .map_err(super::ServerRuntimeBoundedReadError::from)?
        .into_inner();
    Ok(super::ScaffoldText::try_from(text).unwrap_or_else(super::ScaffoldText::from))
}

pub(super) fn replace_file(
    path: super::StdScaffoldPathRef<'_>,
    replacements: super::ReplacementsRef<'_>,
) -> Result<(), super::ScaffoldError> {
    let Ok(contents) = read_bounded_text(path) else {
        return Ok(());
    };
    let updated_contents = replacements
        .0
        .iter()
        .fold(contents.as_ref().to_owned(), |value, (from, to)| {
            value.replace(from, to.as_str())
        });
    std::fs::write(path.0, updated_contents)?;
    Ok(())
}

#[allow(
    clippy::single_call_fn,
    reason = "project command owns identity traversal"
)]
pub(super) fn rename_identity(
    root: super::StdScaffoldPathRef<'_>,
    project_name: super::ProjectNameRef<'_>,
    repository_url: super::RepositoryUrlRef<'_>,
) -> Result<(), super::ScaffoldError> {
    let replacements = [
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_REPOSITORY_URL,
            repository_url.0.to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_SNAKE,
            project_name.0.to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_KEBAB,
            super::naming::kebab_case(project_name).as_ref().to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_TITLE,
            super::naming::title_case(project_name).as_ref().to_owned(),
        ),
    ];
    let mut pending = vec![root.0.to_path_buf()];
    while let Some(path) = pending.pop() {
        if bool::from(should_skip(super::StdScaffoldPathRef::from(path.as_path()))) {
            continue;
        }
        if path.is_dir() {
            std::fs::read_dir(path)?.try_for_each(|entry| {
                pending.push(entry?.path());
                Ok::<(), std::io::Error>(())
            })?;
        } else {
            replace_file(
                super::StdScaffoldPathRef::from(path.as_path()),
                super::ReplacementsRef::from(replacements.as_slice()),
            )?;
        }
    }
    Ok(())
}

pub(super) fn copy_template_tree(
    source: super::StdScaffoldPathRef<'_>,
    destination: super::StdScaffoldPathRef<'_>,
    replacements: super::ReplacementsRef<'_>,
) -> Result<(), super::ScaffoldError> {
    std::fs::create_dir_all(destination.0)?;
    std::fs::read_dir(source.0)?.try_for_each(|entry_result| {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.0.join(entry.file_name());
        if source_path.is_dir() {
            copy_template_tree(
                super::StdScaffoldPathRef::from(source_path.as_path()),
                super::StdScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        } else {
            let _copied_bytes = std::fs::copy(source_path, destination_path.as_path())?;
            replace_file(
                super::StdScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        }
    })?;
    Ok(())
}

pub(super) fn insert_once(
    path: super::StdScaffoldPathRef<'_>,
    marker: super::ScaffoldTextRef<'_>,
    replacement: super::ScaffoldTextRef<'_>,
) -> Result<(), super::ScaffoldError> {
    let contents = read_bounded_text(path)?;
    if contents.as_ref().contains(replacement.0) {
        return Ok(());
    }
    let updated = contents
        .as_ref()
        .replacen(marker.0, replacement.0, constants_usize::ONE);
    if updated == contents.as_ref() {
        return Err(super::ScaffoldError::Marker);
    }
    std::fs::write(path.0, updated)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn ignored_template_directories_are_explicit() {
        assert!(bool::from(super::should_skip(
            super::super::StdScaffoldPathRef::from(std::path::Path::new("target/generated"))
        )));
        assert!(!bool::from(super::should_skip(
            super::super::StdScaffoldPathRef::from(std::path::Path::new("server/src"))
        )));
    }
}
