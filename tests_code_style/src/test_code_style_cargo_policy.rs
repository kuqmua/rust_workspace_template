#![allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]

#[test]
fn test_crate_names_follow_workspace_vocabulary() {
    crate::code_style::assert_crate_manifest_cargo_policy(
        crate::types::StaticStr::from(constants_str::VALUE_4CE7AB5C),
        |path, parsed, ers| {
            let Some(name) = parsed
                .get(constants_str::PACKAGE)
                .and_then(|package| package.get(constants_str::NAME))
                .and_then(toml::Value::as_str)
            else {
                return;
            };
            let valid_chars = name
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_');
            let has_nonstandard_word = name.split('_').any(|part| {
                matches!(
                    part,
                    constants_str::VALUE_875B9380
                        | constants_str::VALUE_BA528516
                        | constants_str::POSTGRESQL
                        | constants_str::VALUE_D665A09C
                        | constants_str::VALUE_F4853BC8
                )
            }) || name.contains(constants_str::VALUE_2C90A5F7);
            if !valid_chars || has_nonstandard_word {
                ers.push(format!(
                    "{}: crate `{name}` must use snake_case workspace vocabulary (`dev`, `env`, `pg`, `accessor`, `macro_helpers`)",
                    path.display()
                ));
            }
        },
    );
}

