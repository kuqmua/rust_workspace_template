pub(crate) fn write_content(
    init_path_ref: crate::init_path_ref::InitPathRef<'_>,
    env_content_ref: crate::env_content_ref::EnvContentRef<'_>,
) -> Result<(), crate::initialize_error::InitializeError> {
    std::fs::write(init_path_ref.get(), env_content_ref.as_ref().as_bytes()).map_err(|source| {
        crate::initialize_error::InitializeError::WriteEnvironment {
            source: source.into(),
        }
    })
}
