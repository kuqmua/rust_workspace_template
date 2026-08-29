pub(crate) fn write_content(
    path: crate::InitPathRef<'_>,
    content: crate::EnvContentRef<'_>,
) -> Result<(), crate::InitializeError> {
    std::fs::write(path.get(), content.as_ref().as_bytes()).map_err(|source| {
        crate::InitializeError::WriteEnvironment {
            source: source.into(),
        }
    })
}
