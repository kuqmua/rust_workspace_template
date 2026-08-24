#[cfg(feature = "test-utils")]
const GENERATED_CRATE_STEPS: [GeneratedCrateStep; 4] = [
    GeneratedCrateStep {
        args: &str_constants::MACRO_CLIPPY_CARGO_FMT_ARGS,
        phase: GeneratedCratePhase::Formatting,
    },
    GeneratedCrateStep {
        args: &str_constants::MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS,
        phase: GeneratedCratePhase::Compilation,
    },
    GeneratedCrateStep {
        args: &str_constants::MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS,
        phase: GeneratedCratePhase::Clippy,
    },
    GeneratedCrateStep {
        args: &str_constants::MACRO_CLIPPY_CARGO_TEST_LIB_ARGS,
        phase: GeneratedCratePhase::Test,
    },
];
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
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
            Self::Clippy => f.write_str(str_constants::CLIPPY),
            Self::Compilation => f.write_str(str_constants::COMPILATION),
            Self::Formatting => f.write_str(str_constants::FORMATTING),
            Self::Test => f.write_str(str_constants::TEST_ALT_3),
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(feature = "test-utils")]
struct GeneratedCrateStep {
    args: &'static [&'static str],
    phase: GeneratedCratePhase,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(feature = "test-utils")]
