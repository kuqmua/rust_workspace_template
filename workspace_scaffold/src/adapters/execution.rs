#[allow(
    clippy::single_call_fn,
    reason = "process adapter owns cargo command construction and execution"
)]
pub(crate) fn run_cargo(
    root: crate::domain_types::ScaffoldPathRef<'_>,
    arguments: crate::domain_types::CargoArgsRef<'_>,
    update_environment: Option<crate::domain_types::UpdateEnvName>,
) -> Result<crate::domain_types::ScaffoldRunOk, crate::domain_types::ScaffoldError> {
    let mut command = macro_helpers::domain_types::tool_command::ToolCommand::new(
        macro_helpers::domain_types::tool_command::ToolProgramRef::from("cargo"),
    );
    let _arguments = command
        .current_dir(macro_helpers::domain_types::tool_command::PathRef::from(
            root.get(),
        ))
        .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(arguments.get()));
    if let Some(environment) = update_environment {
        let _environment = command.env(
            macro_helpers::domain_types::tool_command::ToolEnvKeyRef::from(environment.get()),
            macro_helpers::domain_types::tool_command::ToolEnvValueRef::from("1"),
        );
    }
    Ok(crate::domain_types::ScaffoldRunOk::from(
        command.status()?.success(),
    ))
}
