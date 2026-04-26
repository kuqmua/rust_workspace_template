use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use test_helpers as _;

fn collect_files_recursively(directory_path: &Path, files: &mut Vec<PathBuf>) {
    let directory_entries = fs::read_dir(directory_path).expect("f1a27b8c");

    for directory_entry_result in directory_entries {
        let directory_entry = directory_entry_result.expect("d2e8c41a");
        let file_type = directory_entry.file_type().expect("8bf903de");
        let path = directory_entry.path();

        if file_type.is_dir() {
            if path.ends_with(".git") || path.ends_with("target") {
                continue;
            }
            collect_files_recursively(&path, files);
            continue;
        }

        files.push(path);
    }
}

fn collect_workspace_files(workspace_root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files_recursively(workspace_root, &mut files);
    files
}

fn workspace_root_path() -> PathBuf {
    let crate_manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_manifest_directory
        .parent()
        .expect("a74d10bf")
        .to_path_buf()
}

fn non_test_source_segment(file_content: &str) -> &str {
    if let Some((before_test_segment, _after_test_segment)) =
        file_content.split_once("#[cfg(test)]")
    {
        return before_test_segment;
    }

    file_content
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("9be35a17")
}

fn workflow_file_paths(workspace_root: &Path) -> Vec<PathBuf> {
    let workflows_directory_path = workspace_root.join(".github").join("workflows");
    fs::read_dir(&workflows_directory_path)
        .and_then(|directory_entries| {
            directory_entries
                .map(|directory_entry_result| directory_entry_result.map(|entry| entry.path()))
                .collect()
        })
        .or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                Ok(Vec::new())
            } else {
                Err(error)
            }
        })
        .expect("7cf34bd1")
        .into_iter()
        .filter(|path| path.extension().is_some_and(|extension| extension == "yml"))
        .collect()
}

fn rust_source_files(workspace_files: &[PathBuf]) -> Vec<&PathBuf> {
    workspace_files
        .iter()
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("rs"))
        .collect()
}

#[cfg(test)]
mod policy_tests {
    use std::path::PathBuf;

    use super::{
        collect_workspace_files, fs, non_test_source_segment, read_file, rust_source_files,
        workflow_file_paths, workspace_root_path,
    };

