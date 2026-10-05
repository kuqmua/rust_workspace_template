#[test]
fn test_source_place_environment_processes_preserve_dotenv_environment_and_default_behavior() {
    let package_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(package_directory.parent().is_some());
    let Some(workspace_directory) = package_directory.parent() else {
        return;
    };
    let mut cargo = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    );
    let cargo_arguments = [
        constants_str::SOURCE_PLACE_TEST_CARGO_TEST_ARGUMENT,
        constants_str::SOURCE_PLACE_TEST_PACKAGE_ARGUMENT,
        constants_str::SOURCE_PLACE_TEST_PACKAGE_NAME,
        constants_str::SOURCE_PLACE_TEST_LIB_ARGUMENT,
        constants_str::SOURCE_PLACE_TEST_NO_RUN_ARGUMENT,
        constants_str::SOURCE_PLACE_TEST_JSON_ARGUMENT,
    ];
    let _configured_cargo = cargo
        .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
            cargo_arguments.as_slice(),
        ))
        .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
            workspace_directory,
        ));
    let compilation_result = cargo.bounded_output(
        macro_helpers::tool_output_limit::ToolOutputLimit::from(constants_usize::VALUE_16_777_216),
    );
    assert!(
        compilation_result
            .as_ref()
            .is_ok_and(|output| output.status.success())
    );
    let Ok(compilation_output) = compilation_result else {
        return;
    };
    let compilation_text_result = std::str::from_utf8(compilation_output.stdout.as_slice());
    assert!(
        compilation_text_result
            .as_ref()
            .is_ok_and(|text| !text.is_empty())
    );
    let Ok(compilation_text) = compilation_text_result else {
        return;
    };
    let artifacts_result = compilation_text
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>();
    assert!(
        artifacts_result
            .as_ref()
            .is_ok_and(|artifacts| !artifacts.is_empty())
    );
    let Ok(artifacts) = artifacts_result else {
        return;
    };
    let executable_option = artifacts.iter().find_map(|artifact| {
        let target_name = artifact
            .get(constants_str::SOURCE_PLACE_TEST_ARTIFACT_TARGET_KEY)
            .and_then(|target| target.get(constants_str::SOURCE_PLACE_TEST_ARTIFACT_NAME_KEY))
            .and_then(serde_json::Value::as_str);
        let test_profile = artifact
            .get(constants_str::SOURCE_PLACE_TEST_ARTIFACT_PROFILE_KEY)
            .and_then(|profile| profile.get(constants_str::SOURCE_PLACE_TEST_CARGO_TEST_ARGUMENT))
            .and_then(serde_json::Value::as_bool);
        if target_name == Some(constants_str::SOURCE_PLACE_TEST_PACKAGE_NAME)
            && test_profile == Some(true)
        {
            artifact
                .get(constants_str::SOURCE_PLACE_TEST_ARTIFACT_EXECUTABLE_KEY)
                .and_then(serde_json::Value::as_str)
        } else {
            None
        }
    });
    assert!(executable_option.is_some());
    let Some(executable) = executable_option else {
        return;
    };
    let mut directory_name =
        std::ffi::OsString::from(constants_str::SOURCE_PLACE_TEST_DIRECTORY_STEM);
    directory_name.push(std::process::id().to_string());
    let directory = std::env::temp_dir().join(directory_name);
    assert!(matches!(directory.try_exists(), Ok(false)));
    let created = std::fs::create_dir_all(&directory);
    assert!(matches!(created, Ok(())));
    let Ok(()) = created else {
        return;
    };
    let dotenv_path = directory.join(constants_str::ENV);
    let mut oversized_dotenv =
        String::from(constants_str::SOURCE_PLACE_TEST_DOTENV_ASSIGNMENT_PREFIX);
    oversized_dotenv.push_str(
        &constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE),
    );
    let outcomes_result = [
        (
            Some(constants_str::SRC_ALT),
            String::new(),
            constants_str::SRC_ALT,
        ),
        (
            Some(constants_str::GITHUB_ALT),
            String::new(),
            constants_str::GITHUB_ALT,
        ),
        (
            Some(constants_str::X),
            String::new(),
            constants_str::GITHUB_ALT,
        ),
        (
            None,
            String::from(constants_str::SOURCE_PLACE_TEST_DOTENV_SRC),
            constants_str::SRC_ALT,
        ),
        (None, String::new(), constants_str::GITHUB_ALT),
        (
            Some(constants_str::SRC_ALT),
            String::from(constants_str::SOURCE_PLACE_TEST_DOTENV_MALFORMED),
            constants_str::SRC_ALT,
        ),
        (None, oversized_dotenv, constants_str::GITHUB_ALT),
    ]
    .into_iter()
    .try_fold(true, |all_passed, (source_option, contents, expected)| {
        std::fs::write(&dotenv_path, contents)?;
        let mut command = macro_helpers::tool_command::ToolCommand::new(
            macro_helpers::tool_program_ref::ToolProgramRef::from(if source_option.is_some() {
                executable
            } else {
                constants_str::SOURCE_PLACE_TEST_ENV_PROGRAM
            }),
        );
        if source_option.is_none() {
            let unset_arguments = [
                constants_str::SOURCE_PLACE_TEST_UNSET_ARGUMENT,
                constants_str::ENV_NAMES_SOURCE_PLACE_TYPE,
                executable,
            ];
            let _configured_env_program = command.args(
                macro_helpers::tool_args_ref::ToolArgsRef::from(unset_arguments.as_slice()),
            );
        }
        let child_arguments = [
            constants_str::SOURCE_PLACE_TEST_EXACT_ARGUMENT,
            constants_str::SOURCE_PLACE_TEST_CHILD_FILTER,
            constants_str::SOURCE_PLACE_TEST_IGNORED_ARGUMENT,
        ];
        let _configured_child = command
            .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
                child_arguments.as_slice(),
            ))
            .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
                directory.as_path(),
            ))
            .env(
                macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(
                    constants_str::SOURCE_PLACE_TEST_EXPECTED_ENV_KEY,
                ),
                macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(expected),
            );
        if let Some(source_value) = source_option {
            let _configured_source = command.env(
                macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(
                    constants_str::ENV_NAMES_SOURCE_PLACE_TYPE,
                ),
                macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(source_value),
            );
        }
        command
            .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                16_384usize,
            ))
            .map(|output| {
                all_passed
                    && output.status.success()
                    && String::from_utf8_lossy(output.stdout.as_slice())
                        .contains(constants_str::SOURCE_PLACE_TEST_SUMMARY)
            })
    });
    let removed_file = std::fs::remove_file(dotenv_path);
    let removed_directory = std::fs::remove_dir_all(directory);
    assert!(matches!(removed_file, Ok(())));
    assert!(matches!(removed_directory, Ok(())));
    assert!(outcomes_result.as_ref().is_ok_and(|all_passed| *all_passed));
}
