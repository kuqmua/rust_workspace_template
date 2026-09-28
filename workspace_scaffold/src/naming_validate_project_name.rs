pub(crate) fn naming_validate_project_name(
    project_name_ref: crate::project_name_ref::ProjectNameRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let text = project_name_ref.get();
    if text.len() > constants_usize::VALUE_16_777_216
        || !text.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        || text.ends_with('_')
        || text.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE)
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(crate::scaffold_error::ScaffoldError::ProjectName);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_project_name_rejects_length_beyond_scaffold_text_bound() {
        let oversized = std::iter::repeat_n(
            'a',
            constants_usize::VALUE_16_777_216.saturating_add(constants_usize::ONE),
        )
        .collect::<String>();
        let name = crate::project_name_ref::ProjectNameRef::from(oversized.as_str());
        assert!(matches!(
            crate::naming_validate_project_name::naming_validate_project_name(name),
            Err(crate::scaffold_error::ScaffoldError::ProjectName)
        ));
        assert!(matches!(
            crate::naming_kebab_case::naming_kebab_case(name),
            Err(crate::scaffold_error::ScaffoldError::ProjectName)
        ));
        assert!(matches!(
            crate::naming_title_case::naming_title_case(name),
            Err(crate::scaffold_error::ScaffoldError::ProjectName)
        ));
        assert!(matches!(
            crate::naming_upper_camel_case::naming_upper_camel_case(name),
            Err(crate::scaffold_error::ScaffoldError::ProjectName)
        ));
    }

    #[test]
    fn test_project_name_rejects_leading_digit() {
        let name = crate::project_name_ref::ProjectNameRef::from(
            constants_str::WORKSPACE_SCAFFOLD_NUMERIC_PREFIX_NAME,
        );
        assert!(matches!(
            crate::naming_validate_project_name::naming_validate_project_name(name),
            Err(crate::scaffold_error::ScaffoldError::ProjectName)
        ));
    }
}
