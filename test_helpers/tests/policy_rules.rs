use std::{
    fs,
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

fn rust_source_files(workspace_files: &[PathBuf]) -> Vec<&PathBuf> {
    workspace_files
        .iter()
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("rs"))
        .collect()
}

#[cfg(test)]
mod policy_tests {
    use super::{
        collect_workspace_files, non_test_source_segment, read_file, rust_source_files,
        workspace_root_path,
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
    fn enforces_expect_message_identifier_length_when_literal_is_on_same_line() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            for line in file_content.lines() {
                if !line.contains("expect(\"") {
                    continue;
                }
                let Some((_, after_expect_open)) = line.split_once("expect(\"") else {
                    continue;
                };
                let Some((message_identifier, _)) = after_expect_open.split_once("\")") else {
                    continue;
                };
                assert!(
                    message_identifier.len() == 8,
                    "expect message id must be 8 chars in {}: {}",
                    rust_file.display(),
                    line.trim()
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
}
