pub(crate) fn validate_project_name(
    value: super::ProjectNameRef<'_>,
) -> Result<(), super::ScaffoldError> {
    let text = value.0;
    if text.is_empty()
        || text.starts_with('_')
        || text.ends_with('_')
        || text.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE)
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(super::ScaffoldError::ProjectName);
    }
    Ok(())
}
