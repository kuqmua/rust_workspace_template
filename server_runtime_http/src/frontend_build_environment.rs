#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct FrontendBuildEnvironment {
    workspace_directory: crate::std_frontend_path_buf::StdFrontendPathBuf,
    node_directory: Option<crate::std_frontend_path_buf::StdFrontendPathBuf>,
    search_path: crate::std_frontend_os_string::StdFrontendOsString,
}

impl FrontendBuildEnvironment {
    pub fn enter_server_directory(
        runtime_path_ref: crate::runtime_path_ref::RuntimePathRef<'_>,
    ) -> Result<(), crate::frontend_preparation_error::FrontendPreparationError> {
        std::env::set_current_dir(runtime_path_ref.get()).map_err(|source| {
            crate::frontend_preparation_error::FrontendPreparationError::Environment(
                crate::service_runtime_io_error::ServiceRuntimeIoError::from(source),
            )
        })
    }

    #[must_use]
    pub fn discover() -> Self {
        let node_directory = std::env::var_os(constants_str::FRONTEND_NODE_BIN_ENV)
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os(constants_str::FRONTEND_HOME_ENV).map(|value| {
                    std::path::PathBuf::from(value)
                        .join(constants_str::FRONTEND_LOCAL_NODE_DIRECTORY)
                })
            })
            .map(crate::std_frontend_path_buf::StdFrontendPathBuf::from);
        Self {
            workspace_directory: crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join(constants_str::FRONTEND_PARENT_DIRECTORY),
            ),
            node_directory,
            search_path: crate::std_frontend_os_string::StdFrontendOsString::from(
                std::env::var_os(constants_str::PATH_ALT).unwrap_or_default(),
            ),
        }
    }

    pub async fn prepare(
        self,
    ) -> Result<(), crate::frontend_preparation_error::FrontendPreparationError> {
        let file_error = |source| {
            crate::frontend_preparation_error::FrontendPreparationError::File(
                crate::service_runtime_io_error::ServiceRuntimeIoError::from(source),
            )
        };
        let frontend_directory = self
            .workspace_directory
            .as_ref()
            .join(constants_str::FRONTEND_DIRECTORY);
        let local_node = match self.node_directory.as_ref() {
            Some(directory) => {
                let path = directory
                    .as_ref()
                    .join(constants_str::FRONTEND_NODE_PROGRAM);
                tokio::fs::try_exists(&path)
                    .await
                    .map_err(file_error)?
                    .then_some(path)
            }
            None => None,
        };
        let search_path = std::env::join_paths(
            local_node
                .as_ref()
                .and_then(|path| path.parent())
                .into_iter()
                .map(std::path::Path::to_path_buf)
                .chain(std::env::split_paths(self.search_path.as_ref())),
        )
        .map_err(|source| {
            crate::frontend_preparation_error::FrontendPreparationError::Environment(
                crate::service_runtime_io_error::ServiceRuntimeIoError::from(
                    std::io::Error::other(source),
                ),
            )
        })?;
        let make_command = |frontend_build_step: crate::frontend_build_step::FrontendBuildStep| {
            let (program, arguments) = match frontend_build_step {
                crate::frontend_build_step::FrontendBuildStep::NodeVersion => (
                    constants_str::FRONTEND_NODE_PROGRAM,
                    &[constants_str::FRONTEND_VERSION_ARGUMENT][..],
                ),
                crate::frontend_build_step::FrontendBuildStep::WasmTarget => (
                    constants_str::FRONTEND_RUSTUP_PROGRAM,
                    &[
                        constants_str::TARGET,
                        constants_str::FRONTEND_ADD_ARGUMENT,
                        constants_str::FRONTEND_WASM_TARGET,
                    ][..],
                ),
                crate::frontend_build_step::FrontendBuildStep::Dependencies => (
                    constants_str::FRONTEND_NPM_PROGRAM,
                    &[constants_str::FRONTEND_CI_ARGUMENT][..],
                ),
                crate::frontend_build_step::FrontendBuildStep::BrowserAssets => (
                    constants_str::FRONTEND_TRUNK_PROGRAM,
                    &[
                        constants_str::FRONTEND_BUILD_ARGUMENT,
                        constants_str::FRONTEND_RELEASE_ARGUMENT,
                    ][..],
                ),
            };
            let mut command = tokio::process::Command::new(program);
            let _configured_command = command
                .args(arguments)
                .current_dir(&frontend_directory)
                .env(constants_str::PATH_ALT, &search_path)
                .env(constants_str::FRONTEND_NO_COLOR_ENV, constants_str::TRUE)
                .env(
                    constants_str::FRONTEND_CARGO_TARGET_ENV,
                    self.workspace_directory
                        .as_ref()
                        .join(constants_str::FRONTEND_TARGET_DIRECTORY),
                )
                .env(
                    constants_str::FRONTEND_CARGO_BUILD_ENV,
                    self.workspace_directory
                        .as_ref()
                        .join(constants_str::FRONTEND_BUILD_DIRECTORY),
                );
            crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(command)
        };
        let node_version = make_command(crate::frontend_build_step::FrontendBuildStep::NodeVersion)
            .node_version()
            .await?;
        crate::validate_frontend_node_version::validate_frontend_node_version(&node_version)?;
        make_command(crate::frontend_build_step::FrontendBuildStep::WasmTarget)
            .run(crate::frontend_build_step::FrontendBuildStep::WasmTarget)
            .await?;
        let read_text =
            async |std_frontend_path_buf: crate::std_frontend_path_buf::StdFrontendPathBuf| {
                let bytes = crate::read_bounded_file_async::read_bounded_file_async(
                    crate::runtime_path_ref::RuntimePathRef::from(std_frontend_path_buf.as_ref()),
                    crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        1_048_576usize,
                    ),
                )
                .await
                .map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)?;
                crate::bounded_text::BoundedText::try_from(bytes)
                    .map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)
            };
        let inputs = crate::frontend_dependency_inputs::FrontendDependencyInputs::new(
            read_text(crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                frontend_directory.join(constants_str::FRONTEND_PACKAGE_MANIFEST),
            ))
            .await?,
            read_text(crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                frontend_directory.join(constants_str::FRONTEND_PACKAGE_LOCK),
            ))
            .await?,
            node_version,
        );
        let fingerprint = inputs.fingerprint();
        let stamp = frontend_directory.join(constants_str::FRONTEND_DEPENDENCY_STAMP);
        let installed = match crate::read_bounded_file_async::read_bounded_file_async(
            crate::runtime_path_ref::RuntimePathRef::from(stamp.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(128usize),
        )
        .await
        {
            Ok(bytes) => bytes.into_inner().as_slice() == fingerprint.get(),
            Err(crate::bounded_read_error::BoundedReadError::Io { source })
                if source.kind() == std::io::ErrorKind::NotFound =>
            {
                false
            }
            Err(error) => {
                return Err(
                    crate::frontend_preparation_error::FrontendPreparationError::Read(error),
                );
            }
        };
        if !installed {
            make_command(crate::frontend_build_step::FrontendBuildStep::Dependencies)
                .run(crate::frontend_build_step::FrontendBuildStep::Dependencies)
                .await?;
            tokio::fs::write(stamp, fingerprint.get())
                .await
                .map_err(file_error)?;
        }
        tracing::info!(message = %constants_str::FRONTEND_PREPARATION_STARTED);
        make_command(crate::frontend_build_step::FrontendBuildStep::BrowserAssets)
            .run(crate::frontend_build_step::FrontendBuildStep::BrowserAssets)
            .await?;
        tracing::info!(message = %constants_str::FRONTEND_PREPARATION_COMPLETED);
        Ok(())
    }
}
