#[cfg(feature = "test-utils")]
pub mod generated_crate_phase;
#[cfg(feature = "test-utils")]
pub mod generated_crate_step;
#[cfg(any(test, feature = "test-utils"))]
pub mod generated_crate_steps_tests;
#[cfg(feature = "test-utils")]
pub mod remove_dir_on_drop;

#[cfg(feature = "test-utils")]
impl Drop for remove_dir_on_drop::RemoveDirOnDrop {
    fn drop(&mut self) {
        remove_dir_all_if_exists(self.get_path(), constants_str::E28698F2);
        if let Some(parent) = self.get_path().parent()
            && let Err(error) = std::fs::remove_dir(parent)
            && error.kind() != std::io::ErrorKind::NotFound
            && error.kind() != std::io::ErrorKind::DirectoryNotEmpty
        {
            std::panic::panic_any(constants_str::PANIC_A83F7C18.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            ));
        }
    }
}
#[cfg(feature = "test-utils")]
fn remove_dir_all_if_exists(path: &std::path::Path, error_id: &str) {
    if let Err(error) = std::fs::remove_dir_all(path)
        && error.kind() != std::io::ErrorKind::NotFound
    {
        std::panic::panic_any(
            constants_str::PANIC_AF2FFBC7
                .replacen(constants_str::PANIC_PLACEHOLDER_81766C62, error_id, 1usize)
                .replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                ),
        );
    }
}
#[cfg(feature = "test-utils")]
pub fn clippy_check(crate_name: &str, _cmd_path: &str, extra_cnt: &str, content_to_generate: &str) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(
            || std::panic::panic_any(constants_str::PANIC_2D592B13),
            std::path::Path::to_path_buf,
        );
    let crate_path = root
        .join(constants_str::TARGET_MACRO_CHECK)
        .join(crate_name);
    remove_dir_all_if_exists(&crate_path, constants_str::E28698F2);
    std::fs::create_dir_all(crate_path.join(constants_str::SRC_ALT)).unwrap_or_else(|error| {
        std::panic::panic_any(constants_str::PANIC_2B24EF1A.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        ))
    });
    let _remove_dir_on_drop = remove_dir_on_drop::RemoveDirOnDrop::new(crate_path.clone());
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
    let path_lib_rs = crate_path.join(constants_str::SRC_LIB_RS);
    let path_cargo_toml = crate_path.join(constants_str::CARGO_TOML);
    let workspace_manifest_path = root.join(constants_str::CARGO_TOML);
    let workspace_cargo_toml = server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(
            workspace_manifest_path.as_path(),
        ),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            constants_usize::VALUE_1_048_576,
        ),
    )
    .and_then(server_runtime_http::bounded_text::BoundedText::try_from)
    .unwrap_or_else(|error| {
        std::panic::panic_any(constants_str::PANIC_BF40D675.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        ))
    });
    let root_path = root.display().to_string();
    let cargo_toml_extra = extra_cnt.lines().fold(
        String::with_capacity(extra_cnt.len()),
        |mut output, line| {
            let transform_line = || -> std::borrow::Cow<'_, str> {
                if !line.contains(constants_str::WORKSPACE_TRUE) {
                    return std::borrow::Cow::Borrowed(line);
                }
                let Some((dep_name, _)) = line.split_once(constants_str::TEXT_ALT) else {
                    return std::borrow::Cow::Borrowed(line);
                };
                let prefix = format!("{dep_name} = ");
                let braces_balance = |value: &str| -> i32 {
                    value
                        .chars()
                        .fold(constants_i32::ZERO, |accumulator, ch| match ch {
                            '{' | '[' => accumulator.checked_add(1i32).unwrap_or_else(|| {
                                std::panic::panic_any(constants_str::PANIC_0A8DF093)
                            }),
                            '}' | ']' => accumulator.checked_sub(1i32).unwrap_or_else(|| {
                                std::panic::panic_any(constants_str::PANIC_4E404FC9)
                            }),
                            _ => accumulator,
                        })
                };
                let mut in_workspace_deps = false;
                let mut workspace_lines = workspace_cargo_toml.as_ref().lines();
                let mut dep_entry = loop {
                    let Some(workspace_line) = workspace_lines.next() else {
                        std::panic::panic_any(constants_str::PANIC_1BB3996C);
                    };
                    if workspace_line == constants_str::WORKSPACE_DEPENDENCIES {
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
                        while balance > constants_i32::ZERO {
                            let next_line = workspace_lines.next().unwrap_or_else(|| {
                                std::panic::panic_any(constants_str::PANIC_7BB3CD14)
                            });
                            out.push('\n');
                            out.push_str(next_line);
                            balance = balance
                                .checked_add(braces_balance(next_line))
                                .unwrap_or_else(|| {
                                    std::panic::panic_any(constants_str::PANIC_F1E71CD6)
                                });
                        }
                        break out;
                    }
                };
                let feature_list = line.split_once(constants_str::FEATURES).map(|(_, tail)| {
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
                if !dep_entry.contains(constants_str::FEATURES)
                    && let Some(features) = feature_list
                    && let Some(idx) = dep_entry.rfind('}')
                {
                    dep_entry.insert_str(idx, &format!(", features = {features}"));
                }
                if let Some(path_prefix_idx) = dep_entry.find(constants_str::PATH_ALT_4) {
                    let dot_idx = path_prefix_idx.saturating_add(constants_str::PATH_ALT_3.len());
                    if dep_entry.get(dot_idx..dot_idx.saturating_add(constants_usize::ONE))
                        == Some(constants_str::DOT)
                    {
                        dep_entry.replace_range(
                            dot_idx..dot_idx.saturating_add(constants_usize::ONE),
                            &root_path,
                        );
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
    cargo_toml_full.reserve(constants_usize::ONE.saturating_add(cargo_toml_extra.len()));
    cargo_toml_full.push('\n');
    cargo_toml_full.push_str(&cargo_toml_extra);
    drop(cargo_toml_extra);
    std::fs::write(path_cargo_toml, cargo_toml_full).unwrap_or_else(|error| {
        std::panic::panic_any(constants_str::PANIC_3757DA9B.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        ))
    });
    std::fs::write(path_lib_rs, content_to_generate).unwrap_or_else(|error| {
        std::panic::panic_any(constants_str::PANIC_55124F90.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        ))
    });
    let _copied_lock_bytes = std::fs::copy(
        root.join(constants_str::CARGO_LOCK),
        crate_path.join(constants_str::CARGO_LOCK),
    )
    .unwrap_or_else(|error| {
        std::panic::panic_any(constants_str::PANIC_1DDA80F9.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        ))
    });
    generated_crate_steps_tests::GENERATED_CRATE_STEPS
        .iter()
        .fold((), |(), step| {
            let status = macro_helpers::tool_command::ToolCommand::new(
                macro_helpers::tool_program_ref::ToolProgramRef::from(
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                ),
            )
            .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
                crate_path.as_path(),
            ))
            .args(macro_helpers::tool_args_ref::ToolArgsRef::from(step.args()))
            .status()
            .unwrap_or_else(|error| {
                std::panic::panic_any(
                    constants_str::PANIC_CD48B869
                        .replacen(
                            constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                            step.phase().to_string().as_str(),
                            1usize,
                        )
                        .replacen(
                            constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                            crate_path.display().to_string().as_str(),
                            1usize,
                        )
                        .replacen(
                            constants_str::PANIC_PLACEHOLDER_81240055,
                            error.to_string().as_str(),
                            1usize,
                        ),
                )
            });
            assert!(
                status.success(),
                "2c037283: generated crate {} phase failed at {}: {status}",
                step.phase().to_string().as_str(),
                crate_path.display()
            );
        });
}
#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    #[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
    struct TmpDirPathBuf(std::path::PathBuf);

    impl TmpDirPathBuf {
        fn new() -> Self {
            let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "macro_clippy_check_test_common_{}_{}",
                std::process::id(),
                seq
            ));
            std::fs::create_dir_all(&path).expect(constants_str::DIAGNOSTIC_0C77B4C7);
            Self::from(path)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }
    impl Drop for TmpDirPathBuf {
        fn drop(&mut self) {
            if let Err(error) = std::fs::remove_dir_all(&self.0)
                && error.kind() != std::io::ErrorKind::NotFound
            {
                std::panic::panic_any(constants_str::PANIC_15AB6A8D.replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                ));
            }
        }
    }
    #[test]
    fn test_remove_dir_on_drop_removes_temp_crate_dir() {
        let dir = TmpDirPathBuf::new();
        let path = dir.path().join(constants_str::CRATE_DIR);
        std::fs::create_dir_all(&path).expect(constants_str::DIAGNOSTIC_9B0E24F1);
        let guard = super::remove_dir_on_drop::RemoveDirOnDrop::new(path.clone());
        drop(guard);
        assert!(!path.exists());
    }
    #[test]
    fn test_remove_dir_all_if_exists_accepts_missing_dir() {
        let dir = TmpDirPathBuf::new();
        let path = dir.path().join(constants_str::MISSING_DIR);
        crate::remove_dir_all_if_exists(&path, constants_str::F39C05AA);
        assert!(!path.exists());
    }
    #[test]
    fn test_generated_crate_phases_have_stable_diagnostics() {
        let phases = [
            super::generated_crate_phase::GeneratedCratePhase::Compilation,
            super::generated_crate_phase::GeneratedCratePhase::Clippy,
            super::generated_crate_phase::GeneratedCratePhase::Formatting,
            super::generated_crate_phase::GeneratedCratePhase::Test,
        ];
        assert_eq!(
            phases.map(|phase| phase.to_string()),
            ["compilation", "clippy", "formatting", "test"]
        );
    }
    #[test]
    fn test_generated_crate_compilation_is_offline_and_follow_up_steps_are_locked() {
        assert!(
            constants_str::MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS
                .contains(&"--offline")
        );
        [
            constants_str::MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS.as_slice(),
            constants_str::MACRO_CLIPPY_CARGO_TEST_LIB_ARGS.as_slice(),
        ]
        .into_iter()
        .all(|args| {
            args.contains(&constants_str::SHARED_VALUES_LOCKED)
                && args.contains(&constants_str::SHARED_VALUES_OFFLINE)
        })
        .then_some(())
        .expect(constants_str::DIAGNOSTIC_3F63F262);
    }
}
