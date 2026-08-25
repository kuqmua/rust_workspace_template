#![allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]

#[test]
fn crate_names_follow_workspace_vocabulary() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from(constants_str::VALUE_4CE7AB5C),
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
fn all_crates_have_publish_false() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from(constants_str::F2A8C5D3),
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
fn all_crates_have_workspace_lints() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from(constants_str::D5F1A4E7),
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
fn all_crates_use_edition_2024() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from(constants_str::A3D7F1C8),
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
fn all_crates_inherit_shared_package_metadata() {
    super::assert_crate_manifest_cargo_policy(
        super::types::StaticStr::from(constants_str::VALUE_EF65E2D1),
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
fn check_workspace_dependencies_having_exact_version() {
    let workspace = super::workspace_table_from_cargo_toml();
    super::toml_val_as_table_ref(
        super::types::TomlValueRef::from(
            workspace.as_ref().get(constants_str::DEPENDENCIES).expect(
                "2376f58e check_workspace_dependencies_having_exact_version invariant must hold",
            ),
        ),
        super::types::StaticStr::from(constants_str::E117FA5A),
    )
    .as_ref()
    .values()
    .for_each(|dep| {
        let v_table = super::toml_val_as_table_ref(
            super::types::TomlValueRef::from(dep),
            super::types::StaticStr::from(constants_str::CB693A3F),
        );
        if let Some(path_v) = v_table.get().get(constants_str::PATH_ALT_5) {
            match path_v {
                toml::Value::String(_) => {
                    match v_table.get().len() {
                        1 => (),
                        2 => super::validate_workspace_dep_default_features(v_table),
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
        super::validate_workspace_dep_version(v_table);
        super::validate_workspace_dep_default_features(v_table);
        match v_table.get().len() {
            2 => {}
            3 => super::validate_workspace_dep_features(v_table),
            _ => panic!("f1139378 {v_table:#?}"),
        }
    });
}
#[test]
fn external_workspace_dependencies_disable_default_features() {
    let workspace = super::workspace_table_from_cargo_toml();
    let dependencies = super::toml_val_as_table_ref(
        super::types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(constants_str::DEPENDENCIES)
                .expect("9ac9fb4c external_workspace_dependencies_disable_default_features invariant must hold"),
        ),
        super::types::StaticStr::from(constants_str::VALUE_5EECAACC),
    );
    let violations = dependencies
        .as_ref()
        .iter()
        .filter(|(_, dependency)| {
            dependency
                .as_table()
                .is_some_and(|table| table.contains_key(constants_str::VERSION_ALT_3))
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
        super::workspace_dep_disables_default_features(super::types::TomlValueRef::from(
            valid.get("dependency").expect("34136b6c workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold"),
        ))
        .get()
    );
    assert!(
        !super::workspace_dep_disables_default_features(super::types::TomlValueRef::from(
            missing.get("dependency").expect("e9b5ed95 workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold"),
        ))
        .get()
    );
    assert!(
        !super::workspace_dep_disables_default_features(super::types::TomlValueRef::from(
            enabled.get("dependency").expect("3e8046ef workspace_dependency_default_feature_policy_rejects_missing_and_true_values invariant must hold"),
        ))
        .get()
    );
}
#[test]
fn workspace_uses_one_async_runtime() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
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
                    .get(constants_str::VALUE_37A8EEC1)
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
            workspace.as_ref().get(constants_str::DEPENDENCIES).expect(
                "3e0ac397 workspace_dependency_catalog_has_no_unused_entries invariant must hold",
            ),
        ),
        super::types::StaticStr::from(constants_str::VALUE_5EB013E8),
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
                    .is_some_and(|table| table.contains_key(constants_str::VERSION_ALT_3))
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
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct TestOwnershipException {
        crate_name: &'static str,
        reason: &'static str,
    }
    let exceptions = [
        TestOwnershipException {
            crate_name: constants_str::VALUE_CA3132B2,
            reason: constants_str::VALUE_0936B9F6,
        },
        TestOwnershipException {
            crate_name: constants_str::CONFIG_LIB_MACROS,
            reason: constants_str::VALUE_266462B1,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_198286F1,
            reason: constants_str::VALUE_EE790765,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_BC0C50B5,
            reason: constants_str::VALUE_266462B1,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_B797AB3D,
            reason: constants_str::VALUE_CBD059B2,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_3BD49AF7,
            reason: constants_str::VALUE_1F27BD33,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_C354B535,
            reason: constants_str::VALUE_4F0D0D6A,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_B58CD11D,
            reason: constants_str::VALUE_32955237,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_49A3E4A5,
            reason: constants_str::VALUE_FCF0F5CE,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_11CDC13C,
            reason: constants_str::VALUE_50445A70,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_2A9F7F88,
            reason: constants_str::VALUE_2BAE5A74,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_62CE157E,
            reason: constants_str::VALUE_DE92495B,
        },
        TestOwnershipException {
            crate_name: constants_str::GENERATE_PG_TABLE,
            reason: constants_str::VALUE_5035064F,
        },
        TestOwnershipException {
            crate_name: constants_str::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
            reason: constants_str::VALUE_2953C66F,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_E644078E,
            reason: constants_str::VALUE_17A89871,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_D13E7908,
            reason: constants_str::VALUE_17A89871,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_174C657A,
            reason: constants_str::VALUE_17A89871,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_A2832C3A,
            reason: constants_str::VALUE_17A89871,
        },
        TestOwnershipException {
            crate_name: constants_str::CODE_STYLE_GENERATE_WHERE_FILTERS_MACRO_NAME,
            reason: constants_str::VALUE_FCB0537D,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_25EADB03,
            reason: constants_str::VALUE_25894D8E,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_39C24497,
            reason: constants_str::VALUE_5FE31A84,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_AEC33C7D,
            reason: constants_str::VALUE_04D1B7A1,
        },
        TestOwnershipException {
            crate_name: constants_str::VALUE_DF1A7C9C,
            reason: constants_str::VALUE_9BF87B94,
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
                        super::source_analysis::PublicLogicVisitor::default(),
                    )
                    .found
                    .get()
                });
                let has_owned_test = source_files.iter().any(|source_file| {
                    super::visit_syn_file(
                        super::types::SynFileRef::from(source_file.ast().as_ref()),
                        super::source_analysis::OwnedTestVisitor::default(),
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
fn source_modules_with_public_logic_own_unit_tests() {
    let reviewed_without_local_tests = std::collections::BTreeMap::from([
        (constants_str::VALUE_FF5D5E0E, constants_str::VALUE_405E3416),
        (constants_str::VALUE_DB7F37E1, constants_str::VALUE_F2207121),
        (constants_str::VALUE_823EE954, constants_str::VALUE_74AEB26B),
        (constants_str::VALUE_D11679FC, constants_str::VALUE_6161F31D),
        (constants_str::VALUE_7AEFC966, constants_str::VALUE_857F7C2F),
        (constants_str::VALUE_794839A7, constants_str::VALUE_DFCDB100),
        (constants_str::VALUE_7F7EAAAF, constants_str::VALUE_BED211BE),
        (constants_str::VALUE_642AA8AC, constants_str::VALUE_D7F0D3FB),
        (constants_str::VALUE_31BDEFD7, constants_str::VALUE_8E47C546),
        (constants_str::VALUE_95F11308, constants_str::VALUE_D7F0D3FB),
        (constants_str::VALUE_02C92481, constants_str::VALUE_FB0F2679),
        (constants_str::VALUE_C652C5A2, constants_str::VALUE_03E3C8DC),
        (constants_str::VALUE_BDEB5C57, constants_str::VALUE_99D169EE),
        (constants_str::VALUE_8F0CF86A, constants_str::VALUE_D3401592),
        (constants_str::VALUE_30B1AC8C, constants_str::VALUE_0B457512),
        (constants_str::VALUE_D6A2A64F, constants_str::VALUE_6705A1D1),
        (constants_str::VALUE_8CD81F6A, constants_str::VALUE_0CCB452D),
        (constants_str::VALUE_4B935405, constants_str::VALUE_E3AA090E),
        (constants_str::VALUE_F8BC20AB, constants_str::VALUE_16B4B741),
        (constants_str::VALUE_84D6426B, constants_str::VALUE_6BEEA909),
        (constants_str::VALUE_EC2A2742, constants_str::VALUE_86B4ECF0),
        (constants_str::VALUE_5288B694, constants_str::VALUE_6FD12145),
        (constants_str::VALUE_566A29FB, constants_str::VALUE_C4ABC7DA),
        (constants_str::VALUE_BB268B0B, constants_str::VALUE_EC45AD4A),
        (constants_str::VALUE_1ACC98BE, constants_str::VALUE_C441A0D8),
        (constants_str::VALUE_7DF10CC7, constants_str::VALUE_15D6492D),
        (constants_str::VALUE_1F61C5FC, constants_str::VALUE_D6049DD6),
        (constants_str::VALUE_A7D2D1E3, constants_str::VALUE_7D137CD7),
        (constants_str::VALUE_1BEBF98C, constants_str::VALUE_C979C05B),
        (constants_str::VALUE_87B73E51, constants_str::VALUE_7E9629EC),
        (constants_str::VALUE_426047D0, constants_str::VALUE_A4D4E469),
        (constants_str::VALUE_BC1068F8, constants_str::VALUE_FE0292DF),
        (constants_str::VALUE_3282DD39, constants_str::VALUE_30FDB118),
        (constants_str::VALUE_A57F952F, constants_str::VALUE_A3F70F9A),
        (constants_str::VALUE_516D6874, constants_str::VALUE_A1299ABB),
        (constants_str::VALUE_73F238C3, constants_str::VALUE_A3F70F9A),
        (constants_str::VALUE_E7C9496D, constants_str::VALUE_4F88C226),
        (constants_str::VALUE_744944F8, constants_str::VALUE_6F6BA65F),
        (constants_str::VALUE_886BA6BB, constants_str::VALUE_F84F38AE),
        (
            constants_str::SERVER_ADMIN_SRC_PASSWORD_RS,
            constants_str::VALUE_8269812F,
        ),
    ]);
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::new();
        let mut violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !super::is_test_source_path(super::types::PathRef::from(std::borrow::Borrow::<
                    std::path::Path,
                >::borrow(
                    source_file.path()
                )))
                .get()
            })
            .filter_map(|source_file| {
                let public_logic = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    super::source_analysis::PublicLogicVisitor::default(),
                )
                .found
                .get();
                let owns_test = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    super::source_analysis::OwnedTestVisitor::default(),
                )
                .found
                .get();
                let path = source_file.path().as_ref().display().to_string();
                let reviewed = reviewed_without_local_tests.iter().any(|(suffix, reason)| {
                    let matches = path.ends_with(*suffix) && !reason.is_empty();
                    if matches {
                        let _inserted = matched.insert((*suffix).to_owned());
                    }
                    matches
                });
                (public_logic && !owns_test && !reviewed).then_some(path)
            })
            .collect::<Vec<String>>();
        if matched.len() != reviewed_without_local_tests.len() {
            violations.push(format!(
                "stale public-logic test exceptions: matched={matched:#?}"
            ));
        }
        assert!(violations.is_empty(), "c73f7bd4 {violations:#?}");
    });
}
#[test]
fn workspace_lint_allows_have_inline_reasons() {
    let source = std::fs::read_to_string(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
        .expect("68dcaf75 workspace_lint_allows_have_inline_reasons invariant must hold");
    let violations = super::unjustified_workspace_lint_allows(super::types::SourceTextRef::from(
        source.as_str(),
    ));
    assert!(violations.is_empty(), "a94f0751 {violations:#?}");
}
#[test]
fn workspace_lint_allow_reason_policy_rejects_missing_and_empty_comments() {
    let violations = super::unjustified_workspace_lint_allows(super::types::SourceTextRef::from(
        constants_str::VALUE_05BB0EE4,
    ));
    assert_eq!(violations.len(), 2usize);
}
#[test]
fn env_and_env_example_have_same_keys() {
    let env_keys =
        super::env_keys_from_file(super::types::StaticStr::from(constants_str::SERVER_ENV));
    let example_keys = super::env_keys_from_file(super::types::StaticStr::from(
        constants_str::SERVER_DOT_ENV_EXAMPLE,
    ));
    let env_keys_set = super::str_set(super::types::SourceTextListRef::from(env_keys.as_slice()));
    let example_keys_set = super::str_set(super::types::SourceTextListRef::from(
        example_keys.as_slice(),
    ));
    let mut ers = super::collect_missing_key_ers(
        super::types::SourceTextListRef::from(env_keys.as_slice()),
        super::types::SourceTextRefHashSet::from(example_keys_set.as_ref()),
        super::types::StaticStr::from(constants_str::ENV),
        super::types::StaticStr::from(constants_str::ENV_EXAMPLE),
    );
    ers.extend(super::collect_missing_key_ers(
        super::types::SourceTextListRef::from(example_keys.as_slice()),
        super::types::SourceTextRefHashSet::from(env_keys_set.as_ref()),
        super::types::StaticStr::from(constants_str::ENV_EXAMPLE),
        super::types::StaticStr::from(constants_str::ENV),
    ));
    super::assert_joined_ers_empty_sorted(
        super::types::DiagnosticMsgsMutRef::from(&mut ers),
        super::types::StaticStr::from(constants_str::C8D2F1A3),
    );
}
#[test]
fn server_has_one_tracked_environment_example() {
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
fn workspace_crates_must_use_workspace_dependencies() {
    super::assert_cargo_toml_ers_empty(
        super::types::StaticStr::from(constants_str::VALUE_5F8A6D17),
        |path, parsed, ers| {
            super::collect_non_workspace_dep_ers(
                super::types::PathRef::from(path),
                super::types::TomlTableRef::from(parsed),
                super::types::DiagnosticMsgsMutRef::from(ers),
            );
        },
    );
}
#[test]
fn target_specific_dependencies_must_use_workspace_dependencies() {
    let invalid_manifest = constants_str::VALUE_DB030A59.parse::<toml::Table>().expect(
        "b49e27c1 target_specific_dependencies_must_use_workspace_dependencies invariant must hold",
    );
    let mut invalid_ers = Vec::new();
    super::collect_non_workspace_dep_ers(
        super::types::PathRef::from(std::path::Path::new(constants_str::VALUE_EAE77D23)),
        super::types::TomlTableRef::from(&invalid_manifest),
        super::types::DiagnosticMsgsMutRef::from(&mut invalid_ers),
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
    super::collect_non_workspace_dep_ers(
        super::types::PathRef::from(std::path::Path::new(constants_str::VALUE_EAE77D23)),
        super::types::TomlTableRef::from(&valid_manifest),
        super::types::DiagnosticMsgsMutRef::from(&mut valid_ers),
    );
    assert!(valid_ers.is_empty());
}
#[test]
fn workspace_dependencies_use_inline_table_style() {
    let regex =
        regex::Regex::new(constants_str::QUESTION_M_S_ASTERISK_A_ZA_Z0_9_PLUS_WORKSPACE_S_ASTERISK)
            .expect("ac15d6b9 workspace_dependencies_use_inline_table_style invariant must hold");
    let mut ers = Vec::new();
    super::for_each_crate_manifest_file(|path| {
        let v = super::cargo_toml_content(super::types::PathRef::from(path))
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
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(constants_str::D7A3C5B1),
        super::types::SourceTextRef::from(constants_str::DOTTED_WORKSPACE_DEPENDENCY_STYLE_FOUND),
    );
}
#[test]
fn workspace_members_exist_on_disk() {
    let workspace = super::workspace_table_from_cargo_toml();
    let members = super::workspace_members_as_strs(
        super::types::TomlTableRef::from(workspace.as_ref()),
        super::types::StaticStr::from(constants_str::VALUE_7F3A1C4E),
    );
    let mut ers = super::types::SourceTextList::from(
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
    super::assert_joined_ers_empty_sorted(
        super::types::DiagnosticMsgsMutRef::from(&mut ers),
        super::types::StaticStr::from(constants_str::A4E3B8D1),
    );
}
#[test]
fn workspace_crates_are_direct_children_of_workspace_root() {
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
fn workspace_members_sorted_alphabetically() {
    let workspace = super::workspace_table_from_cargo_toml();
    let members_vec = super::workspace_members_as_strs(
        super::types::TomlTableRef::from(workspace.as_ref()),
        super::types::StaticStr::from(constants_str::C1D4F7A2),
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
        super::types::StaticStr::from(constants_str::B7C2E5F8),
        super::types::SourceTextRef::from(constants_str::MEMBERS_NOT_SORTED),
    );
}

#[test]
fn workspace_packages_have_at_most_one_binary_target() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
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
