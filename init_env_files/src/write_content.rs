pub(crate) fn write_content(
    path: crate::domain_types::InitPathRef<'_>,
    content: crate::domain_types::EnvContentRef<'_>,
) -> Result<(), crate::domain_types::InitializeError> {
    std::fs::write(path.get(), content.as_ref().as_bytes()).map_err(|source| {
        crate::domain_types::InitializeError::WriteEnvironment {
            source: source.into(),
        }
    })
}
