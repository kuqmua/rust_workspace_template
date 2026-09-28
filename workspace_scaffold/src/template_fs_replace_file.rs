pub(crate) fn template_fs_replace_file(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    replacements_ref: crate::replacements_ref::ReplacementsRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let contents = match crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
        scaffold_path_ref,
    ) {
        Ok(contents) => contents,
        Err(server_runtime_http::bounded_read_error::BoundedReadError::Utf8 { .. }) => {
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let replacements = replacements_ref.get();
    if replacements.iter().any(|(from, _to)| from.is_empty()) {
        return Err(crate::scaffold_error::ScaffoldError::Catalog);
    }
    let mut remaining = contents.as_ref();
    let mut updated_contents = String::with_capacity(remaining.len());
    while !remaining.is_empty() {
        let next = replacements
            .iter()
            .enumerate()
            .filter_map(|(index, (from, _to))| {
                remaining.find(*from).map(|position| (position, index))
            })
            .min();
        let Some((position, index)) = next else {
            updated_contents.push_str(remaining);
            break;
        };
        let Some((from, to)) = replacements.get(index) else {
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        };
        let Some(prefix) = remaining.get(..position) else {
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        };
        let Some(after_position) = remaining.get(position..) else {
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        };
        let Some(after_match) = after_position.strip_prefix(*from) else {
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        };
        updated_contents.push_str(prefix);
        updated_contents.push_str(to);
        remaining = after_match;
    }
    std::fs::write(scaffold_path_ref.get(), updated_contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_replacement_text_is_not_replaced_again() {
        let path = std::env::temp_dir()
            .join(module_path!())
            .join(std::process::id().to_string());
        std::fs::create_dir_all(path.parent().expect(constants_str::DIAGNOSTIC_32EF00B4))
            .expect(constants_str::DIAGNOSTIC_7FE212A3);
        let source = format!(
            "{}{}{}",
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_REPOSITORY_URL,
            constants_str::NEWLINE,
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_SNAKE
        );
        std::fs::write(path.as_path(), source).expect(constants_str::DIAGNOSTIC_FB14838E);
        let repository_url = format!(
            "{}{}",
            constants_str::HTTPS_SCHEME_PREFIX,
            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_SNAKE
        );
        let replacements = [
            (
                constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_REPOSITORY_URL,
                repository_url.clone(),
            ),
            (
                constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_SNAKE,
                constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE.to_owned(),
            ),
        ];
        crate::template_fs_replace_file::template_fs_replace_file(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        )
        .expect(constants_str::DIAGNOSTIC_C1DA227F);
        let updated =
            std::fs::read_to_string(path.as_path()).expect(constants_str::DIAGNOSTIC_1C4C1606);
        assert_eq!(
            updated,
            format!(
                "{}{}{}",
                repository_url,
                constants_str::NEWLINE,
                constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE
            )
        );
        let empty_pattern = [(
            constants_str::EMPTY,
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE.to_owned(),
        )];
        let invalid_result = crate::template_fs_replace_file::template_fs_replace_file(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            crate::replacements_ref::ReplacementsRef::from(empty_pattern.as_slice()),
        );
        assert!(matches!(
            invalid_result,
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
        std::fs::remove_file(path).expect(constants_str::DIAGNOSTIC_3EEC52B8);
    }

    #[test]
    fn test_missing_template_file_reports_read_error() {
        let missing_path = std::env::temp_dir().join(format!(
            "workspace-scaffold-missing-template-{}",
            std::process::id()
        ));
        let result = crate::template_fs_replace_file::template_fs_replace_file(
            crate::scaffold_path_ref::ScaffoldPathRef::from(missing_path.as_path()),
            crate::replacements_ref::ReplacementsRef::from(&[][..]),
        );
        assert!(matches!(
            result,
            Err(crate::scaffold_error::ScaffoldError::Read(
                server_runtime_http::bounded_read_error::BoundedReadError::Io { .. }
            ))
        ));
    }
}
