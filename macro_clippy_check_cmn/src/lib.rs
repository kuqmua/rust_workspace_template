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
        remove_empty_parent_dir_if_exists(&self.path, "a83f7c18");
    }
}
#[cfg(feature = "test-utils")]
fn write_or_panic(path: &std::path::Path, cnt: &str, write_er_id: &str) {
    std::fs::write(path, cnt).unwrap_or_else(|er| panic!("{write_er_id}: {er}"));
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // small filesystem wrapper keeps panic IDs consistent for generated temp crate creation
fn create_dir_all_or_panic(path: &std::path::Path, er_id: &str) {
    std::fs::create_dir_all(path).unwrap_or_else(|er| panic!("{er_id}: {er}"));
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
#[allow(clippy::single_call_fn)] // cleanup guard removes the generated crate and then prunes the empty macro-check parent directory
fn remove_empty_parent_dir_if_exists(path: &std::path::Path, er_id: &str) {
    if let Some(parent) = path.parent()
        && let Err(er) = std::fs::remove_dir(parent)
        && er.kind() != std::io::ErrorKind::NotFound
        && er.kind() != std::io::ErrorKind::DirectoryNotEmpty
    {
        panic!("{er_id}: {er}");
    }
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // manifest reads need stable panic IDs because failures are environment/setup errors
fn read_to_string_or_panic(path: &std::path::Path, er_id: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|er| panic!("{er_id}: {er}"))
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // root workspace manifest path is derived from the package that owns this helper crate
fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| panic!("2d592b13"), std::path::Path::to_path_buf)
}
#[cfg(feature = "test-utils")]
fn braces_balance(v: &str) -> i32 {
    v.chars().fold(0i32, |acc, ch| match ch {
        '{' | '[' => acc.checked_add(1i32).unwrap_or_else(|| panic!("0a8df093")),
        '}' | ']' => acc.checked_sub(1i32).unwrap_or_else(|| panic!("4e404fc9")),
        _ => acc,
    })
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // dependency lookup is isolated from line rewriting so missing workspace deps report one stable panic ID
fn workspace_dep_entry(workspace_cargo_toml: &str, dep_name: &str) -> String {
    let prefix = format!("{dep_name} = ");
    let mut in_workspace_deps = false;
    let mut lines = workspace_cargo_toml.lines();
    while let Some(line) = lines.next() {
        if line == "[workspace.dependencies]" {
            in_workspace_deps = true;
            continue;
        }
        if in_workspace_deps && line.starts_with('[') {
            break;
        }
        if in_workspace_deps && line.starts_with(&prefix) {
            let mut out = String::from(line);
            let mut balance = braces_balance(line);
            while balance > 0i32 {
                let next_line = lines.next().unwrap_or_else(|| panic!("7bb3cd14"));
                out.push('\n');
                out.push_str(next_line);
                balance = balance
                    .checked_add(braces_balance(next_line))
                    .unwrap_or_else(|| panic!("f1e71cd6"));
            }
            return out;
        }
    }
    panic!("1bb3996c");
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // split out intentionally to keep low-level cargo spawn/status check reusable from orchestration helper
fn run_cargo_checked(
    target_crate_dir: &std::path::Path,
    args: &[&str],
    cmd_spawn_er_id: &str,
    failed_id: &str,
) {
    let status = std::process::Command::new("cargo")
        .current_dir(target_crate_dir)
        .args(args)
        .status()
        .unwrap_or_else(|er| panic!("{cmd_spawn_er_id}: {er}"));
    assert!(status.success(), "{failed_id}: {status}");
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // centralizes ordered cargo check execution to keep command flow reusable and consistent
fn run_cargo_check_steps(target_crate_dir: &std::path::Path, steps: &[(&[&str], &str, &str)]) {
    steps
        .iter()
        .fold((), |(), (args, cmd_spawn_er_id, failed_id)| {
            run_cargo_checked(target_crate_dir, args, cmd_spawn_er_id, failed_id);
        });
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // extracts feature overrides from dependency lines before they are merged into workspace entries
fn dep_features_from_workspace_line(line: &str) -> Option<String> {
    let (_, tail) = line.split_once("features = ")?;
    let features = tail
        .chars()
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
        .collect::<String>();
    Some(features)
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // generated temp manifests need caller feature overrides while preserving workspace dependency versions
fn merge_dep_features(mut dep_entry: String, features: Option<String>) -> String {
    if dep_entry.contains("features = ") {
        return dep_entry;
    }
    if let Some(feature_list) = features
        && let Some(idx) = dep_entry.rfind('}')
    {
        dep_entry.insert_str(idx, &format!(", features = {feature_list}"));
    }
    dep_entry
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // path dependencies copied from workspace.dependencies must become absolute for target/macro-check crates
fn rewrite_dep_entry_paths(dep_entry: &str, root: &std::path::Path) -> String {
    let root_path = root.display().to_string();
    dep_entry.replace("path = \"./", &format!("path = \"{root_path}/"))
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // resolves one workspace dependency line while preserving non-dependency manifest lines untouched
fn resolve_workspace_dep_line(
    line: &str,
    root: &std::path::Path,
    workspace_cargo_toml: &str,
) -> String {
    let Some((dep_name, _)) = line.split_once(" = ") else {
        return line.to_owned();
    };
    let workspace_entry = workspace_dep_entry(workspace_cargo_toml, dep_name);
    let merged_entry = merge_dep_features(workspace_entry, dep_features_from_workspace_line(line));
    rewrite_dep_entry_paths(&merged_entry, root)
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // converts workspace-inherited dependency snippets into standalone temp crate manifest snippets
fn resolve_extra_cnt(root: &std::path::Path, extra_cnt: &str) -> String {
    let workspace_cargo_toml = read_to_string_or_panic(&root.join("Cargo.toml"), "bf40d675");
    extra_cnt
        .lines()
        .map(|line| {
            if line.contains("workspace = true") {
                resolve_workspace_dep_line(line, root, &workspace_cargo_toml)
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // shared helper keeps generated-file write flow consistent for clippy-check setup
fn write_generated_files(
    path_cargo_toml: &std::path::Path,
    cargo_toml_full: &str,
    path_lib_rs: &std::path::Path,
    content_to_gen: &str,
) {
    write_or_panic(path_cargo_toml, cargo_toml_full, "3757da9b");
    write_or_panic(path_lib_rs, content_to_gen, "55124f90");
}
#[cfg(feature = "test-utils")]
pub fn clippy_check(crate_name: &str, _cmd_path: &str, extra_cnt: &str, content_to_gen: &str) {
    let root = workspace_root();
    let crate_path = root.join("target/macro-check").join(crate_name);
    remove_dir_all_if_exists(&crate_path, "e28698f2");
    create_dir_all_or_panic(&crate_path.join("src"), "2b24ef1a");
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
    let cargo_toml_full = format!("{cargo_toml_cnt}\n{}", resolve_extra_cnt(&root, extra_cnt));
    write_generated_files(
        &path_cargo_toml,
        &cargo_toml_full,
        &path_lib_rs,
        content_to_gen,
    );
    run_cargo_check_steps(&crate_path, &CARGO_CHECK_STEPS);
}
#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    struct TmpDir(std::path::PathBuf);
    impl TmpDir {
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
    impl Drop for TmpDir {
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
        let dir = TmpDir::new();
        let path = dir.path().join("crate_dir");
        std::fs::create_dir_all(&path).expect("9b0e24f1");
        let guard = super::RemoveDirOnDrop { path: path.clone() };
        drop(guard);
        assert!(!path.exists());
    }
    #[test]
    fn resolve_extra_cnt_rewrites_workspace_dependencies() {
        let root = super::workspace_root();
        let resolved = super::resolve_extra_cnt(
            &root,
            "[dependencies]\nquote = { workspace = true }\npg_crud = { workspace = true, features = [\"test-utils\"] }",
        );
        assert!(resolved.contains("quote = { version = "), "420e5e9a");
        assert!(resolved.contains("pg_crud = { path = \""), "29aa4cf7");
        assert!(resolved.contains("features = [\"test-utils\"]"), "1ec15e06");
    }
    #[test]
    fn remove_dir_all_if_exists_accepts_missing_dir() {
        let dir = TmpDir::new();
        let path = dir.path().join("missing_dir");
        super::remove_dir_all_if_exists(&path, "f39c05aa");
        assert!(!path.exists());
    }
}
