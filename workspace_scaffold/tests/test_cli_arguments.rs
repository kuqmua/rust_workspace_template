#![allow(
    unused_crate_dependencies,
    reason = "the CLI integration target uses the shared process adapter and constants while executable dependencies belong to the binary"
)]

#[cfg(test)]
mod tests {
    #[test]
    fn test_scaffold_cli_rejects_invalid_arguments_before_workspace_writes() {
        let rejects = |tool_args_ref: macro_helpers::tool_args_ref::ToolArgsRef<'_>| {
            macro_helpers::tool_command::ToolCommand::new(
                macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                    "CARGO_BIN_EXE_workspace-scaffold"
                )),
            )
            .args(tool_args_ref)
            .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                8192usize,
            ))
            .is_ok_and(|output| output.status.code() == Some(2i32) && output.stdout.is_empty())
        };
        assert!(rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
            &[][..]
        )));
        assert!(rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
            &[constants_str::X][..]
        )));
        assert!(
            [
                constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND,
                constants_str::SERVICE,
                constants_str::WORKSPACE_SCAFFOLD_MANIFEST_COMMAND,
                constants_str::WORKSPACE_SCAFFOLD_MANIFEST_EXAMPLE_COMMAND,
                constants_str::VALUE_24CACF50,
                constants_str::VALUE_AEE50B18,
            ]
            .into_iter()
            .all(
                |command| rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                    &[command][..]
                ))
            )
        );
        assert!(
            [
                constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND,
                constants_str::SERVICE,
            ]
            .into_iter()
            .all(|command| {
                rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                    &[command, constants_str::X][..],
                )) && rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                    &[
                        command,
                        constants_str::X,
                        constants_str::VALUE_1,
                        constants_str::X,
                    ][..],
                ))
            })
        );
        assert!(
            [
                constants_str::WORKSPACE_SCAFFOLD_MANIFEST_COMMAND,
                constants_str::WORKSPACE_SCAFFOLD_MANIFEST_EXAMPLE_COMMAND,
            ]
            .into_iter()
            .all(
                |command| rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                    &[command, constants_str::X, constants_str::X][..]
                ))
            )
        );
        assert!(
            [constants_str::VALUE_24CACF50, constants_str::VALUE_AEE50B18,]
                .into_iter()
                .all(|command| {
                    rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                        &[command, constants_str::X][..],
                    )) && [constants_str::SYNC, constants_str::CHECK]
                        .into_iter()
                        .all(|mode| {
                            rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                                &[command, mode, constants_str::X][..],
                            ))
                        })
                })
        );
        assert!(
            [constants_str::EMPTY, constants_str::PARENT_PATH_SEGMENT]
                .into_iter()
                .all(|name| {
                    rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                        &[
                            constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND,
                            name,
                            constants_str::VALUE_A680FDEF,
                        ][..],
                    )) && rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                        &[constants_str::SERVICE, name, constants_str::VALUE_1][..],
                    ))
                })
        );
        assert!(
            [constants_str::X, constants_str::VALUE_861AC68D]
                .into_iter()
                .all(
                    |repository| rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                        &[
                            constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND,
                            constants_str::X,
                            repository
                        ][..]
                    ))
                )
        );
        let overflowing_port = u32::from(u16::MAX).saturating_add(1u32).to_string();
        assert!(
            [
                constants_str::VALUE_0,
                constants_str::X,
                overflowing_port.as_str()
            ]
            .into_iter()
            .all(
                |port| rejects(macro_helpers::tool_args_ref::ToolArgsRef::from(
                    &[constants_str::SERVICE, constants_str::X, port][..]
                ))
            )
        );
    }

    #[test]
    fn test_scaffold_cli_validates_manifest_files_without_modifying_them() {
        let path = std::env::temp_dir().join(format!(
            "{}-{}",
            constants_str::WORKSPACE_SCAFFOLD_MANIFEST_COMMAND,
            std::process::id()
        ));
        let invoke = |tool_arg_ref: macro_helpers::tool_arg_ref::ToolArgRef<'_>| {
            macro_helpers::tool_command::ToolCommand::new(
                macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                    "CARGO_BIN_EXE_workspace-scaffold"
                )),
            )
            .arg(tool_arg_ref)
            .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
                path.to_string_lossy().as_ref(),
            ))
            .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                8192usize,
            ))
        };
        [
            (constants_str::PRODUCTION_MANIFEST_VALID_TEST, 0i32, 2i32),
            (constants_str::PRODUCTION_MANIFEST_EXAMPLE_TEST, 2i32, 0i32),
            (constants_str::EMPTY, 2i32, 2i32),
        ]
        .into_iter()
        .fold((), |(), (contents, manifest_status, example_status)| {
            assert!(std::fs::write(&path, contents).is_ok_and(|()| path.is_file()));
            [
                (
                    constants_str::WORKSPACE_SCAFFOLD_MANIFEST_COMMAND,
                    manifest_status,
                ),
                (
                    constants_str::WORKSPACE_SCAFFOLD_MANIFEST_EXAMPLE_COMMAND,
                    example_status,
                ),
            ]
            .into_iter()
            .fold((), |(), (command, status)| {
                assert!(
                    invoke(macro_helpers::tool_arg_ref::ToolArgRef::from(command)).is_ok_and(
                        |output| output.status.code() == Some(status)
                            && output.stdout.is_empty()
                            && output.stderr.is_empty() == (status == 0i32)
                    )
                );
                assert!(std::fs::read(&path).is_ok_and(|bytes| bytes == contents.as_bytes()));
            });
        });
        assert!(std::fs::write(&path, [u8::MAX]).is_ok_and(|()| path.is_file()));
        [
            constants_str::WORKSPACE_SCAFFOLD_MANIFEST_COMMAND,
            constants_str::WORKSPACE_SCAFFOLD_MANIFEST_EXAMPLE_COMMAND,
        ]
        .into_iter()
        .fold((), |(), command| {
            assert!(
                invoke(macro_helpers::tool_arg_ref::ToolArgRef::from(command)).is_ok_and(
                    |output| output.status.code() == Some(2i32)
                        && output.stdout.is_empty()
                        && output.stderr.is_empty()
                )
            );
            assert!(std::fs::read(&path).is_ok_and(|bytes| bytes == [u8::MAX]));
        });
        assert!(std::fs::remove_file(&path).is_ok_and(|()| !path.exists()));
        [
            constants_str::WORKSPACE_SCAFFOLD_MANIFEST_COMMAND,
            constants_str::WORKSPACE_SCAFFOLD_MANIFEST_EXAMPLE_COMMAND,
        ]
        .into_iter()
        .fold((), |(), command| {
            assert!(
                invoke(macro_helpers::tool_arg_ref::ToolArgRef::from(command)).is_ok_and(
                    |output| output.status.code() == Some(2i32)
                        && output.stdout.is_empty()
                        && output.stderr.is_empty()
                )
            );
            assert!(!path.exists());
        });
    }
}
