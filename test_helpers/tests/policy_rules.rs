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
}
