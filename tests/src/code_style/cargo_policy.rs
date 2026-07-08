#[test]
fn all_crates_have_publish_false() {
    super::assert_root_workspace_cargo_policy("f2a8c5d3", |path, parsed, ers| {
        let publish = parsed
            .get("package")
            .and_then(|v_1c7b4e9d| v_1c7b4e9d.get("publish"));
        if publish != Some(&toml::Value::Boolean(false)) {
            ers.push(format!("{}: missing `publish = false`", path.display()));
        }
    });
}
#[test]
fn all_crates_have_workspace_lints() {
    super::assert_root_workspace_cargo_policy("d5f1a4e7", |path, parsed, ers| {
        match parsed
            .get("lints")
            .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
        {
            Some(lints_tbl) => {
                if lints_tbl.get("workspace") != Some(&toml::Value::Boolean(true)) {
                    ers.push(format!(
                        "{}: [lints] missing `workspace = true`",
                        path.display()
                    ));
                }
            }
            None => {
                ers.push(format!("{}: missing [lints] section", path.display()));
            }
        }
    });
}
#[test]
fn all_crates_use_edition_2024() {
    super::assert_root_workspace_cargo_policy("a3d7f1c8", |path, parsed, ers| {
        let edition = parsed
            .get("package")
            .and_then(|v_6d9f2a3e| v_6d9f2a3e.get("edition"))
            .and_then(toml::Value::as_str);
        if edition != Some("2024") {
            ers.push(format!("{}: edition is not \"2024\"", path.display()));
        }
    });
}
#[test]
fn check_workspace_dependencies_having_exact_version() {
    let workspace = super::workspace_tbl_from_cargo_toml();
    super::toml_val_as_tbl_ref(workspace.get("dependencies").expect("2376f58e"), "e117fa5a")
        .values()
        .for_each(super::validate_workspace_dep_spec);
}
#[test]
fn env_and_envexample_have_same_keys() {
    let env_keys = super::env_keys_from_file("../server/.env");
    let example_keys = super::env_keys_from_file("../server/.envexample");
    let env_keys_set = super::str_set(&env_keys);
    let example_keys_set = super::str_set(&example_keys);
    let mut ers =
        super::collect_missing_key_ers(&env_keys, &example_keys_set, ".env", ".envexample");
    ers.extend(super::collect_missing_key_ers(
        &example_keys,
        &env_keys_set,
        ".envexample",
        ".env",
    ));
    super::assert_joined_ers_empty_sorted(&mut ers, "c8d2f1a3");
}
#[test]
fn workspace_crates_must_use_workspace_dependencies() {
    super::assert_cargo_toml_ers_empty(
        &[
            "../Cargo.toml", //workspace
        ],
        "5f8a6d17",
        super::collect_non_workspace_dep_ers,
    );
}
#[test]
fn workspace_dependencies_use_inline_table_style() {
    let rgx =
        regex::Regex::new(r"(?m)^\s*[A-Za-z0-9_-]+\.workspace\s*=\s*true\s*$").expect("ac15d6b9");
    let mut ers = Vec::new();
    super::for_each_cargo_toml_project_file(&[], |path| {
        let v = super::cargo_toml_content(path).expect("762c1d9e");
        ers.extend(rgx.find_iter(&v).map(|mtch| {
                let line_nbr = v
                    .bytes()
                    .take(mtch.start())
                    .filter(|byte| *byte == b'\n')
                    .count()
                    .saturating_add(1);
                format!(
                    "{}:{line_nbr} use `dep = {{ workspace = true }}` instead of dotted workspace dependency style",
                    path.display()
                )
            }));
    });
    super::assert_joined_ers_empty_with_ctx(
        &ers,
        "d7a3c5b1",
        "dotted workspace dependency style found:",
    );
}
#[test]
fn workspace_members_exist_on_disk() {
    let workspace = super::workspace_tbl_from_cargo_toml();
    let members = super::workspace_members_as_strs(&workspace, "7f3a1c4e");
    let mut ers = super::collect_workspace_member_missing_cargo_toml_ers(&members);
    super::assert_joined_ers_empty_sorted(&mut ers, "a4e3b8d1");
}
#[test]
fn workspace_members_sorted_alphabetically() {
    let workspace = super::workspace_tbl_from_cargo_toml();
    let members_vec = super::workspace_members_as_strs(&workspace, "c1d4f7a2");
    let mut sorted = members_vec.clone();
    sorted.sort_unstable();
    let ers = members_vec
        .iter()
        .zip(sorted.iter())
        .enumerate()
        .filter(|(_, (got, expected))| got != expected)
        .map(|(k_4b1e6a8c, (got, expected))| {
            format!("index {k_4b1e6a8c}: got `{got}`, expected `{expected}`")
        })
        .collect::<Vec<String>>();
    super::assert_joined_ers_empty_with_ctx(&ers, "b7c2e5f8", "members not sorted:");
}
