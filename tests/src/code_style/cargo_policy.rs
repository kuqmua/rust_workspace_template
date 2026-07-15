#[test]
fn all_crates_have_publish_false() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr(str_constants::expr::S_1273),
        |path, parsed, ers| {
            let publish = parsed
                .get(str_constants::expr::S_1580)
                .and_then(|v_1c7b4e9d| v_1c7b4e9d.get(str_constants::expr::S_1642));
            if publish != Some(&toml::Value::Boolean(false)) {
                ers.push(format!("{}: missing `publish = false`", path.display()));
            }
        },
    );
}
#[test]
fn all_crates_have_workspace_lints() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr(str_constants::expr::S_1131),
        |path, parsed, ers| match parsed
            .get(str_constants::expr::S_1460)
            .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
        {
            Some(lints_table) => {
                if lints_table.get(str_constants::expr::S_1913) != Some(&toml::Value::Boolean(true))
                {
                    ers.push(format!(
                        "{}: [lints] missing `workspace = true`",
                        path.display()
                    ));
                }
            }
            None => {
                ers.push(format!("{}: missing [lints] section", path.display()));
            }
        },
    );
}
#[test]
fn all_crates_use_edition_2024() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr(str_constants::expr::S_0878),
        |path, parsed, ers| {
            let edition = parsed
                .get(str_constants::expr::S_1580)
                .and_then(|v_6d9f2a3e| v_6d9f2a3e.get(str_constants::expr::S_1238))
                .and_then(toml::Value::as_str);
            if edition != Some(str_constants::expr::S_0219) {
                ers.push(format!("{}: edition is not \"2024\"", path.display()));
            }
        },
    );
}
#[test]
fn check_workspace_dependencies_having_exact_version() {
    let workspace = super::workspace_table_from_cargo_toml();
    super::toml_val_as_table_ref(
        super::types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(str_constants::expr::S_1175)
                .expect("2376f58e"),
        ),
        super::types::StaticStr(str_constants::expr::S_1206),
    )
    .as_ref()
    .values()
    .for_each(|dep| super::validate_workspace_dep_spec(super::types::TomlValueRef::from(dep)));
}
#[test]
fn env_and_envexample_have_same_keys() {
    let env_keys = super::env_keys_from_file(super::types::StaticStr(str_constants::expr::S_0062));
    let example_keys =
        super::env_keys_from_file(super::types::StaticStr(str_constants::expr::S_0063));
    let env_keys_set = super::str_set(super::types::SourceTextListRef::from(env_keys.as_slice()));
    let example_keys_set = super::str_set(super::types::SourceTextListRef::from(
        example_keys.as_slice(),
    ));
    let mut ers = super::collect_missing_key_ers(
        super::types::SourceTextListRef::from(env_keys.as_slice()),
        super::types::StdSourceTextRefSet::from(example_keys_set.as_ref()),
        super::types::StaticStr(str_constants::expr::S_0075),
        super::types::StaticStr(str_constants::expr::S_0077),
    );
    ers.extend(super::collect_missing_key_ers(
        super::types::SourceTextListRef::from(example_keys.as_slice()),
        super::types::StdSourceTextRefSet::from(env_keys_set.as_ref()),
        super::types::StaticStr(str_constants::expr::S_0077),
        super::types::StaticStr(str_constants::expr::S_0075),
    ));
    super::assert_joined_ers_empty_sorted(
        super::types::DiagnosticMsgsMutRef::from(&mut ers),
        super::types::StaticStr(str_constants::expr::S_1060),
    );
}
#[test]
fn workspace_crates_must_use_workspace_dependencies() {
    super::assert_cargo_toml_ers_empty(
        super::types::StaticStr(str_constants::expr::S_0386),
        |path, parsed, ers| {
            super::collect_non_workspace_dep_ers(
                super::types::StdPathRef::from(path),
                super::types::TomlTableRef::from(parsed),
                super::types::DiagnosticMsgsMutRef::from(ers),
            );
        },
    );
}
#[test]
fn workspace_dependencies_use_inline_table_style() {
    let regex = regex::Regex::new(str_constants::expr::S_0039).expect("ac15d6b9");
    let mut ers = Vec::new();
    super::for_each_crate_manifest_file(|path| {
        let v = super::cargo_toml_content(super::types::StdPathRef::from(path)).expect("762c1d9e");
        ers.extend(regex.find_iter(v.as_ref()).map(|mtch| {
            let line_number = v
                .as_ref()
                .bytes()
                    .take(mtch.start())
                    .filter(|byte| *byte == b'\n')
                    .count()
                    .saturating_add(1);
                format!(
                    "{}:{line_number} use `dep = {{ workspace = true }}` instead of dotted workspace dependency style",
                    path.display()
                )
            }));
    });
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr(str_constants::expr::S_1135),
        super::types::SourceTextRef::from(str_constants::expr::S_1197),
    );
}
#[test]
fn workspace_members_exist_on_disk() {
    let workspace = super::workspace_table_from_cargo_toml();
    let members = super::workspace_members_as_strs(
        super::types::TomlTableRef::from(workspace.as_ref()),
        super::types::StaticStr(str_constants::expr::S_0462),
    );
    let mut ers = super::collect_workspace_member_missing_cargo_toml_ers(
        super::types::SourceTextListRef::from(members.as_slice()),
    );
    super::assert_joined_ers_empty_sorted(
        super::types::DiagnosticMsgsMutRef::from(&mut ers),
        super::types::StaticStr(str_constants::expr::S_0884),
    );
}
#[test]
fn workspace_members_sorted_alphabetically() {
    let workspace = super::workspace_table_from_cargo_toml();
    let members_vec = super::workspace_members_as_strs(
        super::types::TomlTableRef::from(workspace.as_ref()),
        super::types::StaticStr(str_constants::expr::S_1038),
    );
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
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr(str_constants::expr::S_0994),
        super::types::SourceTextRef::from(str_constants::expr::S_1513),
    );
}
