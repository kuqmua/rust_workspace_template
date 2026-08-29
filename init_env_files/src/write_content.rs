pub(crate) fn write_content(
    path: crate::init_path_ref::InitPathRef<'_>,
    content: crate::env_content_ref::EnvContentRef<'_>,
) -> Result<(), crate::initialize_error::InitializeError> {
    std::fs::write(path.get(), content.as_ref().as_bytes()).map_err(|source| {
        crate::initialize_error::InitializeError::WriteEnvironment {
            source: source.into(),
        }
    })
}
