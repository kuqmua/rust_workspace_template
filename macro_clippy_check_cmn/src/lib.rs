#[cfg(feature = "test-utils")]
const CARGO_FMT_ARGS: [&str; 1] = ["fmt"];
#[cfg(feature = "test-utils")]
const CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 6] = [
    "clippy",
    "--all-targets",
    "--all-features",
    "--",
    "-A",
    "warnings",
];
#[cfg(feature = "test-utils")]
const CARGO_CHECK_STEPS: [(&[&str], &str, &str); 2] = [
    (&CARGO_FMT_ARGS, "8dc4f045", "2a1deb01"),
    (
        &CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS,
        "cd48b869",
        "2c037283",
    ),
];
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
    let cargo_toml_extra = extra_cnt
        .lines()
        .map(|line| {
            if !line.contains("workspace = true") {
                return line.to_owned();
            }
            let Some((dep_name, _)) = line.split_once(" = ") else {
                return line.to_owned();
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
            dep_entry.replace("path = \"./", &format!("path = \"{root_path}/"))
        })
        .collect::<Vec<String>>()
        .join("\n");
    let cargo_toml_full = format!("{cargo_toml_cnt}\n{cargo_toml_extra}");
    std::fs::write(path_cargo_toml, cargo_toml_full).unwrap_or_else(|er| panic!("3757da9b: {er}"));
    std::fs::write(path_lib_rs, content_to_gen).unwrap_or_else(|er| panic!("55124f90: {er}"));
    CARGO_CHECK_STEPS
        .iter()
        .fold((), |(), (args, cmd_spawn_er_id, failed_id)| {
            let status = std::process::Command::new("cargo")
                .current_dir(&crate_path)
                .args(*args)
                .status()
                .unwrap_or_else(|er| panic!("{cmd_spawn_er_id}: {er}"));
            assert!(status.success(), "{failed_id}: {status}");
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
}
