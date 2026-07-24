#![allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]

#[test]
fn all_crates_have_publish_false() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from(str_constants::F2A8C5D3),
        |path, parsed, ers| {
            let publish = parsed
                .get(str_constants::PACKAGE)
                .and_then(|v_1c7b4e9d| v_1c7b4e9d.get(str_constants::PUBLISH));
            let inherits_workspace = publish
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get(str_constants::WORKSPACE))
                == Some(&toml::Value::Boolean(true));
            if !inherits_workspace {
                ers.push(format!(
                    "{}: missing `publish.workspace = true`",
                    path.display()
                ));
            }
        },
    );
}
#[test]
fn all_crates_have_workspace_lints() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from(str_constants::D5F1A4E7),
        |path, parsed, ers| match parsed
            .get(str_constants::LINTS)
            .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
        {
            Some(lints_table) => {
                if lints_table.get(str_constants::WORKSPACE) != Some(&toml::Value::Boolean(true)) {
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
        super::types::StaticStr::from(str_constants::A3D7F1C8),
        |path, parsed, ers| {
            let edition = parsed
                .get(str_constants::PACKAGE)
                .and_then(|v_6d9f2a3e| v_6d9f2a3e.get(str_constants::EDITION));
            let inherits_workspace = edition
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get(str_constants::WORKSPACE))
                == Some(&toml::Value::Boolean(true));
            if !inherits_workspace {
                ers.push(format!(
                    "{}: missing `edition.workspace = true`",
                    path.display()
                ));
            }
        },
    );
}
#[test]
fn all_crates_inherit_shared_package_metadata() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from("26bd454d"),
        |path, parsed, ers| {
            [
                "version",
                str_constants::PUBLISH,
                "repository",
                "license",
                str_constants::EDITION,
            ]
            .into_iter()
            .for_each(|field| {
                let inherits_workspace = parsed
                    .get(str_constants::PACKAGE)
                    .and_then(|package| package.get(field))
                    .and_then(toml::Value::as_table)
                    .and_then(|table| table.get(str_constants::WORKSPACE))
                    == Some(&toml::Value::Boolean(true));
                if !inherits_workspace {
                    ers.push(format!(
                        "{}: `{field}` must inherit from `[workspace.package]`",
                        path.display()
                    ));
                }
            });
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
                .get(str_constants::DEPENDENCIES)
                .expect("2376f58e"),
        ),
        super::types::StaticStr::from(str_constants::E117FA5A),
    )
    .as_ref()
    .values()
    .for_each(|dep| super::validate_workspace_dep_spec(super::types::TomlValueRef::from(dep)));
}
#[test]
fn external_workspace_dependencies_disable_default_features() {
    let workspace = super::workspace_table_from_cargo_toml();
    let dependencies = super::toml_val_as_table_ref(
        super::types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(str_constants::DEPENDENCIES)
                .expect("9ac9fb4c"),
        ),
        super::types::StaticStr::from("2db3165f"),
    );
    let violations = dependencies
        .as_ref()
        .iter()
        .filter(|(_, dependency)| {
            dependency
                .as_table()
                .is_some_and(|table| table.contains_key(str_constants::VERSION_ALT_3))
                && !super::workspace_dep_disables_default_features(
                    super::types::TomlValueRef::from(*dependency),
                )
                .get()
        })
        .map(|(name, _)| name.as_str())
        .collect::<Vec<&str>>();
    assert!(violations.is_empty(), "b85e8406 {violations:#?}");
}
#[test]
fn workspace_dependency_default_feature_policy_rejects_missing_and_true_values() {
    let valid = toml::from_str::<toml::Value>(
        r#"[dependency]
version = "=1.2.3"
default-features = false
"#,
    )
    .expect("227e7634");
    let missing = toml::from_str::<toml::Value>(
        r#"[dependency]
version = "=1.2.3"
"#,
    )
    .expect("0e82eab4");
    let enabled = toml::from_str::<toml::Value>(
        r#"[dependency]
version = "=1.2.3"
default-features = true
"#,
    )
    .expect("e441c429");
    assert!(
        super::workspace_dep_disables_default_features(super::types::TomlValueRef::from(
            valid.get("dependency").expect("34136b6c"),
        ))
        .get()
    );
    assert!(
        !super::workspace_dep_disables_default_features(super::types::TomlValueRef::from(
            missing.get("dependency").expect("e9b5ed95"),
        ))
        .get()
    );
    assert!(
        !super::workspace_dep_disables_default_features(super::types::TomlValueRef::from(
            enabled.get("dependency").expect("3e8046ef"),
        ))
        .get()
    );
}
#[test]
fn workspace_uses_one_async_runtime() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let runtime_names = ["async-std", "glommio", "monoio", "smol", "tokio"];
        let workspace_names = snapshot.workspace_crate_names();
        let used = snapshot
            .workspace_metadata()
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .flat_map(|package| package.dependencies.iter())
            .map(|dependency| dependency.name.as_str())
            .filter(|name| runtime_names.contains(name))
            .collect::<std::collections::BTreeSet<&str>>();
        assert_eq!(
            used,
            std::collections::BTreeSet::from(["tokio"]),
            "af25689c"
        );
    });
}
#[test]
fn workspace_crates_do_not_enable_default_features() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let violations = snapshot
            .workspace_metadata()
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .filter_map(|package| {
                package
                    .features
                    .get("default")
                    .filter(|features| !features.is_empty())
                    .map(|features| format!("{}: {features:?}", package.name))
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "32f78ac2 {violations:#?}");
    });
}
#[test]
fn workspace_dependency_catalog_has_no_unused_entries() {
    let workspace = super::workspace_table_from_cargo_toml();
    let catalog = super::toml_val_as_table_ref(
        super::types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(str_constants::DEPENDENCIES)
                .expect("3e0ac397"),
        ),
        super::types::StaticStr::from("0c6249e6"),
    );
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let used = snapshot
            .workspace_metadata()
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .flat_map(|package| package.dependencies.iter())
            .map(|dependency| dependency.name.as_str())
            .collect::<std::collections::BTreeSet<&str>>();
        let violations = catalog
            .as_ref()
            .iter()
            .filter(|(name, dependency)| {
                dependency
                    .as_table()
                    .is_some_and(|table| table.contains_key(str_constants::VERSION_ALT_3))
                    && !used.contains(name.as_str())
            })
            .map(|(name, _)| name)
            .collect::<Vec<&String>>();
        assert!(violations.is_empty(), "57dd3daa {violations:#?}");
    });
}
#[test]
fn workspace_normal_dependency_graph_is_acyclic() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let metadata = snapshot.workspace_metadata();
        let workspace_names = snapshot.workspace_crate_names();
        let mut remaining = metadata
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .map(|package| {
                (
                    package.name.as_str(),
                    package
                        .dependencies
                        .iter()
                        .filter(|dependency| {
                            dependency.kind == cargo_metadata::DependencyKind::Normal
                                && workspace_names.as_ref().contains(dependency.name.as_str())
                        })
                        .map(|dependency| dependency.name.as_str())
                        .collect::<std::collections::BTreeSet<&str>>(),
                )
            })
            .collect::<std::collections::BTreeMap<&str, std::collections::BTreeSet<&str>>>();
        loop {
            let ready = remaining
                .iter()
                .filter(|(_, dependencies)| dependencies.is_empty())
                .map(|(name, _)| *name)
                .collect::<Vec<&str>>();
            if ready.is_empty() {
                break;
            }
            ready.iter().for_each(|name| {
                let _removed_dependencies = remaining.remove(name);
            });
            remaining.values_mut().for_each(|dependencies| {
                ready.iter().for_each(|name| {
                    let _was_removed = dependencies.remove(name);
                });
            });
        }
        assert!(
            remaining.is_empty(),
            "85e729af dependency cycle: {remaining:#?}"
        );
    });
}
#[test]
fn library_crates_with_public_logic_own_tests() {
    struct TestOwnershipException {
        crate_name: &'static str,
        reason: &'static str,
    }
    let exceptions = [
        TestOwnershipException {
            crate_name: "app_state",
            reason: "the crate is a facade over generated state traits",
        },
        TestOwnershipException {
            crate_name: "config_lib_macros",
            reason: "the proc-macro is exercised by config_lib integration tests",
        },
        TestOwnershipException {
            crate_name: "generate_getter_traits_for_struct_fields",
            reason: "the generator is exercised by config_lib compile-time expansion",
        },
        TestOwnershipException {
            crate_name: "try_from_env",
            reason: "the proc-macro is exercised by config_lib integration tests",
        },
        TestOwnershipException {
            crate_name: "location_macros",
            reason: "the proc-macro is exercised by location_lib tests",
        },
        TestOwnershipException {
            crate_name: "naming",
            reason: "the crate is a facade over tested naming crates",
        },
        TestOwnershipException {
            crate_name: "naming_common_macros",
            reason: "the generated macro surface is exercised by naming_common tests",
        },
        TestOwnershipException {
            crate_name: "naming_macros",
            reason: "the proc-macro is exercised by naming tests",
        },
        TestOwnershipException {
            crate_name: "optml",
            reason: "the proc-macro is exercised by downstream derive users",
        },
        TestOwnershipException {
            crate_name: "pg_crud",
            reason: "the crate is a facade over tested CRUD crates",
        },
        TestOwnershipException {
            crate_name: "pg_crud_common_macros",
            reason: "the macro surface is exercised by pg_crud_common tests",
        },
        TestOwnershipException {
            crate_name: "pg_crud_macros_common",
            reason: "the generator support crate is exercised by generated contract tests",
        },
        TestOwnershipException {
            crate_name: "pg_crud_macros_common_macros",
            reason: "the macro surface is exercised by generated CRUD tests",
        },
        TestOwnershipException {
            crate_name: "generate_pg_table",
            reason: "the proc-macro is exercised by generate_pg_table_test",
        },
        TestOwnershipException {
            crate_name: "pg_types",
            reason: "the crate is a facade over tested PostgreSQL type crates",
        },
        TestOwnershipException {
            crate_name: "generate_pg_types",
            reason: "the proc-macro is exercised by generate_pg_types_test",
        },
        TestOwnershipException {
            crate_name: "pg_types_chrono_net",
            reason: "the crate exports generated PostgreSQL type adapters",
        },
        TestOwnershipException {
            crate_name: "pg_types_common",
            reason: "the crate exports generated PostgreSQL type adapters",
        },
        TestOwnershipException {
            crate_name: "pg_types_numeric",
            reason: "the crate exports generated PostgreSQL type adapters",
        },
        TestOwnershipException {
            crate_name: "pg_types_text_misc",
            reason: "the crate exports generated PostgreSQL type adapters",
        },
        TestOwnershipException {
            crate_name: "generate_where_filters",
            reason: "the proc-macro is exercised by generate_where_filters_test",
        },
        TestOwnershipException {
            crate_name: "server_app_state_macros",
            reason: "the proc-macro is exercised by server_app_state tests",
        },
        TestOwnershipException {
            crate_name: "str_constants_macros",
            reason: "the proc-macro is exercised by str_constants tests",
        },
        TestOwnershipException {
            crate_name: "to_err_string_macros",
            reason: "the proc-macro is exercised by to_err_string tests",
        },
        TestOwnershipException {
            crate_name: "token_patterns_macros",
            reason: "the proc-macro is exercised by token_patterns tests",
        },
    ];
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let workspace_directories = snapshot
            .workspace_metadata()
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .filter_map(|package| package.manifest_path.as_std_path().parent())
            .collect::<Vec<&std::path::Path>>();
        let violations = snapshot
            .workspace_metadata()
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .filter(|package| {
                package.targets.iter().any(|target| {
                    target.kind.iter().any(|kind| {
                        matches!(
                            kind,
                            cargo_metadata::TargetKind::Lib | cargo_metadata::TargetKind::ProcMacro
                        )
                    })
                })
            })
            .filter_map(|package| {
                let crate_directory = package.manifest_path.as_std_path().parent()?;
                let source_files = snapshot
                    .rs_files()
                    .iter()
                    .filter(|source_file| {
                        let source_path = source_file.path().as_ref();
                        source_path.starts_with(crate_directory)
                            && !workspace_directories.iter().any(|other_directory| {
                                *other_directory != crate_directory
                                    && source_path.starts_with(other_directory)
                            })
                    })
                    .collect::<Vec<&super::snapshot::RsSourceFile>>();
                let has_public_logic = source_files.iter().any(|source_file| {
                    super::visit_syn_file(
                        super::types::SynFileRef::from(source_file.ast().as_ref()),
                        super::PublicLogicVisitor::default(),
                    )
                    .found
                    .get()
                });
                let has_owned_test = source_files.iter().any(|source_file| {
                    super::visit_syn_file(
                        super::types::SynFileRef::from(source_file.ast().as_ref()),
                        super::OwnedTestVisitor::default(),
                    )
                    .found
                    .get()
                });
                let reviewed = exceptions.iter().any(|exception| {
                    package.name == exception.crate_name && !exception.reason.is_empty()
                });
                (has_public_logic && !has_owned_test && !reviewed).then(|| package.name.to_string())
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "44cd2db7 {violations:#?}");
    });
}
#[test]
fn workspace_lint_allows_have_inline_reasons() {
    let source = std::fs::read_to_string(str_constants::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
        .expect("68dcaf75");
    let violations = super::unjustified_workspace_lint_allows(super::types::SourceTextRef::from(
        source.as_str(),
    ));
    assert!(violations.is_empty(), "a94f0751 {violations:#?}");
}
#[test]
fn workspace_lint_allow_reason_policy_rejects_missing_and_empty_comments() {
    let violations = super::unjustified_workspace_lint_allows(super::types::SourceTextRef::from(
        r#"
[workspace.lints.rust]
unsafe_code = "deny"
dead_code = "allow"
[workspace.lints.clippy]
panic = "allow" #
unwrap_used = "allow" # tests check unwrap failures separately
[profile.dev]
debug = true
"#,
    ));
    assert_eq!(violations.len(), 2usize);
}
#[test]
fn env_and_env_example_have_same_keys() {
    let env_keys =
        super::env_keys_from_file(super::types::StaticStr::from(str_constants::SERVER_ENV));
    let example_keys = super::env_keys_from_file(super::types::StaticStr::from(
        str_constants::SERVER_DOT_ENV_EXAMPLE,
    ));
    let env_keys_set = super::str_set(super::types::SourceTextListRef::from(env_keys.as_slice()));
    let example_keys_set = super::str_set(super::types::SourceTextListRef::from(
        example_keys.as_slice(),
    ));
    let mut ers = super::collect_missing_key_ers(
        super::types::SourceTextListRef::from(env_keys.as_slice()),
        super::types::StdSourceTextRefSet::from(example_keys_set.as_ref()),
        super::types::StaticStr::from(str_constants::ENV),
        super::types::StaticStr::from(str_constants::ENV_EXAMPLE),
    );
    ers.extend(super::collect_missing_key_ers(
        super::types::SourceTextListRef::from(example_keys.as_slice()),
        super::types::StdSourceTextRefSet::from(env_keys_set.as_ref()),
        super::types::StaticStr::from(str_constants::ENV_EXAMPLE),
        super::types::StaticStr::from(str_constants::ENV),
    ));
    super::assert_joined_ers_empty_sorted(
        super::types::DiagnosticMsgsMutRef::from(&mut ers),
        super::types::StaticStr::from(str_constants::C8D2F1A3),
    );
}
#[test]
fn server_has_one_tracked_environment_example() {
    assert!(
        !std::path::Path::new("../server/.envexample").exists(),
        "42fa780c"
    );
    assert!(
        std::path::Path::new(str_constants::SERVER_DOT_ENV_EXAMPLE).is_file(),
        "73be248d"
    );
}
#[test]
fn workspace_crates_must_use_workspace_dependencies() {
    super::assert_cargo_toml_ers_empty(
        super::types::StaticStr::from(str_constants::VALUE_5F8A6D17),
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
fn target_specific_dependencies_must_use_workspace_dependencies() {
    let invalid_manifest = r#"
[target.'cfg(target_arch = "wasm32")'.dependencies]
serde = "1"

[target.'cfg(target_arch = "wasm32")'.dev-dependencies]
serde_json = { path = "../serde_json" }

[target.'cfg(target_arch = "wasm32")'.build-dependencies]
toml = { version = "1" }
"#
    .parse::<toml::Table>()
    .expect("b49e27c1");
    let mut invalid_ers = Vec::new();
    super::collect_non_workspace_dep_ers(
        super::types::StdPathRef::from(std::path::Path::new("fixture/Cargo.toml")),
        super::types::TomlTableRef::from(&invalid_manifest),
        super::types::DiagnosticMsgsMutRef::from(&mut invalid_ers),
    );
    assert_eq!(invalid_ers.len(), 3usize);
    [
        "target.cfg(target_arch = \"wasm32\").dependencies",
        "target.cfg(target_arch = \"wasm32\").dev-dependencies",
        "target.cfg(target_arch = \"wasm32\").build-dependencies",
    ]
    .into_iter()
    .for_each(|section| {
        assert!(
            invalid_ers
                .iter()
                .any(|error| error.contains(format!("[{section}]").as_str())),
            "d2a74c90"
        );
    });

    let valid_manifest = r#"
[target.'cfg(target_arch = "wasm32")'.dependencies]
serde = { workspace = true }
"#
    .parse::<toml::Table>()
    .expect("8f1c3a6d");
    let mut valid_ers = Vec::new();
    super::collect_non_workspace_dep_ers(
        super::types::StdPathRef::from(std::path::Path::new("fixture/Cargo.toml")),
        super::types::TomlTableRef::from(&valid_manifest),
        super::types::DiagnosticMsgsMutRef::from(&mut valid_ers),
    );
    assert!(valid_ers.is_empty());
}
#[test]
fn workspace_dependencies_use_inline_table_style() {
    let regex =
        regex::Regex::new(str_constants::QUESTION_M_S_ASTERISK_A_ZA_Z0_9_PLUS_WORKSPACE_S_ASTERISK)
            .expect("ac15d6b9");
    let mut ers = Vec::new();
    super::for_each_crate_manifest_file(|path| {
        let v = super::cargo_toml_content(super::types::StdPathRef::from(path)).expect("762c1d9e");
        ers.extend(regex.find_iter(v.as_ref()).filter_map(|mtch| {
            let field = mtch
                .as_str()
                .split_once('.')
                .map(|(field, _suffix)| field.trim())
                .expect("34f5ed27");
            if [
                "description",
                str_constants::EDITION,
                "license",
                str_constants::PUBLISH,
                "repository",
                "version",
            ]
            .contains(&field)
            {
                return None;
            }
            let line_number = v
                .as_ref()
                .bytes()
                    .take(mtch.start())
                    .filter(|byte| *byte == b'\n')
                    .count()
                    .saturating_add(1);
                Some(format!(
                    "{}:{line_number} use `dep = {{ workspace = true }}` instead of dotted workspace dependency style",
                    path.display()
                ))
            }));
    });
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(str_constants::D7A3C5B1),
        super::types::SourceTextRef::from(str_constants::DOTTED_WORKSPACE_DEPENDENCY_STYLE_FOUND),
    );
}
#[test]
fn workspace_members_exist_on_disk() {
    let workspace = super::workspace_table_from_cargo_toml();
    let members = super::workspace_members_as_strs(
        super::types::TomlTableRef::from(workspace.as_ref()),
        super::types::StaticStr::from(str_constants::VALUE_7F3A1C4E),
    );
    let mut ers = super::collect_workspace_member_missing_cargo_toml_ers(
        super::types::SourceTextListRef::from(members.as_slice()),
    );
    super::assert_joined_ers_empty_sorted(
        super::types::DiagnosticMsgsMutRef::from(&mut ers),
        super::types::StaticStr::from(str_constants::A4E3B8D1),
    );
}
#[test]
fn workspace_members_sorted_alphabetically() {
    let workspace = super::workspace_table_from_cargo_toml();
    let members_vec = super::workspace_members_as_strs(
        super::types::TomlTableRef::from(workspace.as_ref()),
        super::types::StaticStr::from(str_constants::C1D4F7A2),
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
        super::types::StaticStr::from(str_constants::B7C2E5F8),
        super::types::SourceTextRef::from(str_constants::MEMBERS_NOT_SORTED),
    );
}