#[test]
fn test_all_crates_have_publish_false() {
    crate::code_style::assert_crate_manifest_cargo_policy(
        crate::types::StaticStr::from(constants_str::F2A8C5D3),
        |path, parsed, ers| {
            let publish = parsed
                .get(constants_str::PACKAGE)
                .and_then(|v_1c7b4e9d| v_1c7b4e9d.get(constants_str::PUBLISH));
            let inherits_workspace = publish
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get(constants_str::WORKSPACE))
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
fn test_all_crates_have_workspace_lints() {
    crate::code_style::assert_crate_manifest_cargo_policy(
        crate::types::StaticStr::from(constants_str::D5F1A4E7),
        |path, parsed, ers| match parsed
            .get(constants_str::LINTS)
            .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
        {
            Some(lints_table) => {
                if lints_table.get(constants_str::WORKSPACE) != Some(&toml::Value::Boolean(true)) {
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
fn test_workspace_denies_single_call_production_functions() {
    let manifest = std::fs::read_to_string(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
        .expect("7b9e2c41 workspace manifest must be readable");
    let parsed = manifest
        .parse::<toml::Table>()
        .expect("c4a81f36 workspace manifest must be valid TOML");
    let single_call_fn = constants_str::SHARED_VALUES_CLIPPY_SINGLE_CALL_FN
        .rsplit_once(constants_str::PATH_SEPARATOR)
        .map_or(
            constants_str::SHARED_VALUES_CLIPPY_SINGLE_CALL_FN,
            |(_, lint)| lint,
        );
    let lint_level = parsed
        .get(constants_str::WORKSPACE)
        .and_then(|workspace| workspace.get(constants_str::LINTS))
        .and_then(|lints| lints.get(constants_str::CLIPPY))
        .and_then(|clippy| clippy.get(single_call_fn))
        .and_then(toml::Value::as_str);
    assert_eq!(
        lint_level,
        Some(constants_str::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND),
        "e13d7a90 workspace must deny single-call functions so private production helpers are localized as closures"
    );
}
#[test]
fn test_all_crates_use_edition_2024() {
    crate::code_style::assert_crate_manifest_cargo_policy(
        crate::types::StaticStr::from(constants_str::A3D7F1C8),
        |path, parsed, ers| {
            let edition = parsed
                .get(constants_str::PACKAGE)
                .and_then(|v_6d9f2a3e| v_6d9f2a3e.get(constants_str::EDITION));
            let inherits_workspace = edition
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get(constants_str::WORKSPACE))
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
fn test_all_crates_inherit_shared_package_metadata() {
    crate::code_style::assert_crate_manifest_cargo_policy(
        crate::types::StaticStr::from(constants_str::VALUE_EF65E2D1),
        |path, parsed, ers| {
            [
                constants_str::VERSION_ALT_3,
                constants_str::PUBLISH,
                constants_str::VALUE_E2885F2B,
                constants_str::VALUE_CC1D3B02,
                constants_str::EDITION,
            ]
            .into_iter()
            .for_each(|field| {
                let inherits_workspace = parsed
                    .get(constants_str::PACKAGE)
                    .and_then(|package| package.get(field))
                    .and_then(toml::Value::as_table)
                    .and_then(|table| table.get(constants_str::WORKSPACE))
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
fn test_check_workspace_dependencies_having_exact_version() {
    let workspace = crate::code_style::workspace_table_from_cargo_toml();
    crate::code_style::toml_val_as_table_ref(
        crate::types::TomlValueRef::from(
            workspace.as_ref().get(constants_str::DEPENDENCIES).expect(
                "2376f58e check_workspace_dependencies_having_exact_version invariant must hold",
            ),
        ),
        crate::types::StaticStr::from(constants_str::E117FA5A),
    )
    .as_ref()
    .values()
    .for_each(|dep| {
        let v_table = crate::code_style::toml_val_as_table_ref(
            crate::types::TomlValueRef::from(dep),
            crate::types::StaticStr::from(constants_str::CB693A3F),
        );
        if let Some(path_v) = v_table.get().get(constants_str::PATH_ALT_5) {
            match path_v {
                toml::Value::String(_) => {
                    match v_table.get().len() {
                        1 => (),
                        2 => crate::code_style::validate_workspace_dep_default_features(v_table),
                        _ => panic!("f6a3b9d1 {v_table:#?}"),
                    }
                    return;
                }
                toml::Value::Table(_)
                | toml::Value::Integer(_)
                | toml::Value::Float(_)
                | toml::Value::Boolean(_)
                | toml::Value::Datetime(_)
                | toml::Value::Array(_) => panic!("6ca03a1f"),
            }
        }
        match v_table
            .get()
            .get(constants_str::VERSION_ALT_3)
            .expect("d5b2b269 workspace dependency version invariant must hold")
        {
            toml::Value::String(version_string) => {
                let exact_three_part_version =
                    version_string.strip_prefix('=').is_some_and(|rest| {
                        let mut parts = rest.split('.');
                        (0usize..3usize).all(|_| {
                            parts
                                .next()
                                .and_then(|part| part.parse::<u64>().ok())
                                .is_some()
                        }) && parts.next().is_none()
                    });
                assert!(exact_three_part_version, "6640b9bf");
            }
            toml::Value::Table(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("a3410a37"),
        }
        crate::code_style::validate_workspace_dep_default_features(v_table);
        match v_table.get().len() {
            2 => {}
            3 => match v_table
                .get()
                .get(constants_str::FEATURES_ALT)
                .expect("473577d5 workspace dependency features invariant must hold")
            {
                &toml::Value::Array(_) => (),
                &toml::Value::String(_)
                | &toml::Value::Table(_)
                | &toml::Value::Integer(_)
                | &toml::Value::Float(_)
                | &toml::Value::Boolean(_)
                | &toml::Value::Datetime(_) => panic!("38ba32e9"),
            },
            _ => panic!("f1139378 {v_table:#?}"),
        }
    });
}
#[test]
fn test_external_workspace_dependencies_disable_default_features() {
    let workspace = crate::code_style::workspace_table_from_cargo_toml();
    let dependencies = crate::code_style::toml_val_as_table_ref(
        crate::types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(constants_str::DEPENDENCIES)
                .expect("9ac9fb4c external_workspace_dependencies_disable_default_features invariant must hold"),
        ),
        crate::types::StaticStr::from(constants_str::VALUE_5EECAACC),
    );
    let violations = dependencies
        .as_ref()
        .iter()
        .filter(|(_, dependency)| {
            dependency
                .as_table()
                .is_some_and(|table| table.contains_key(constants_str::VERSION_ALT_3))
                && !crate::code_style::workspace_dep_disables_default_features(
                    crate::types::TomlValueRef::from(*dependency),
                )
                .get()
        })
        .map(|(name, _)| name.as_str())
        .collect::<Vec<&str>>();
    assert!(violations.is_empty(), "b85e8406 {violations:#?}");
}
#[test]
fn test_workspace_dependency_default_feature_policy_rejects_missing_and_true_values() {
    let valid = toml::from_str::<toml::Value>(
        constants_str::VALUE_ED42B9D4,
    )
    .expect("227e7634 workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold");
    let missing = toml::from_str::<toml::Value>(
        constants_str::VALUE_79152E94,
    )
    .expect("0e82eab4 workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold");
    let enabled = toml::from_str::<toml::Value>(
        constants_str::VALUE_4CB11A6C,
    )
    .expect("e441c429 workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold");
    assert!(
        crate::code_style::workspace_dep_disables_default_features(crate::types::TomlValueRef::from(
            valid.get("dependency").expect("34136b6c workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold"),
        ))
        .get()
    );
    assert!(
        !crate::code_style::workspace_dep_disables_default_features(crate::types::TomlValueRef::from(
            missing.get("dependency").expect("e9b5ed95 workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold"),
        ))
        .get()
    );
    assert!(
        !crate::code_style::workspace_dep_disables_default_features(crate::types::TomlValueRef::from(
            enabled.get("dependency").expect("3e8046ef workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold"),
        ))
        .get()
    );
}
#[test]
fn test_workspace_uses_one_async_runtime() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let runtime_names = [
            constants_str::VALUE_24C2D1FB,
            constants_str::VALUE_B93D6F4A,
            constants_str::VALUE_43677C71,
            constants_str::VALUE_EC2B18D8,
            constants_str::TOKIO,
        ];
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
fn test_workspace_crates_do_not_enable_default_features() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
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
                    .get(constants_str::VALUE_37A8EEC1)
                    .filter(|features| !features.is_empty())
                    .map(|features| format!("{}: {features:?}", package.name))
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "32f78ac2 {violations:#?}");
    });
}
#[test]
fn test_workspace_dependency_catalog_has_no_unused_entries() {
    let workspace = crate::code_style::workspace_table_from_cargo_toml();
    let catalog = crate::code_style::toml_val_as_table_ref(
        crate::types::TomlValueRef::from(
            workspace.as_ref().get(constants_str::DEPENDENCIES).expect(
                "3e0ac397 workspace_dependency_catalog_has_no_unused_entries invariant must hold",
            ),
        ),
        crate::types::StaticStr::from(constants_str::VALUE_5EB013E8),
    );
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
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
                    .is_some_and(|table| table.contains_key(constants_str::VERSION_ALT_3))
                    && !used.contains(name.as_str())
            })
            .map(|(name, _)| name)
            .collect::<Vec<&String>>();
        assert!(violations.is_empty(), "57dd3daa {violations:#?}");
    });
}
#[test]
fn test_workspace_normal_dependency_graph_is_acyclic() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
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
fn test_library_crates_with_public_logic_own_tests() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
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
                    .collect::<Vec<&super::test_code_style_snapshot::RsSourceFile>>();
                let has_public_logic = source_files.iter().any(|source_file| {
                    crate::code_style::visit_syn_file(
                        crate::types::SynFileRef::from(source_file.ast().as_ref()),
                        super::source_analysis::PublicLogicVisitor::default(),
                    )
                    .get_found()
                    .get()
                });
                let has_owned_test = source_files.iter().any(|source_file| {
                    crate::code_style::visit_syn_file(
                        crate::types::SynFileRef::from(source_file.ast().as_ref()),
                        super::source_analysis::OwnedTestVisitor::default(),
                    )
                    .get_found()
                    .get()
                });
                (has_public_logic && !has_owned_test).then(|| package.name.to_string())
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "44cd2db7 {violations:#?}");
    });
}

#[test]
fn test_workspace_lint_allows_have_inline_reasons() {
    let source = std::fs::read_to_string(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
        .expect("68dcaf75 workspace_lint_allows_have_inline_reasons invariant must hold");
    let violations = crate::code_style::unjustified_workspace_lint_allows(
        crate::types::SourceTextRef::from(source.as_str()),
    );
    assert!(violations.is_empty(), "a94f0751 {violations:#?}");
}
#[test]
fn test_workspace_lint_allow_reason_policy_rejects_missing_and_empty_comments() {
    let violations = crate::code_style::unjustified_workspace_lint_allows(
        crate::types::SourceTextRef::from(constants_str::VALUE_05BB0EE4),
    );
    assert_eq!(violations.len(), 2usize);
}
#[test]
fn test_env_and_env_example_have_same_keys() {
    if !std::path::Path::new(constants_str::SERVER_ENV).is_file() {
        return;
    }
    let env_keys = crate::code_style::env_keys_from_file(crate::types::StaticStr::from(
        constants_str::SERVER_ENV,
    ));
    let example_keys = crate::code_style::env_keys_from_file(crate::types::StaticStr::from(
        constants_str::SERVER_DOT_ENV_EXAMPLE,
    ));
    let env_keys_set =
        crate::code_style::str_set(crate::types::SourceTextListRef::from(env_keys.as_slice()));
    let example_keys_set = crate::code_style::str_set(crate::types::SourceTextListRef::from(
        example_keys.as_slice(),
    ));
    let mut ers = crate::code_style::collect_missing_key_ers(
        crate::types::SourceTextListRef::from(env_keys.as_slice()),
        crate::types::SourceTextRefHashSet::from(example_keys_set.as_ref()),
        crate::types::StaticStr::from(constants_str::ENV),
        crate::types::StaticStr::from(constants_str::ENV_EXAMPLE),
    );
    ers.extend(crate::code_style::collect_missing_key_ers(
        crate::types::SourceTextListRef::from(example_keys.as_slice()),
        crate::types::SourceTextRefHashSet::from(env_keys_set.as_ref()),
        crate::types::StaticStr::from(constants_str::ENV_EXAMPLE),
        crate::types::StaticStr::from(constants_str::ENV),
    ));
    crate::code_style::assert_joined_ers_empty_sorted(
        crate::types::DiagnosticMsgsMutRef::from(&mut ers),
        crate::types::StaticStr::from(constants_str::C8D2F1A3),
    );
}
#[test]
fn test_server_has_one_tracked_environment_example() {
    assert!(
        !std::path::Path::new("../server/.envexample").exists(),
        "42fa780c"
    );
    assert!(
        std::path::Path::new(constants_str::SERVER_DOT_ENV_EXAMPLE).is_file(),
        "73be248d"
    );
}
#[test]
fn test_workspace_crates_must_use_workspace_dependencies() {
    crate::code_style::assert_cargo_toml_ers_empty(
        crate::types::StaticStr::from(constants_str::VALUE_5F8A6D17),
        |path, parsed, ers| {
            crate::code_style::collect_non_workspace_dep_ers(
                crate::types::PathRef::from(path),
                crate::types::TomlTableRef::from(parsed),
                crate::types::DiagnosticMsgsMutRef::from(ers),
            );
        },
    );
}
#[test]
fn test_target_specific_dependencies_must_use_workspace_dependencies() {
    let invalid_manifest = constants_str::VALUE_DB030A59.parse::<toml::Table>().expect(
        "b49e27c1 target_specific_dependencies_must_use_workspace_dependencies invariant must hold",
    );
    let mut invalid_ers = Vec::new();
    crate::code_style::collect_non_workspace_dep_ers(
        crate::types::PathRef::from(std::path::Path::new(constants_str::VALUE_EAE77D23)),
        crate::types::TomlTableRef::from(&invalid_manifest),
        crate::types::DiagnosticMsgsMutRef::from(&mut invalid_ers),
    );
    assert_eq!(invalid_ers.len(), 3usize);
    [
        constants_str::VALUE_33C3D866,
        constants_str::VALUE_6B80EB5B,
        constants_str::VALUE_DE87D770,
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

    let valid_manifest = constants_str::VALUE_98F81CDD.parse::<toml::Table>().expect(
        "8f1c3a6d target_specific_dependencies_must_use_workspace_dependencies invariant must hold",
    );
    let mut valid_ers = Vec::new();
    crate::code_style::collect_non_workspace_dep_ers(
        crate::types::PathRef::from(std::path::Path::new(constants_str::VALUE_EAE77D23)),
        crate::types::TomlTableRef::from(&valid_manifest),
        crate::types::DiagnosticMsgsMutRef::from(&mut valid_ers),
    );
    assert!(valid_ers.is_empty());
}
#[test]
fn test_workspace_dependencies_use_inline_table_style() {
    let regex =
        regex::Regex::new(constants_str::QUESTION_M_S_ASTERISK_A_ZA_Z0_9_PLUS_WORKSPACE_S_ASTERISK)
            .expect("ac15d6b9 workspace_dependencies_use_inline_table_style invariant must hold");
    let mut ers = Vec::new();
    crate::code_style::for_each_crate_manifest_file(|path| {
        let v = crate::code_style::cargo_toml_content(crate::types::PathRef::from(path))
            .expect("762c1d9e workspace_dependencies_use_inline_table_style invariant must hold");
        ers.extend(regex.find_iter(v.as_ref()).filter_map(|mtch| {
            let field = mtch
                .as_str()
                .split_once('.')
                .map(|(field, _suffix)| field.trim())
                .expect("34f5ed27 workspace_dependencies_use_inline_table_style invariant must hold");
            if [
                constants_str::DESCRIPTION,
                constants_str::EDITION,
                constants_str::VALUE_CC1D3B02,
                constants_str::PUBLISH,
                constants_str::VALUE_E2885F2B,
                constants_str::VERSION_ALT_3,
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
    crate::code_style::assert_joined_ers_empty_with_ctx(
        crate::types::SourceTextListRef::from(ers.as_slice()),
        crate::types::StaticStr::from(constants_str::D7A3C5B1),
        crate::types::SourceTextRef::from(constants_str::DOTTED_WORKSPACE_DEPENDENCY_STYLE_FOUND),
    );
}
#[test]
fn test_workspace_members_exist_on_disk() {
    let workspace = crate::code_style::workspace_table_from_cargo_toml();
    let members = crate::code_style::workspace_members_as_strs(
        crate::types::TomlTableRef::from(workspace.as_ref()),
        crate::types::StaticStr::from(constants_str::VALUE_7F3A1C4E),
    );
    let mut ers = crate::types::SourceTextList::from(
        members
            .as_slice()
            .iter()
            .filter_map(|member_str| {
                let path = std::path::Path::new(constants_str::TEXT_ALT_8)
                    .join(member_str)
                    .join(constants_str::CARGO_TOML);
                (!path.exists()).then(|| {
                    format!(
                        "member `{member_str}` Cargo.toml not found at {}",
                        path.display()
                    )
                })
            })
            .collect::<Vec<String>>(),
    );
    crate::code_style::assert_joined_ers_empty_sorted(
        crate::types::DiagnosticMsgsMutRef::from(&mut ers),
        crate::types::StaticStr::from(constants_str::A4E3B8D1),
    );
}
#[test]
fn test_workspace_crates_are_direct_children_of_workspace_root() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(
            "f7a31d9c workspace_crates_are_direct_children_of_workspace_root invariant must hold",
        );
    let mut violations = walkdir::WalkDir::new(workspace_root)
        .into_iter()
        .filter_entry(|entry| {
            entry.file_name() != constants_str::TARGET
                && entry.file_name() != constants_str::GIT
                && entry.file_name() != constants_str::WORKSPACE_SCAFFOLD_NODE_MODULES
        })
        .map(|entry| entry.unwrap_or_else(|error| panic!("b93c6e41 {error}")))
        .filter(|entry| !entry.file_type().is_dir() && entry.file_name() == constants_str::CARGO_TOML)
        .filter_map(|entry| {
            let crate_directory = entry.path().parent().expect("3de790a4 workspace_crates_are_direct_children_of_workspace_root invariant must hold");
            let relative = crate_directory
                .strip_prefix(workspace_root)
                .expect("c16f84b2 workspace_crates_are_direct_children_of_workspace_root invariant must hold");
            let parts = relative
                .components()
                .map(|component| component.as_os_str().to_string_lossy())
                .collect::<Vec<std::borrow::Cow<'_, str>>>();
            (parts.len() > constants_usize::ONE).then(|| {
                format!(
                    "nested crate `{}` must be moved to `{}` and `[workspace].members` must use `{}`",
                    relative.display(),
                    parts.join(constants_str::UNDERSCORE),
                    parts.join(constants_str::UNDERSCORE),
                )
            })
        })
        .collect::<Vec<String>>();
    violations.sort_unstable();
    assert!(
        violations.is_empty(),
        "5a2e8c71 workspace crates must be direct children of the workspace root. Nested paths such as `my_folder/my_logic/my_crate` are forbidden; flatten every path by joining its directory names with underscores:\n{}",
        violations.join("\n")
    );
}
#[test]
fn test_workspace_crate_src_modules_are_flat() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let mut violations = snapshot
            .workspace_metadata()
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .flat_map(|package| {
                let crate_directory =
                    package.manifest_path.as_std_path().parent().expect(
                        "92161504 workspace_crate_src_modules_are_flat invariant must hold",
                    );
                let source_directory = crate_directory.join(constants_str::SRC_ALT);
                if !source_directory.is_dir() {
                    return Vec::new();
                }
                walkdir::WalkDir::new(source_directory.as_path())
                    .min_depth(constants_usize::ONE)
                    .into_iter()
                    .map(|entry| entry.unwrap_or_else(|error| panic!("49938956 {error}")))
                    .filter(|entry| !entry.file_type().is_dir())
                    .filter(|entry| {
                        entry.path().extension().and_then(std::ffi::OsStr::to_str)
                            == Some(constants_str::RS)
                    })
                    .filter_map(move |entry| {
                        (entry.path().parent() != Some(source_directory.as_path()))
                            .then(|| entry.path().display().to_string())
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        violations.sort_unstable();
        assert!(
            violations.is_empty(),
            "037f95b6 Rust module files must be stored directly in each crate's src directory:\n{}",
            violations.join("\n")
        );
    });
}
#[test]
fn test_workspace_members_sorted_alphabetically() {
    let workspace = crate::code_style::workspace_table_from_cargo_toml();
    let members_vec = crate::code_style::workspace_members_as_strs(
        crate::types::TomlTableRef::from(workspace.as_ref()),
        crate::types::StaticStr::from(constants_str::C1D4F7A2),
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
    crate::code_style::assert_joined_ers_empty_with_ctx(
        crate::types::SourceTextListRef::from(ers.as_slice()),
        crate::types::StaticStr::from(constants_str::B7C2E5F8),
        crate::types::SourceTextRef::from(constants_str::MEMBERS_NOT_SORTED),
    );
}

#[test]
fn test_workspace_packages_have_at_most_one_binary_target() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let violations = snapshot
            .workspace_metadata()
            .get()
            .packages
            .iter()
            .filter(|package| workspace_names.as_ref().contains(package.name.as_str()))
            .filter_map(|package| {
                let binary_names = package
                    .targets
                    .iter()
                    .filter(|target| target.kind.contains(&cargo_metadata::TargetKind::Bin))
                    .map(|target| target.name.as_str())
                    .collect::<Vec<&str>>();
                (binary_names.len() > constants_usize::ONE).then(|| {
                    format!(
                        "package `{}` owns multiple binaries: {}; move each additional binary into a dedicated workspace crate",
                        package.name,
                        binary_names.join(", ")
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "c80384b0 {violations:#?}");
    });
}
