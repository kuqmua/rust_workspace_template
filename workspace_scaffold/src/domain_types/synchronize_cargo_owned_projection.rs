use super::{
    CargoArgsRef, GeneratedProjection, ScaffoldError, ScaffoldPathRef, ScaffoldRunOk, ShouldWrite,
    UpdateEnvName,
};

pub(crate) fn synchronize_cargo_owned_projection(
    root: ScaffoldPathRef<'_>,
    arguments: CargoArgsRef<'_>,
    update_environment: UpdateEnvName,
    projection: GeneratedProjection,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let mut command = macro_helpers::domain_types::tool_command::ToolCommand::new(
        macro_helpers::domain_types::tool_command::ToolProgramRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    );
    let _arguments = command
        .current_dir(macro_helpers::domain_types::tool_command::PathRef::from(
            root.get(),
        ))
        .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(arguments.get()));
    if bool::from(write_changes) {
        let _environment = command.env(
            macro_helpers::domain_types::tool_command::ToolEnvKeyRef::from(
                update_environment.get(),
            ),
            macro_helpers::domain_types::tool_command::ToolEnvValueRef::from(
                constants_str::VALUE_1,
            ),
        );
    }
    let run_ok = ScaffoldRunOk::from(command.status()?.success());
    if run_ok.get() {
        Ok(())
    } else {
        Err(match projection {
            GeneratedProjection::CodeStyle => ScaffoldError::GeneratedCodeStyle,
            GeneratedProjection::Config => ScaffoldError::GeneratedConfig,
        })
    }
}
