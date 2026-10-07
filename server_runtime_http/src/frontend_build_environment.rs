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
                crate::frontend_build_step::FrontendBuildStep::BrowserAssets
                    if cfg!(debug_assertions) =>
                {
                    (
                        constants_str::FRONTEND_TRUNK_PROGRAM,
                        &[
                            constants_str::FRONTEND_BUILD_ARGUMENT,
                            constants_str::FRONTEND_RELEASE_ARGUMENT,
                            constants_str::FRONTEND_CARGO_PROFILE_ARGUMENT,
                            constants_str::FRONTEND_DEVELOPMENT_PROFILE,
                        ][..],
                    )
                }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_frontend_environment_rejects_invalid_server_directory() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::CARGO_TOML)
            .join(constants_str::X);
        let result =
            crate::frontend_build_environment::FrontendBuildEnvironment::enter_server_directory(
                crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            );
        assert!(matches!(
            result,
            Err(crate::frontend_preparation_error::FrontendPreparationError::Environment(_))
        ));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_frontend_preparation_rejects_existing_node_path_with_path_separator() {
        let node_directory = std::env::temp_dir().join(format!(
            "{}{}{}",
            stringify!(test_frontend_preparation_rejects_existing_node_path_with_path_separator),
            constants_str::DOUBLE_COLON,
            std::process::id()
        ));
        tokio::fs::create_dir(&node_directory)
            .await
            .unwrap_or_else(|error| std::panic::panic_any(error));
        let outcome = async {
            tokio::fs::write(
                node_directory.join(constants_str::FRONTEND_NODE_PROGRAM),
                [],
            )
            .await
            .map_err(crate::service_runtime_io_error::ServiceRuntimeIoError::from)?;
            let expected = std::env::join_paths([node_directory.as_path()]);
            let frontend_build_environment =
                crate::frontend_build_environment::FrontendBuildEnvironment {
                    workspace_directory: crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                        node_directory.join(constants_str::CARGO_TOML),
                    ),
                    node_directory: Some(crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                        node_directory.clone(),
                    )),
                    search_path: crate::std_frontend_os_string::StdFrontendOsString::from(
                        std::ffi::OsString::default(),
                    ),
                };
            let actual = frontend_build_environment.prepare().await;
            Ok::<_, crate::service_runtime_io_error::ServiceRuntimeIoError>((expected, actual))
        }
        .await;
        tokio::fs::remove_dir_all(&node_directory)
            .await
            .unwrap_or_else(|error| std::panic::panic_any(error));
        let (expected, actual) = outcome.unwrap_or_else(|error| std::panic::panic_any(error));
        assert!(
            expected
                .as_ref()
                .is_err_and(|error| !error.to_string().is_empty())
        );
        assert!(actual.is_err_and(|error| {
            std::error::Error::source(&error).is_some()
                && matches!(error, crate::frontend_preparation_error::FrontendPreparationError::Environment(source)
                    if Some(source.to_string()) == expected.err().map(|native_error| native_error.to_string()))
        }));
    }

    #[tokio::test]
    async fn test_frontend_preparation_without_local_node_reports_first_command_failure() {
        let workspace =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(constants_str::CARGO_TOML);
        let frontend_build_environment =
            crate::frontend_build_environment::FrontendBuildEnvironment {
                workspace_directory: crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                    workspace,
                ),
                node_directory: None,
                search_path: crate::std_frontend_os_string::StdFrontendOsString::from(
                    std::ffi::OsString::default(),
                ),
            };
        let result = frontend_build_environment.prepare().await;
        assert!(matches!(
            result,
            Err(
                crate::frontend_preparation_error::FrontendPreparationError::Command {
                    frontend_build_step: crate::frontend_build_step::FrontendBuildStep::NodeVersion,
                    ..
                }
            )
        ));
    }

    #[tokio::test]
    async fn test_frontend_preparation_reports_node_directory_io_failure() {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let node_directory = workspace.join(constants_str::CARGO_TOML);
        let expected =
            tokio::fs::try_exists(node_directory.join(constants_str::FRONTEND_NODE_PROGRAM)).await;
        assert!(
            expected
                .as_ref()
                .is_err_and(|error| error.kind() == std::io::ErrorKind::NotADirectory)
        );
        let frontend_build_environment =
            crate::frontend_build_environment::FrontendBuildEnvironment {
                workspace_directory: crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                    workspace.to_path_buf(),
                ),
                node_directory: Some(crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                    node_directory,
                )),
                search_path: crate::std_frontend_os_string::StdFrontendOsString::from(
                    std::ffi::OsString::default(),
                ),
            };
        let result = frontend_build_environment.prepare().await;
        assert!(
            matches!(result, Err(crate::frontend_preparation_error::FrontendPreparationError::File(source)) if Some(source.to_string()) == expected.err().map(|error| error.to_string()))
        );
    }
    #[tokio::test]
    async fn test_frontend_preparation_missing_local_node_falls_back_to_command_lookup() {
        let node_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let exists =
            tokio::fs::try_exists(node_directory.join(constants_str::FRONTEND_NODE_PROGRAM)).await;
        assert!(matches!(exists, Ok(false)));
        let frontend_build_environment =
            crate::frontend_build_environment::FrontendBuildEnvironment {
                workspace_directory: crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                    node_directory.join(constants_str::CARGO_TOML),
                ),
                node_directory: Some(crate::std_frontend_path_buf::StdFrontendPathBuf::from(
                    node_directory.to_path_buf(),
                )),
                search_path: crate::std_frontend_os_string::StdFrontendOsString::from(
                    std::ffi::OsString::default(),
                ),
            };
        let result = frontend_build_environment.prepare().await;
        assert!(
            result.is_err_and(|error| std::error::Error::source(&error).is_some()
                && matches!(
                    error,
                    crate::frontend_preparation_error::FrontendPreparationError::Command {
                        frontend_build_step:
                            crate::frontend_build_step::FrontendBuildStep::NodeVersion,
                        ..
                    }
                ))
        );
    }
    #[cfg(unix)]
    #[tokio::test]
    #[ignore = "requires Node 22 or newer on PATH and /usr/bin/true and /usr/bin/false; provisions an isolated frontend fixture"]
    async fn test_frontend_preparation_cache_and_input_failures() {
        let workspace = std::env::temp_dir().join(format!(
            "{}-{}",
            constants_str::FRONTEND_PREPARATION_STARTED,
            std::process::id()
        ));
        let frontend_directory = workspace.join(constants_str::FRONTEND_DIRECTORY);
        let file_error = |source| {
            crate::frontend_preparation_error::FrontendPreparationError::File(
                crate::service_runtime_io_error::ServiceRuntimeIoError::from(source),
            )
        };
        let creation = tokio::fs::create_dir(&workspace).await;
        assert!(matches!(creation, Ok(())));
        let Ok(()) = creation else {
            return;
        };
        let outcome = async {
            tokio::fs::create_dir(&frontend_directory)
                .await
                .map_err(file_error)?;
            tokio::fs::write(
                frontend_directory.join(constants_str::FRONTEND_PACKAGE_MANIFEST),
                constants_str::FRONTEND_DEPENDENCY_FIXTURE_ONE,
            )
            .await
            .map_err(file_error)?;
            tokio::fs::write(
                frontend_directory.join(constants_str::FRONTEND_PACKAGE_LOCK),
                constants_str::FRONTEND_DEPENDENCY_FIXTURE_ONE,
            )
            .await
            .map_err(file_error)?;
            tokio::fs::create_dir(
                frontend_directory
                    .join(constants_str::FRONTEND_DEPENDENCY_STAMP)
                    .parent()
                    .ok_or_else(|| file_error(std::io::Error::other(constants_str::X)))?,
            )
            .await
            .map_err(file_error)?;
            let paths = [
                constants_str::FRONTEND_RUSTUP_PROGRAM,
                constants_str::FRONTEND_NPM_PROGRAM,
                constants_str::FRONTEND_TRUNK_PROGRAM,
            ]
            .map(|program| workspace.join(program));
            let [rustup_path, npm_path, trunk_path] = paths;
            let _links = tokio::try_join!(
                tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, rustup_path),
                tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, npm_path),
                tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, trunk_path),
            )
            .map_err(file_error)?;
            let make_environment = || {
                let mut environment =
                    crate::frontend_build_environment::FrontendBuildEnvironment::discover();
                environment.workspace_directory =
                    crate::std_frontend_path_buf::StdFrontendPathBuf::from(workspace.clone());
                let search_path = std::env::join_paths(
                    std::iter::once(workspace.clone())
                        .chain(std::env::split_paths(environment.search_path.as_ref())),
                )
                .map_err(|source| file_error(std::io::Error::other(source)))?;
                environment.search_path =
                    crate::std_frontend_os_string::StdFrontendOsString::from(search_path);
                Ok::<_, crate::frontend_preparation_error::FrontendPreparationError>(environment)
            };
            let target_command = workspace.join(constants_str::FRONTEND_RUSTUP_PROGRAM);
            tokio::fs::remove_file(&target_command).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_FALSE_EXECUTABLE_PATH, &target_command).await.map_err(file_error)?;
            let failed_target = make_environment()?.prepare().await;
            assert!(matches!(failed_target, Err(crate::frontend_preparation_error::FrontendPreparationError::Failed {
                frontend_build_step: crate::frontend_build_step::FrontendBuildStep::WasmTarget,
                ..
            })));
            let stamp = frontend_directory.join(constants_str::FRONTEND_DEPENDENCY_STAMP);
            assert!(matches!(tokio::fs::try_exists(&stamp).await, Ok(false)));
            tokio::fs::remove_file(&target_command).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, &target_command).await.map_err(file_error)?;
            make_environment()?.prepare().await?;
            let initial_stamp = crate::read_bounded_file_async::read_bounded_file_async(
                crate::runtime_path_ref::RuntimePathRef::from(stamp.as_path()),
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(128usize),
            )
            .await
            .map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)?
            .into_inner();
            assert_eq!(initial_stamp.len(), 8usize);
            tokio::fs::remove_file(workspace.join(constants_str::FRONTEND_NPM_PROGRAM))
                .await
                .map_err(file_error)?;
            tokio::fs::symlink(
                constants_str::TEST_FALSE_EXECUTABLE_PATH,
                workspace.join(constants_str::FRONTEND_NPM_PROGRAM),
            )
            .await
            .map_err(file_error)?;
            make_environment()?.prepare().await?;
            let cached_stamp = crate::read_bounded_file_async::read_bounded_file_async(
                crate::runtime_path_ref::RuntimePathRef::from(stamp.as_path()),
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(128usize),
            )
            .await
            .map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)?
            .into_inner();
            assert_eq!(cached_stamp, initial_stamp);
            tokio::fs::write(
                frontend_directory.join(constants_str::FRONTEND_PACKAGE_LOCK),
                constants_str::X,
            )
            .await
            .map_err(file_error)?;
            let changed_inputs = make_environment()?.prepare().await;
            assert!(matches!(
                changed_inputs,
                Err(
                    crate::frontend_preparation_error::FrontendPreparationError::Failed {
                        frontend_build_step:
                            crate::frontend_build_step::FrontendBuildStep::Dependencies,
                        ..
                    }
                )
            ));

            let retained_stamp = crate::read_bounded_file_async::read_bounded_file_async(
                crate::runtime_path_ref::RuntimePathRef::from(stamp.as_path()),
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(128usize),
            ).await.map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)?.into_inner();
            assert_eq!(retained_stamp, initial_stamp);
            tokio::fs::remove_file(workspace.join(constants_str::FRONTEND_NPM_PROGRAM)).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, workspace.join(constants_str::FRONTEND_NPM_PROGRAM)).await.map_err(file_error)?;
            make_environment()?.prepare().await?;
            let refreshed_stamp = crate::read_bounded_file_async::read_bounded_file_async(
                crate::runtime_path_ref::RuntimePathRef::from(stamp.as_path()),
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(128usize),
            ).await.map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)?.into_inner();
            assert_ne!(refreshed_stamp, initial_stamp);
            tokio::fs::remove_file(workspace.join(constants_str::FRONTEND_NPM_PROGRAM)).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_FALSE_EXECUTABLE_PATH, workspace.join(constants_str::FRONTEND_NPM_PROGRAM)).await.map_err(file_error)?;
            make_environment()?.prepare().await?;
            let reused_stamp = crate::read_bounded_file_async::read_bounded_file_async(
                crate::runtime_path_ref::RuntimePathRef::from(stamp.as_path()),
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(128usize),
            ).await.map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)?.into_inner();
            assert_eq!(reused_stamp, refreshed_stamp);
            tokio::fs::write(&stamp, constants_str::X.repeat(129usize)).await.map_err(file_error)?;
            let oversized_stamp = make_environment()?.prepare().await;
            assert!(matches!(oversized_stamp, Err(crate::frontend_preparation_error::FrontendPreparationError::Read(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })) if maximum_bytes.get() == 128usize));
            tokio::fs::remove_file(&stamp).await.map_err(file_error)?;
            tokio::fs::symlink(frontend_directory.join(constants_str::FRONTEND_PACKAGE_MANIFEST).join(constants_str::X), &stamp).await.map_err(file_error)?;
            let unreadable_stamp = make_environment()?.prepare().await;
            assert!(unreadable_stamp.is_err_and(|error| std::error::Error::source(&error).is_some() && matches!(error, crate::frontend_preparation_error::FrontendPreparationError::Read(crate::bounded_read_error::BoundedReadError::Io { source }) if source.kind() == std::io::ErrorKind::NotADirectory)));
            tokio::fs::write(frontend_directory.join(constants_str::FRONTEND_PACKAGE_MANIFEST), [u8::MAX]).await.map_err(file_error)?;
            let invalid_manifest = make_environment()?.prepare().await;
            assert!(matches!(invalid_manifest, Err(crate::frontend_preparation_error::FrontendPreparationError::Read(crate::bounded_read_error::BoundedReadError::Utf8 { .. }))));

            tokio::fs::write(frontend_directory.join(constants_str::FRONTEND_PACKAGE_MANIFEST), constants_str::X.repeat(1_048_577usize)).await.map_err(file_error)?;
            let oversized_manifest = make_environment()?.prepare().await;
            assert!(matches!(oversized_manifest, Err(crate::frontend_preparation_error::FrontendPreparationError::Read(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })) if maximum_bytes.get() == 1_048_576usize));
            tokio::fs::write(frontend_directory.join(constants_str::FRONTEND_PACKAGE_MANIFEST), constants_str::FRONTEND_DEPENDENCY_FIXTURE_ONE).await.map_err(file_error)?;
            tokio::fs::write(frontend_directory.join(constants_str::FRONTEND_PACKAGE_LOCK), [u8::MAX]).await.map_err(file_error)?;
            let invalid_lock = make_environment()?.prepare().await;
            assert!(matches!(invalid_lock, Err(crate::frontend_preparation_error::FrontendPreparationError::Read(crate::bounded_read_error::BoundedReadError::Utf8 { .. }))));

            tokio::fs::write(frontend_directory.join(constants_str::FRONTEND_PACKAGE_LOCK), constants_str::FRONTEND_DEPENDENCY_FIXTURE_ONE).await.map_err(file_error)?;
            tokio::fs::remove_file(workspace.join(constants_str::FRONTEND_NPM_PROGRAM)).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, workspace.join(constants_str::FRONTEND_NPM_PROGRAM)).await.map_err(file_error)?;
            tokio::fs::remove_file(&stamp).await.map_err(file_error)?;
            tokio::fs::symlink(workspace.join(constants_str::CARGO_TOML).join(constants_str::X), &stamp).await.map_err(file_error)?;
            let failed_stamp_write = make_environment()?.prepare().await;
            assert!(failed_stamp_write.is_err_and(|error| matches!(error, crate::frontend_preparation_error::FrontendPreparationError::File(_)) && std::error::Error::source(&error).is_some()));
            tokio::fs::remove_file(&stamp).await.map_err(file_error)?;
            tokio::fs::remove_file(workspace.join(constants_str::FRONTEND_TRUNK_PROGRAM)).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_FALSE_EXECUTABLE_PATH, workspace.join(constants_str::FRONTEND_TRUNK_PROGRAM)).await.map_err(file_error)?;
            let failed_browser_build = make_environment()?.prepare().await;
            assert!(matches!(failed_browser_build, Err(crate::frontend_preparation_error::FrontendPreparationError::Failed { frontend_build_step: crate::frontend_build_step::FrontendBuildStep::BrowserAssets, .. })));
            let completed_stamp = crate::read_bounded_file_async::read_bounded_file_async(crate::runtime_path_ref::RuntimePathRef::from(stamp.as_path()), crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(128usize)).await.map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)?.into_inner();
            assert_eq!(completed_stamp, initial_stamp);

            let local_node_directory = workspace.join(constants_str::FRONTEND_NODE_PROGRAM);
            tokio::fs::create_dir(&local_node_directory).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, local_node_directory.join(constants_str::FRONTEND_NODE_PROGRAM)).await.map_err(file_error)?;
            let mut local_environment = make_environment()?;
            local_environment.node_directory = Some(crate::std_frontend_path_buf::StdFrontendPathBuf::from(local_node_directory));
            let local_version = local_environment.prepare().await;
            assert!(matches!(local_version, Err(crate::frontend_preparation_error::FrontendPreparationError::NodeVersion)));
            let invalid_search_directory = workspace.join(format!("{}:{}", constants_str::X, constants_str::X));
            tokio::fs::create_dir(&invalid_search_directory).await.map_err(file_error)?;
            tokio::fs::symlink(constants_str::TEST_TRUE_EXECUTABLE_PATH, invalid_search_directory.join(constants_str::FRONTEND_NODE_PROGRAM)).await.map_err(file_error)?;
            let mut invalid_search_environment = make_environment()?;
            invalid_search_environment.node_directory = Some(crate::std_frontend_path_buf::StdFrontendPathBuf::from(invalid_search_directory));
            let invalid_search_path = invalid_search_environment.prepare().await;
            assert!(invalid_search_path.is_err_and(|error| matches!(error, crate::frontend_preparation_error::FrontendPreparationError::Environment(_)) && std::error::Error::source(&error).is_some()));
            Ok::<(), crate::frontend_preparation_error::FrontendPreparationError>(())
        }
        .await;
        let cleanup = tokio::fs::remove_dir_all(&workspace).await;
        assert!(matches!(cleanup, Ok(())));
        assert!(matches!(outcome, Ok(())));
    }
}