struct RemoveDirOnDrop {
    path: std::path::PathBuf,
}
#[cfg(feature = "test-utils")]
impl Drop for RemoveDirOnDrop {
    fn drop(&mut self) {
        remove_dir_all_if_exists(&self.path, str_constants::E28698F2);
        if let Some(parent) = self.path.parent()
            && let Err(error) = std::fs::remove_dir(parent)
            && error.kind() != std::io::ErrorKind::NotFound
            && error.kind() != std::io::ErrorKind::DirectoryNotEmpty
        {
            panic!("a83f7c18: {error}");
        }
    }
}
#[cfg(feature = "test-utils")]
fn remove_dir_all_if_exists(path: &std::path::Path, error_id: &str) {
    if let Err(error) = std::fs::remove_dir_all(path)
        && error.kind() != std::io::ErrorKind::NotFound
    {
        panic!("{error_id}: {error}");
    }
}
#[cfg(feature = "test-utils")]
pub fn clippy_check(crate_name: &str, _cmd_path: &str, extra_cnt: &str, content_to_generate: &str) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| panic!("2d592b13"), std::path::Path::to_path_buf);
    let crate_path = root
        .join(str_constants::TARGET_MACRO_CHECK)
        .join(crate_name);
    remove_dir_all_if_exists(&crate_path, str_constants::E28698F2);
    std::fs::create_dir_all(crate_path.join(str_constants::SRC_ALT))
        .unwrap_or_else(|error| panic!("2b24ef1a: {error}"));
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
    let path_lib_rs = crate_path.join(str_constants::SRC_LIB_RS);
    let path_cargo_toml = crate_path.join(str_constants::CARGO_TOML);
    let workspace_manifest_path = root.join(str_constants::CARGO_TOML);
    let workspace_cargo_toml = server_runtime_http::read_bounded_file(
        server_runtime_http::StdPathRef::from(workspace_manifest_path.as_path()),
        server_runtime_http::BoundedReadMaximumBytes::from(1_048_576usize),
    )
    .and_then(server_runtime_http::BoundedText::try_from)
    .unwrap_or_else(|error| panic!("bf40d675: {error}"));
    let root_path = root.display().to_string();
    let cargo_toml_extra = extra_cnt.lines().fold(
        String::with_capacity(extra_cnt.len()),
        |mut output, line| {
            let transform_line = || -> std::borrow::Cow<'_, str> {
                if !line.contains(str_constants::WORKSPACE_TRUE) {
                    return std::borrow::Cow::Borrowed(line);
                }
                let Some((dep_name, _)) = line.split_once(str_constants::TEXT_ALT) else {
                    return std::borrow::Cow::Borrowed(line);
                };
                let prefix = format!("{dep_name} = ");
                let braces_balance = |value: &str| -> i32 {
                    value.chars().fold(0i32, |accumulator, ch| match ch {
                        '{' | '[' => accumulator
                            .checked_add(1i32)
                            .unwrap_or_else(|| panic!("0a8df093")),
                        '}' | ']' => accumulator
                            .checked_sub(1i32)
                            .unwrap_or_else(|| panic!("4e404fc9")),
                        _ => accumulator,
                    })
                };
                let mut in_workspace_deps = false;
                let mut workspace_lines = workspace_cargo_toml.as_ref().lines();
                let mut dep_entry = loop {
                    let Some(workspace_line) = workspace_lines.next() else {
                        panic!("1bb3996c");
                    };
                    if workspace_line == str_constants::WORKSPACE_DEPENDENCIES {
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
                let feature_list = line.split_once(str_constants::FEATURES).map(|(_, tail)| {
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
                if !dep_entry.contains(str_constants::FEATURES)
                    && let Some(features) = feature_list
                    && let Some(idx) = dep_entry.rfind('}')
                {
                    dep_entry.insert_str(idx, &format!(", features = {features}"));
                }
                if let Some(path_prefix_idx) = dep_entry.find(str_constants::PATH_ALT_4) {
                    let dot_idx = path_prefix_idx.saturating_add(str_constants::PATH_ALT_3.len());
                    if dep_entry.get(dot_idx..dot_idx.saturating_add(1usize))
                        == Some(str_constants::DOT)
                    {
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
    std::fs::write(path_cargo_toml, cargo_toml_full)
        .unwrap_or_else(|error| panic!("3757da9b: {error}"));
    std::fs::write(path_lib_rs, content_to_generate)
        .unwrap_or_else(|error| panic!("55124f90: {error}"));
    let _copied_lock_bytes = std::fs::copy(
        root.join(str_constants::CARGO_LOCK),
        crate_path.join(str_constants::CARGO_LOCK),
    )
    .unwrap_or_else(|error| panic!("1dda80f9: {error}"));
    GENERATED_CRATE_STEPS.iter().fold((), |(), step| {
        let status = macros_helpers::tool_command::ToolCommand::new(
            macros_helpers::tool_command::ToolProgramRef::from(
                str_constants::WORKSPACE_TEST_RUNNER_CARGO,
            ),
        )
        .current_dir(macros_helpers::tool_command::StdPathRef::from(
            crate_path.as_path(),
        ))
        .args(macros_helpers::tool_command::ToolArgsRef::from(step.args))
        .status()
        .unwrap_or_else(|error| {
            panic!(
                "cd48b869: generated crate {} phase failed to start at {}: {error}",
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
    #[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
    struct StdTmpDir(std::path::PathBuf);

    impl StdTmpDir {
        fn new() -> Self {
            let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "macro_clippy_check_common_{}_{}",
                std::process::id(),
                seq
            ));
            std::fs::create_dir_all(&path).expect("0c77b4c7 new invariant must hold");
            Self::from(path)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }
    impl Drop for StdTmpDir {
        fn drop(&mut self) {
            if let Err(error) = std::fs::remove_dir_all(&self.0)
                && error.kind() != std::io::ErrorKind::NotFound
            {
                panic!("15ab6a8d: {error}");
            }
        }
    }
    #[test]
    fn remove_dir_on_drop_removes_temp_crate_dir() {
        let dir = StdTmpDir::new();
        let path = dir.path().join(str_constants::CRATE_DIR);
        std::fs::create_dir_all(&path)
            .expect("9b0e24f1 remove_dir_on_drop_removes_temp_crate_dir invariant must hold");
        let guard = super::RemoveDirOnDrop { path: path.clone() };
        drop(guard);
        assert!(!path.exists());
    }
    #[test]
    fn remove_dir_all_if_exists_accepts_missing_dir() {
        let dir = StdTmpDir::new();
        let path = dir.path().join(str_constants::MISSING_DIR);
        super::remove_dir_all_if_exists(&path, str_constants::F39C05AA);
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
    #[test]
    fn generated_crate_compilation_is_offline_and_follow_up_steps_are_locked() {
        assert!(
            str_constants::MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS
                .contains(&"--offline")
        );
        [
            str_constants::MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS.as_slice(),
            str_constants::MACRO_CLIPPY_CARGO_TEST_LIB_ARGS.as_slice(),
        ]
        .into_iter()
        .all(|args| args.contains(&"--locked") && args.contains(&"--offline"))
        .then_some(())
        .expect("3f63f262 generated_crate_compilation_is_offline_and_follow_up_steps_are_locked invariant must hold");
    }
}
