pub(crate) fn synchronize_cargo_owned_projection(
    root: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    arguments: crate::cargo_args_ref::CargoArgsRef<'_>,
    update_environment: crate::update_env_name::UpdateEnvName,
    projection: crate::generated_projection::GeneratedProjection,
    write_changes: crate::should_write::ShouldWrite,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let mut command = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(
            constants_str::catalog::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    );
    let _arguments = command
        .current_dir(macro_helpers::path_ref::PathRef::from(root.get()))
        .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
            arguments.get(),
        ));
    if bool::from(write_changes) {
        let _environment = command.env(
            macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(update_environment.get()),
            macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(
                constants_str::catalog::VALUE_1,
            ),
        );
    }
    let run_ok = crate::scaffold_run_ok::ScaffoldRunOk::from(command.status()?.success());
    if run_ok.get() {
        Ok(())
    } else {
        Err(match projection {
            crate::generated_projection::GeneratedProjection::CodeStyle => {
                crate::scaffold_error::ScaffoldError::GeneratedCodeStyle
            }
            crate::generated_projection::GeneratedProjection::Config => {
                crate::scaffold_error::ScaffoldError::GeneratedConfig
            }
        })
    }
}
