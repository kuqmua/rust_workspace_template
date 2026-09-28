#![allow(
    unused_crate_dependencies,
    reason = "binary integration test uses only the process wrapper from the package dependency catalog"
)]

#[cfg(test)]
mod tests {
    #[test]
    fn test_oversized_mode_reports_length_error_without_unknown_mode_fallback() {
        let mode = constants_str::X.repeat(1_025usize);
        let mut command = macro_helpers::tool_command::ToolCommand::new(
            macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                "CARGO_BIN_EXE_workspace_test_runner"
            )),
        );
        let _command = command.arg(macro_helpers::tool_arg_ref::ToolArgRef::from(mode.as_str()));
        let result = command.bounded_output(
            macro_helpers::tool_output_limit::ToolOutputLimit::from(2_048usize),
        );
        assert!(result.is_ok_and(|output| {
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            !output.status.success()
                && stderr.contains(mode.len().to_string().as_str())
                && !stderr.contains(constants_str::RUNNER_CLI_TEXT_A8DCE6DA)
                && output.stdout.is_empty()
        }));
    }
}