    #[test]
    fn forbids_todo_and_unimplemented_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("todo!("),
                "found todo! in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("unimplemented!("),
                "found unimplemented! in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_panic_and_assert_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("panic!("),
                "found panic! in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("assert!("),
                "found assert! in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_source_dropping_map_err_pattern() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("map_err(|_|"),
                "found source-dropping map_err pattern in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_global_mutable_or_lazy_singleton_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("static mut"),
                "found static mut in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("lazy_static!"),
                "found lazy_static singleton in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("once_cell::sync::Lazy"),
                "found once_cell::sync::Lazy singleton in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("LazyLock::new("),
                "found LazyLock singleton in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_deterministic_test_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if !rust_file
                .components()
                .any(|component| component.as_os_str() == "tests")
            {
                continue;
            }
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }

            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("sleep("),
                "found sleep in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("SystemTime::now("),
                "found wall-clock time in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("Utc::now("),
                "found timezone-dependent call in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("Local::now("),
                "found local-time call in tests: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn requires_contract_and_round_trip_tests_for_shared_logic() {
        let workspace_root = workspace_root_path();
        let integration_test_path = workspace_root
            .join("shared_logic")
            .join("tests")
            .join("integration_baseline.rs");
        let file_content = read_file(&integration_test_path);

        assert!(
            file_content.contains("evaluates_public_api_contract"),
            "missing public API contract test in {}",
            integration_test_path.display()
        );
        assert!(
            file_content.contains("preserves_value_on_round_trip"),
            "missing round-trip test in {}",
            integration_test_path.display()
        );
    }

    #[test]
    fn requires_property_based_tests_for_shared_logic_invariants() {
        let workspace_root = workspace_root_path();
        let property_test_path = workspace_root
            .join("shared_logic")
            .join("tests")
            .join("property_invariants.rs");
        let property_test_content = read_file(&property_test_path);

        assert!(
            property_test_content.contains("proptest!"),
            "missing proptest! test module in {}",
            property_test_path.display()
        );
        assert!(
            property_test_content.contains("wire_format_round_trip_preserves_request"),
            "missing round-trip property test in {}",
            property_test_path.display()
        );
        assert!(
            property_test_content.contains("addition_commutativity_holds_for_safe_operand_range"),
            "missing domain invariant property test in {}",
            property_test_path.display()
        );
        assert!(
            property_test_content
                .contains("subtraction_is_inverse_of_addition_for_safe_operand_range"),
            "missing additive-inverse property test in {}",
            property_test_path.display()
        );
    }

    #[test]
    fn requires_trybuild_compile_fail_contract_for_shared_logic() {
        let workspace_root = workspace_root_path();
        let compile_fail_contract_path = workspace_root
            .join("shared_logic")
            .join("tests")
            .join("compile_fail_contracts.rs");
        let compile_fail_contract_content = read_file(&compile_fail_contract_path);
        let compile_fail_case_path = workspace_root
            .join("shared_logic")
            .join("tests")
            .join("ui")
            .join("arithmetic_operation_type_contract_rejects_boolean_argument.rs");

        assert!(
            compile_fail_contract_content.contains("trybuild::TestCases::new"),
            "missing trybuild test harness in {}",
            compile_fail_contract_path.display()
        );
        assert!(
            compile_fail_contract_content.contains("compile_fail"),
            "missing compile_fail invocation in {}",
            compile_fail_contract_path.display()
        );
        assert!(
            compile_fail_case_path.exists(),
            "missing compile-fail case source file: {}",
            compile_fail_case_path.display()
        );
    }

    #[test]
    fn requires_semver_check_and_hack_in_ci_and_changelog_file() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content.contains("cargo-semver-checks-action"),
            "missing semver check action in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content.contains("cargo hack check"),
            "missing cargo hack feature-matrix check in {}",
            ci_workflow_path.display()
        );

        let changelog_path = workspace_root.join("CHANGELOG.md");
        let changelog_content = read_file(&changelog_path);
        assert!(
            !changelog_content.trim().is_empty(),
            "CHANGELOG.md exists but is empty: {}",
            changelog_path.display()
        );
    }

    #[test]
    fn enforces_server_cli_tests_to_use_shared_command_helpers() {
        let workspace_root = workspace_root_path();
        let server_tests_directory_path = workspace_root.join("server").join("tests");
        let server_test_paths = fs::read_dir(&server_tests_directory_path)
            .expect("9f2b1c7d")
            .map(|directory_entry_result| directory_entry_result.expect("4d1a8c7e").path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
            .collect::<Vec<PathBuf>>();

        for server_test_path in server_test_paths {
            let server_test_content = read_file(&server_test_path);
            assert!(
                server_test_content.contains("run_server_command(")
                    || server_test_content.contains("run_server_command_with_report_format("),
                "server cli tests must use shared command helpers in {}",
                server_test_path.display()
            );
            assert!(
                !server_test_content.contains("Command::new("),
                "server cli tests must not invoke Command::new directly in {}",
                server_test_path.display()
            );
        }
    }

    #[test]
    fn forbids_unwrap_and_error_masking_shortcuts_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("unwrap("),
                "found unwrap in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("unwrap_or_default("),
                "found unwrap_or_default in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("unwrap_or("),
                "found unwrap_or in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_expect_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("expect("),
                "found expect in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_direct_command_new_outside_test_helpers_command_wrappers() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);
        let command_wrapper_source_path = workspace_root
            .join("test_helpers")
            .join("src")
            .join("lib.rs");

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }
            if rust_file == &command_wrapper_source_path {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("Command::new("),
                "direct Command::new usage is forbidden outside test_helpers command wrappers: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_allow_lint_attributes_in_rust_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("#[allow("),
                "found #[allow(...)] attribute in {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("#![allow("),
                "found #![allow(...)] attribute in {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_dependency_declarations_for_member_crates() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);

        let member_manifest_paths = workspace_files
            .iter()
            .filter(|path| path.ends_with("Cargo.toml"))
            .filter(|path| {
                !path.ends_with("Cargo.toml") || *path != &workspace_root.join("Cargo.toml")
            })
            .filter(|path| {
                path.starts_with(workspace_root.join("server"))
                    || path.starts_with(workspace_root.join("shared_logic"))
                    || path.starts_with(workspace_root.join("test_helpers"))
            })
            .collect::<Vec<_>>();

        for manifest_path in member_manifest_paths {
            let manifest_content = read_file(manifest_path);
            let mut is_in_dependency_section = false;
            for line in manifest_content.lines() {
                let trimmed_line = line.trim();
                if trimmed_line.starts_with('[') {
                    is_in_dependency_section = trimmed_line == "[dependencies]"
                        || trimmed_line == "[dev-dependencies]"
                        || trimmed_line == "[build-dependencies]";
                    continue;
                }
                if !is_in_dependency_section
                    || trimmed_line.is_empty()
                    || trimmed_line.starts_with('#')
                {
                    continue;
                }
                assert!(
                    trimmed_line.contains(".workspace = true")
                        || trimmed_line.contains("workspace = true"),
                    "dependency entry is not workspace-based in {}: {}",
                    manifest_path.display(),
                    trimmed_line
                );
                assert!(
                    !trimmed_line.contains("version = "),
                    "member manifest must not pin crate versions directly in {}: {}",
                    manifest_path.display(),
                    trimmed_line
                );
            }
        }
    }

    #[test]
    fn requires_crates_io_dependencies_to_live_in_workspace_dependencies() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);
        let mut is_in_workspace_dependencies_section = false;

        for line in root_manifest_content.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with('[') {
                is_in_workspace_dependencies_section = trimmed_line == "[workspace.dependencies]";
                continue;
            }
            if !is_in_workspace_dependencies_section
                || trimmed_line.is_empty()
                || trimmed_line.starts_with('#')
            {
                continue;
            }
            if trimmed_line.contains("version = ") {
                assert!(
                    trimmed_line.contains("default-features = false"),
                    "workspace crates.io dependency must disable default features in {}: {}",
                    root_manifest_path.display(),
                    trimmed_line
                );
            }
        }
    }

    #[test]
    fn forbids_makefile_justfile_and_shell_scripts() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);

        assert!(
            !workspace_files
                .iter()
                .any(|path| path.file_name().is_some_and(|name| name == "Makefile")),
            "Makefile is forbidden by policy"
        );
        assert!(
            !workspace_files
                .iter()
                .any(|path| path.file_name().is_some_and(|name| name == "Justfile")),
            "Justfile is forbidden by policy"
        );
        assert!(
            !workspace_files
                .iter()
                .any(|path| path.extension().is_some_and(|extension| extension == "sh")),
            "shell script files (*.sh) are forbidden by policy"
        );
    }

    #[test]
    fn enforces_expect_message_identifier_format_for_single_and_multi_line_calls() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            let mut message_identifiers = Vec::new();
            let mut remaining_content = file_content.as_str();

            while let Some((_before_expect_call, after_expect_call)) =
                remaining_content.split_once("expect(")
            {
                let expect_argument = after_expect_call.trim_start();
                if let Some(expect_argument_without_open_quote) = expect_argument.strip_prefix('"')
                {
                    if let Some((message_identifier, _remaining_after_message)) =
                        expect_argument_without_open_quote.split_once('"')
                    {
                        message_identifiers.push(message_identifier);
                    }
                }
                remaining_content = after_expect_call;
            }

            for message_identifier in message_identifiers {
                assert!(
                    message_identifier.len() == 8,
                    "expect message id must be exactly 8 chars in {}: {}",
                    rust_file.display(),
                    message_identifier
                );
                assert!(
                    message_identifier
                        .chars()
                        .all(|character| character.is_ascii_hexdigit()
                            && !character.is_ascii_uppercase()),
                    "expect message id must be lowercase hex in {}: {}",
                    rust_file.display(),
                    message_identifier
                );
            }
        }
    }

