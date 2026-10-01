#[test]
fn test_location_macro_rejects_unexpected_arguments() {
    let executable = std::env::current_exe().expect(constants_str::DIAGNOSTIC_B7C3460F);
    let dependencies = executable
        .parent()
        .expect(constants_str::DIAGNOSTIC_603EF2FE);
    let library = std::fs::read_dir(dependencies)
        .expect(constants_str::DIAGNOSTIC_C5A8A5C7)
        .map(|entry| entry.expect(constants_str::DIAGNOSTIC_4312F5D2).path())
        .filter(|path| {
            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|name| {
                    name.starts_with(constants_str::MIGRATED_LOCATION_LIBRARY_PREFIX)
                        && name.ends_with(std::env::consts::DLL_SUFFIX)
                })
        })
        .max()
        .expect(constants_str::DIAGNOSTIC_A0AE134E);
    let external = format!(
        "{}={}",
        constants_str::MIGRATED_LOCATION_EXTERN,
        library.display()
    );
    let mut command = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(constants_str::RUSTC),
    );
    let _command = command
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::MIGRATED_RUSTC_EDITION,
        ))
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::MIGRATED_RUSTC_CRATE_TYPE,
        ))
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::MIGRATED_RUSTC_EMIT,
        ))
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::MIGRATED_RUSTC_EXTERN,
        ))
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            external.as_str(),
        ))
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::MIGRATED_LOCATION_INPUT_PATH,
        ));
    let output = command
        .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
            constants_usize::VALUE_1_048_576,
        ))
        .expect(constants_str::DIAGNOSTIC_0F1DD9C0);
    assert!(!output.status.success());
    let error =
        std::str::from_utf8(output.stderr.as_slice()).expect(constants_str::DIAGNOSTIC_202EE166);
    assert!(error.contains(constants_str::MIGRATED_LOCATION_DIAGNOSTIC));
}
