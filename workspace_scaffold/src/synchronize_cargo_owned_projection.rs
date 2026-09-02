pub(crate) fn synchronize_cargo_owned_projection(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    cargo_args_ref: crate::cargo_args_ref::CargoArgsRef<'_>,
    update_env_name: crate::update_env_name::UpdateEnvName,
    generated_projection: crate::generated_projection::GeneratedProjection,
    should_write: crate::should_write::ShouldWrite,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let mut command = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    );
    let _arguments = command
        .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
            scaffold_path_ref.get(),
        ))
        .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
            cargo_args_ref.get(),
        ));
    if bool::from(should_write) {
        let _environment = command.env(
            macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(update_env_name.get()),
            macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(constants_str::VALUE_1),
        );
    }
    let run_ok = crate::scaffold_run_ok::ScaffoldRunOk::from(command.status()?.success());
    if run_ok.get() {
        Ok(())
    } else {
        Err(match generated_projection {
            crate::generated_projection::GeneratedProjection::CodeStyle => {
                crate::scaffold_error::ScaffoldError::GeneratedCodeStyle
            }
            crate::generated_projection::GeneratedProjection::Config => {
                crate::scaffold_error::ScaffoldError::GeneratedConfig
            }
        })
    }
}