    #[test]
    fn forbids_direct_index_zero_or_one_access_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            for line in file_content.lines() {
                let compact_line = line.replace(' ', "");
                assert!(
                    !compact_line.contains("[0]"),
                    "found forbidden [0] indexing pattern in {}: {}",
                    rust_file.display(),
                    line.trim()
                );
                assert!(
                    !compact_line.contains("[1]"),
                    "found forbidden [1] indexing pattern in {}: {}",
                    rust_file.display(),
                    line.trim()
                );
            }
        }
    }

    #[test]
    fn requires_shared_logic_workspace_crate_usage() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);
        assert!(
            root_manifest_content.contains("\"shared_logic\""),
            "workspace members must include shared_logic crate in {}",
            root_manifest_path.display()
        );

        let server_manifest_path = workspace_root.join("server").join("Cargo.toml");
        let server_manifest_content = read_file(&server_manifest_path);
        assert!(
            server_manifest_content.contains("shared_logic.workspace = true"),
            "server crate must use shared_logic via workspace dependency in {}",
            server_manifest_path.display()
        );
    }

    #[test]
    fn forbids_regular_loops_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);

            assert!(
                !source_segment.contains("\nfor "),
                "found regular for-loop in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("\nwhile "),
                "found regular while-loop in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_numeric_as_casts_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);
        let forbidden_as_patterns = [
            " as i8",
            " as i16",
            " as i32",
            " as i64",
            " as i128",
            " as isize",
            " as u8",
            " as u16",
            " as u32",
            " as u64",
            " as u128",
            " as usize",
            " as f32",
            " as f64",
        ];

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            for forbidden_as_pattern in forbidden_as_patterns {
                assert!(
                    !source_segment.contains(forbidden_as_pattern),
                    "found numeric `as` cast in non-test code: {} pattern: {}",
                    rust_file.display(),
                    forbidden_as_pattern
                );
            }
        }
    }

    #[test]
    fn forbids_axum_from_fn_layer_usage() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains(".layer(from_fn("),
                "found forbidden axum from_fn layer in {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_hardened_release_profile_configuration() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);

        assert!(
            root_manifest_content.contains("[profile.release]"),
            "missing [profile.release] section in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("lto = \"fat\""),
            "release profile must enable fat LTO in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("codegen-units = 1"),
            "release profile must use codegen-units = 1 in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("panic = \"abort\""),
            "release profile must use panic = \"abort\" in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("strip = \"symbols\""),
            "release profile must strip symbols in {}",
            root_manifest_path.display()
        );
    }

    #[test]
    fn enforces_workspace_verify_alias_order() {
        let workspace_root = workspace_root_path();
        let cargo_config_path = workspace_root.join(".cargo").join("config.toml");
        let cargo_config_content = read_file(&cargo_config_path);
        let expected_verify_alias = "workspace-verify = \"!cargo fmt && cargo clippy \
                                     --all-targets --all-features -- -D warnings && cargo test\"";

        assert!(
            cargo_config_content.contains(expected_verify_alias),
            "workspace-verify alias must execute fmt -> clippy -> test in {}",
            cargo_config_path.display()
        );
    }

    #[test]
    fn enforces_nightly_toolchain_channel() {
        let workspace_root = workspace_root_path();
        let rust_toolchain_path = workspace_root.join("rust-toolchain.toml");
        let rust_toolchain_content = read_file(&rust_toolchain_path);

        assert!(
            rust_toolchain_content.contains("channel = \"nightly\""),
            "workspace template requires nightly toolchain in {}",
            rust_toolchain_path.display()
        );
    }

    #[test]
    fn forbids_debug_print_macros_in_non_test_code_except_entrypoint_output() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);
        let server_main_path = workspace_root.join("server").join("src").join("main.rs");

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            let path_is_server_main = rust_file == &server_main_path;
            if !path_is_server_main {
                assert!(
                    !source_segment.contains("println!("),
                    "found println! in non-test non-entrypoint code: {}",
                    rust_file.display()
                );
                assert!(
                    !source_segment.contains("eprintln!("),
                    "found eprintln! in non-test non-entrypoint code: {}",
                    rust_file.display()
                );
            }
            assert!(
                !source_segment.contains("dbg!("),
                "found dbg! in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_alias_set_for_daily_workflows() {
        let workspace_root = workspace_root_path();
        let cargo_config_path = workspace_root.join(".cargo").join("config.toml");
        let cargo_config_content = read_file(&cargo_config_path);

        let required_aliases = [
            "workspace-format = ",
            "workspace-lint = ",
            "workspace-test = ",
            "workspace-check-no-default-features = ",
            "workspace-doc = ",
            "workspace-nextest = ",
            "workspace-hack = ",
            "workspace-deny = ",
            "workspace-udeps = ",
            "workspace-verify = ",
        ];

        for required_alias in required_aliases {
            assert!(
                cargo_config_content.contains(required_alias),
                "missing required cargo alias `{}` in {}",
                required_alias,
                cargo_config_path.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_resolver_and_member_list_contract() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);

        assert!(
            root_manifest_content.contains("resolver = \"2\""),
            "workspace root must define resolver = \"2\" in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("\"server\""),
            "workspace members must include server crate in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("\"shared_logic\""),
            "workspace members must include shared_logic crate in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("\"test_helpers\""),
            "workspace members must include test_helpers crate in {}",
            root_manifest_path.display()
        );
    }

    #[test]
    fn forbids_domain_shared_logic_from_direct_environment_or_filesystem_access() {
        let workspace_root = workspace_root_path();
        let shared_logic_source_path = workspace_root
            .join("shared_logic")
            .join("src")
            .join("lib.rs");
        let shared_logic_source_content = read_file(&shared_logic_source_path);
        let non_test_shared_logic_source_segment =
            non_test_source_segment(&shared_logic_source_content);

        assert!(
            !non_test_shared_logic_source_segment.contains("std::env::"),
            "domain shared_logic must not access std::env directly in {}",
            shared_logic_source_path.display()
        );
        assert!(
            !non_test_shared_logic_source_segment.contains("std::fs::"),
            "domain shared_logic must not access std::fs directly in {}",
            shared_logic_source_path.display()
        );
    }

    #[test]
    fn forbids_server_from_direct_thiserror_dependency() {
        let workspace_root = workspace_root_path();
        let server_manifest_path = workspace_root.join("server").join("Cargo.toml");
        let server_manifest_content = read_file(&server_manifest_path);

        assert!(
            !server_manifest_content.contains("thiserror"),
            "server crate must not depend on thiserror directly in {}",
            server_manifest_path.display()
        );
    }

    #[test]
    fn enforces_fast_ci_test_commands_presence() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content
                .contains("cargo nextest run --all-targets --all-features --profile ci"),
            "fast CI must run nextest in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content.contains("cargo test --doc --all-features"),
            "fast CI must run doc tests in {}",
            ci_workflow_path.display()
        );
    }

    #[test]
    fn enforces_full_ci_mode_to_keep_baseline_quality_gates() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content.contains(
                "(needs.changed-files.outputs.fast == 'true' || needs.changed-files.outputs.full \
                 == 'true')"
            ),
            "CI must keep baseline jobs enabled for both fast and full modes in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content.contains("run: cargo fmt --check"),
            "CI must keep formatting gate in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content
                .contains("run: cargo clippy --all-targets --all-features -- -D warnings"),
            "CI must keep clippy gate in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content
                .contains("run: cargo nextest run --all-targets --all-features --profile ci"),
            "CI must keep test gate in {}",
            ci_workflow_path.display()
        );
    }

    #[test]
    fn enforces_documented_local_verification_order() {
        let workspace_root = workspace_root_path();
        let readme_path = workspace_root.join("README.md");
        let readme_content = read_file(&readme_path);
        let contributing_path = workspace_root.join("CONTRIBUTING.md");
        let contributing_content = read_file(&contributing_path);
        let required_verification_sequence =
            "cargo fmt\ncargo clippy --all-targets --all-features -- -D warnings\ncargo test";

        assert!(
            readme_content.contains(required_verification_sequence),
            "README must document fmt -> clippy -> test order in {}",
            readme_path.display()
        );
        assert!(
            contributing_content.contains(required_verification_sequence),
            "CONTRIBUTING must document fmt -> clippy -> test order in {}",
            contributing_path.display()
        );
    }

    #[test]
    fn enforces_workflow_level_minimal_permissions() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            assert!(
                workflow_content.contains("permissions:\n  contents: read"),
                "workflow must define top-level minimal permissions in {}",
                workflow_file_path.display()
            );
        }
    }

    #[test]
    fn enforces_workflow_concurrency_cancellation() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            assert!(
                workflow_content.contains("cancel-in-progress: true"),
                "workflow must enable cancel-in-progress in {}",
                workflow_file_path.display()
            );
        }
    }

    #[test]
    fn enforces_timeout_minutes_for_each_workflow_job() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            let runs_on_count = workflow_content.matches("runs-on:").count();
            let timeout_minutes_count = workflow_content.matches("timeout-minutes:").count();
            assert!(
                runs_on_count == timeout_minutes_count,
                "every workflow job must define timeout-minutes in {}: runs-on={}, \
                 timeout-minutes={}",
                workflow_file_path.display(),
                runs_on_count,
                timeout_minutes_count
            );
        }
    }

    #[test]
    fn enforces_marketplace_actions_to_be_pinned_by_full_commit_sha() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            for workflow_line in workflow_content.lines() {
                let Some((_, action_reference_value)) = workflow_line.split_once("uses:") else {
                    continue;
                };
                let action_reference = action_reference_value.trim();
                if action_reference.starts_with("./") || action_reference.starts_with("docker://") {
                    continue;
                }
                let Some((action_name, action_version_reference)) =
                    action_reference.split_once('@')
                else {
                    continue;
                };
                if !action_name.contains('/') {
                    continue;
                }
                assert!(
                    action_version_reference.len() == 40
                        && action_version_reference
                            .chars()
                            .all(|character| character.is_ascii_hexdigit()),
                    "GitHub Action must be pinned by full 40-char SHA in {}: {}",
                    workflow_file_path.display(),
                    workflow_line.trim()
                );
            }
        }
    }

    #[test]
    fn enforces_must_use_annotations_for_shared_logic_public_result_apis() {
        let workspace_root = workspace_root_path();
        let shared_logic_source_path = workspace_root
            .join("shared_logic")
            .join("src")
            .join("lib.rs");
        let shared_logic_source_content = read_file(&shared_logic_source_path);
        let required_signatures = [
            "#[must_use = \"calculation evaluation result must be handled by the caller\"]\npub \
             fn evaluate_calculation_request(",
            "#[must_use = \"request parsing result must be handled by the caller\"]\npub fn \
             build_calculation_request_from_text_parts(",
            "#[must_use = \"wire-format deserialization result must be handled by the \
             caller\"]\npub fn deserialize_calculation_request_from_wire_format(",
            "#[must_use = \"report rendering result must be handled by the caller\"]\npub fn \
             render_calculation_report(",
            "#[must_use = \"report rendering result must be handled by the caller\"]\npub fn \
             render_calculation_report_with_format(",
        ];

        for required_signature in required_signatures {
            assert!(
                shared_logic_source_content.contains(required_signature),
                "missing #[must_use] annotation for shared_logic API `{}` in {}",
                required_signature,
                shared_logic_source_path.display()
            );
        }
    }

    #[test]
    fn forbids_public_struct_fields_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("policy_rules.rs") {
                continue;
            }

            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            let mut is_inside_public_struct = false;
            let mut public_struct_brace_depth = 0usize;

            for source_line in source_segment.lines() {
                let trimmed_line = source_line.trim_start();
                if !is_inside_public_struct
                    && trimmed_line.starts_with("pub struct ")
                    && trimmed_line.contains('{')
                {
                    is_inside_public_struct = true;
                    public_struct_brace_depth = source_line.matches('{').count();
                    public_struct_brace_depth =
                        public_struct_brace_depth.saturating_sub(source_line.matches('}').count());
                    continue;
                }

                if is_inside_public_struct {
                    assert!(
                        !trimmed_line.starts_with("pub "),
                        "found public struct field in non-test code: {}: {}",
                        rust_file.display(),
                        source_line.trim()
                    );

                    public_struct_brace_depth += source_line.matches('{').count();
                    public_struct_brace_depth =
                        public_struct_brace_depth.saturating_sub(source_line.matches('}').count());
                    if public_struct_brace_depth == 0 {
                        is_inside_public_struct = false;
                    }
                }
            }
        }
    }

    #[test]
    fn enforces_non_cli_startup_contract_tests_for_server() {
        let workspace_root = workspace_root_path();
        let cli_contract_test_path = workspace_root
            .join("server")
            .join("tests")
            .join("cli_contract.rs");
        let cli_contract_test_content = read_file(&cli_contract_test_path);

        assert!(
            cli_contract_test_content.contains("starts_without_arguments_in_default_text_mode"),
            "missing startup contract test for default launch in {}",
            cli_contract_test_path.display()
        );
        assert!(
            cli_contract_test_content
                .contains("ignores_command_line_arguments_and_keeps_startup_output_stable"),
            "missing non-cli argument-ignoring contract test in {}",
            cli_contract_test_path.display()
        );
        assert!(
            cli_contract_test_content
                .contains("returns_failure_for_non_unicode_report_format_environment_variable"),
            "missing non-unicode environment contract test in {}",
            cli_contract_test_path.display()
        );
    }

    #[test]
    fn forbids_placeholder_repository_metadata_in_workspace_crates() {
        let workspace_root = workspace_root_path();
        let manifest_paths = [
            workspace_root.join("server").join("Cargo.toml"),
            workspace_root.join("shared_logic").join("Cargo.toml"),
            workspace_root.join("test_helpers").join("Cargo.toml"),
        ];

        for manifest_path in manifest_paths {
            let manifest_content = read_file(&manifest_path);
            assert!(
                !manifest_content.contains("repository = \"https://github.com/user/repo\""),
                "placeholder repository metadata must be replaced in {}",
                manifest_path.display()
            );
        }
    }

    #[test]
    fn enforces_nightly_full_test_run_in_ci_for_harness_parity() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content
                .contains("cargo +nightly test --workspace --all-targets --all-features"),
            "CI must run nightly full cargo test for harness parity in {}",
            ci_workflow_path.display()
        );
    }

    #[test]
    fn enforces_dedicated_json_snapshot_contract_test_file_for_server_cli() {
        let workspace_root = workspace_root_path();
        let snapshot_test_path = workspace_root
            .join("server")
            .join("tests")
            .join("cli_json_snapshot.rs");
        let snapshot_test_content = read_file(&snapshot_test_path);

        assert!(
            snapshot_test_content.contains("json_startup_snapshot_contract_is_stable"),
            "missing dedicated JSON snapshot contract test in {}",
            snapshot_test_path.display()
        );
        assert!(
            snapshot_test_content.contains("assert_eq!("),
            "JSON snapshot contract test must assert exact output in {}",
            snapshot_test_path.display()
        );
    }
}
