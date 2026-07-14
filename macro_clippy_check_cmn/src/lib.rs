#[cfg(feature = "test-utils")]
const CARGO_FMT_ARGS: [&str; 1] = ["fmt"];
#[cfg(feature = "test-utils")]
const CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 3] =
    ["check", "--all-targets", "--all-features"];
#[cfg(feature = "test-utils")]
// Generated fixtures intentionally preserve public wire/error shapes and mechanically emitted assertions; the narrow legacy allowlist keeps those contracts stable while every other warning remains denied.
const CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 22] = [
    "clippy",
    "--all-targets",
    "--all-features",
    "--",
    "-D",
    "warnings",
    "-A",
    "clippy::bool_assert_comparison",
    "-A",
    "clippy::clone_on_copy",
    "-A",
    "clippy::collapsible_if",
    "-A",
    "clippy::let_and_return",
    "-A",
    "clippy::result_large_err",
    "-A",
    "clippy::single_call_fn",
    "-A",
    "clippy::useless_borrows_in_formatting",
    "-A",
    "clippy::write_literal",
];
#[cfg(feature = "test-utils")]
const CARGO_TEST_LIB_ARGS: [&str; 2] = ["test", "--lib"];
#[cfg(feature = "test-utils")]
const GENERATED_CRATE_STEPS: [GeneratedCrateStep; 4] = [
    GeneratedCrateStep {
        args: &CARGO_FMT_ARGS,
        phase: GeneratedCratePhase::Formatting,
    },
    GeneratedCrateStep {
        args: &CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS,
        phase: GeneratedCratePhase::Compilation,
    },
    GeneratedCrateStep {
        args: &CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS,
        phase: GeneratedCratePhase::Clippy,
    },
    GeneratedCrateStep {
        args: &CARGO_TEST_LIB_ARGS,
        phase: GeneratedCratePhase::Test,
    },
];
#[cfg(feature = "test-utils")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneratedCratePhase {
    Clippy,
    Compilation,
    Formatting,
    Test,
}
#[cfg(feature = "test-utils")]
impl std::fmt::Display for GeneratedCratePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clippy => f.write_str("clippy"),
            Self::Compilation => f.write_str("compilation"),
            Self::Formatting => f.write_str("formatting"),
            Self::Test => f.write_str("test"),
        }
    }
}
#[cfg(feature = "test-utils")]
struct GeneratedCrateStep {
    args: &'static [&'static str],
    phase: GeneratedCratePhase,
}
#[cfg(feature = "test-utils")]
struct RemoveDirOnDrop {
    path: std::path::PathBuf,
}
#[cfg(feature = "test-utils")]
impl Drop for RemoveDirOnDrop {
    fn drop(&mut self) {
        remove_dir_all_if_exists(&self.path, "e28698f2");
        if let Some(parent) = self.path.parent()
            && let Err(er) = std::fs::remove_dir(parent)
            && er.kind() != std::io::ErrorKind::NotFound
            && er.kind() != std::io::ErrorKind::DirectoryNotEmpty
        {
            panic!("a83f7c18: {er}");
        }
    }
}
#[cfg(feature = "test-utils")]
fn remove_dir_all_if_exists(path: &std::path::Path, er_id: &str) {
    if let Err(er) = std::fs::remove_dir_all(path)
        && er.kind() != std::io::ErrorKind::NotFound
    {
        panic!("{er_id}: {er}");
    }
}
#[cfg(feature = "test-utils")]
pub fn clippy_check(crate_name: &str, _cmd_path: &str, extra_cnt: &str, content_to_gen: &str) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| panic!("2d592b13"), std::path::Path::to_path_buf);
    let crate_path = root.join("target/macro-check").join(crate_name);
    remove_dir_all_if_exists(&crate_path, "e28698f2");
    std::fs::create_dir_all(crate_path.join("src")).unwrap_or_else(|er| panic!("2b24ef1a: {er}"));
    let _remove_dir_on_drop = RemoveDirOnDrop {
        path: crate_path.clone(),
    };
    let cargo_toml_cnt = format!(
        r#"[package]
name = "{crate_name}"
publish = false
version = "0.1.0"
edition = "2024"
description = "description"
repository = "repository"
readme = "readme"
license = "license"
keywords = ["keyword"]
categories = ["category"]
[workspace]"#
    );
    let path_lib_rs = crate_path.join("src/lib.rs");
    let path_cargo_toml = crate_path.join("Cargo.toml");
    let workspace_cargo_toml = std::fs::read_to_string(root.join("Cargo.toml"))
        .unwrap_or_else(|er| panic!("bf40d675: {er}"));
    let root_path = root.display().to_string();
    let cargo_toml_extra = extra_cnt.lines().fold(
        String::with_capacity(extra_cnt.len()),
        |mut output, line| {
            let transform_line = || -> std::borrow::Cow<'_, str> {
                if !line.contains("workspace = true") {
                    return std::borrow::Cow::Borrowed(line);
                }
                let Some((dep_name, _)) = line.split_once(" = ") else {
                    return std::borrow::Cow::Borrowed(line);
                };
                let prefix = format!("{dep_name} = ");
                let braces_balance = |value: &str| -> i32 {
                    value.chars().fold(0i32, |acc, ch| match ch {
                        '{' | '[' => acc.checked_add(1i32).unwrap_or_else(|| panic!("0a8df093")),
                        '}' | ']' => acc.checked_sub(1i32).unwrap_or_else(|| panic!("4e404fc9")),
                        _ => acc,
                    })
                };
                let mut in_workspace_deps = false;
                let mut workspace_lines = workspace_cargo_toml.lines();
                let mut dep_entry = loop {
                    let Some(workspace_line) = workspace_lines.next() else {
                        panic!("1bb3996c");
                    };
                    if workspace_line == "[workspace.dependencies]" {
                        in_workspace_deps = true;
                        continue;
                    }
                    assert!(
                        !(in_workspace_deps && workspace_line.starts_with('[')),
                        "1bb3996c"
                    );
                    if in_workspace_deps && workspace_line.starts_with(&prefix) {
                        let mut out = String::from(workspace_line);
                        let mut balance = braces_balance(workspace_line);
                        while balance > 0i32 {
                            let next_line =
                                workspace_lines.next().unwrap_or_else(|| panic!("7bb3cd14"));
                            out.push('\n');
                            out.push_str(next_line);
                            balance = balance
                                .checked_add(braces_balance(next_line))
                                .unwrap_or_else(|| panic!("f1e71cd6"));
                        }
                        break out;
                    }
                };
                let feature_list = line.split_once("features = ").map(|(_, tail)| {
                    tail.chars()
                        .scan(false, |done, ch| {
                            if *done {
                                None
                            } else {
                                if ch == ']' {
                                    *done = true;
                                }
                                Some(ch)
                            }
                        })
                        .collect::<String>()
                });
                if !dep_entry.contains("features = ")
                    && let Some(features) = feature_list
                    && let Some(idx) = dep_entry.rfind('}')
                {
                    dep_entry.insert_str(idx, &format!(", features = {features}"));
                }
                if let Some(path_prefix_idx) = dep_entry.find("path = \"./") {
                    let dot_idx = path_prefix_idx.saturating_add("path = \"".len());
                    if dep_entry.get(dot_idx..dot_idx.saturating_add(1usize)) == Some(".") {
                        dep_entry
                            .replace_range(dot_idx..dot_idx.saturating_add(1usize), &root_path);
                    }
                }
                std::borrow::Cow::Owned(dep_entry)
            };
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(&transform_line());
            output
        },
    );
    let mut cargo_toml_full = cargo_toml_cnt;
    cargo_toml_full.reserve(1usize.saturating_add(cargo_toml_extra.len()));
    cargo_toml_full.push('\n');
    cargo_toml_full.push_str(&cargo_toml_extra);
    drop(cargo_toml_extra);
    std::fs::write(path_cargo_toml, cargo_toml_full).unwrap_or_else(|er| panic!("3757da9b: {er}"));
    std::fs::write(path_lib_rs, content_to_gen).unwrap_or_else(|er| panic!("55124f90: {er}"));
    GENERATED_CRATE_STEPS.iter().fold((), |(), step| {
        let status = macros_helpers::tool_command::ToolCommand::new(
            macros_helpers::tool_command::ToolProgramRef::from("cargo"),
        )
        .current_dir(macros_helpers::tool_command::StdPathRef::from(
            crate_path.as_path(),
        ))
        .args(macros_helpers::tool_command::ToolArgsRef::from(step.args))
        .status()
        .unwrap_or_else(|er| {
            panic!(
                "cd48b869: generated crate {} phase failed to start at {}: {er}",
                step.phase,
                crate_path.display()
            )
        });
        assert!(
            status.success(),
            "2c037283: generated crate {} phase failed at {}: {status}",
            step.phase,
            crate_path.display()
        );
    });
}
#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    struct StdTmpDir(std::path::PathBuf);
    impl StdTmpDir {
        fn new() -> Self {
            let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "macro_clippy_check_cmn_{}_{}",
                std::process::id(),
                seq
            ));
            std::fs::create_dir_all(&path).expect("2b24ef1a");
            Self(path)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }
    impl Drop for StdTmpDir {
        fn drop(&mut self) {
            if let Err(er) = std::fs::remove_dir_all(&self.0)
                && er.kind() != std::io::ErrorKind::NotFound
            {
                panic!("15ab6a8d: {er}");
            }
        }
    }
    #[test]
    fn remove_dir_on_drop_removes_temp_crate_dir() {
        let dir = StdTmpDir::new();
        let path = dir.path().join("crate_dir");
        std::fs::create_dir_all(&path).expect("9b0e24f1");
        let guard = super::RemoveDirOnDrop { path: path.clone() };
        drop(guard);
        assert!(!path.exists());
    }
    #[test]
    fn remove_dir_all_if_exists_accepts_missing_dir() {
        let dir = StdTmpDir::new();
        let path = dir.path().join("missing_dir");
        super::remove_dir_all_if_exists(&path, "f39c05aa");
        assert!(!path.exists());
    }
    #[test]
    fn generated_crate_phases_have_stable_diagnostics() {
        let phases = [
            super::GeneratedCratePhase::Compilation,
            super::GeneratedCratePhase::Clippy,
            super::GeneratedCratePhase::Formatting,
            super::GeneratedCratePhase::Test,
        ];
        assert_eq!(
            phases.map(|phase| phase.to_string()),
            ["compilation", "clippy", "formatting", "test"]
        );
    }
}
