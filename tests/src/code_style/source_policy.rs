const REVIEWED_PUBLIC_FIELDS: &[ReviewedPublicFields] = &[
    ReviewedPublicFields {
        fields: &["file", "line", "column"],
        path_suffix: "location_lib/src/location.rs",
        reason: "location proc-macro output exposes occurrence coordinates as its public data contract",
        struct_name: "Occr",
    },
    ReviewedPublicFields {
        fields: &["secs", "nanos"],
        path_suffix: "location_lib/src/location.rs",
        reason: "serialized duration representation is a public wire-format helper",
        struct_name: "StdTimeDuration",
    },
    ReviewedPublicFields {
        fields: &["identifier", "type0", "vis"],
        path_suffix: "macros_helpers/src/syn_field.rs",
        reason: "macro generators consume the parsed field descriptor across crate boundaries",
        struct_name: "SynField",
    },
    ReviewedPublicFields {
        fields: &[
            "bulk_item_budget",
            "config",
            "idempotency_response_budget",
            "pg_pool",
            "project_git_info",
        ],
        path_suffix: "server_app_state/src/lib.rs",
        reason: "Axum state consumers in service crates require direct access to shared immutable state",
        struct_name: "ServerAppState",
    },
    ReviewedPublicFields {
        fields: &["greater_than", "create", "variant"],
        path_suffix: "pg_crud/pg_crud_common/src/lib.rs",
        reason: "public generator test descriptor is constructed by downstream generated test crates",
        struct_name: "PgTypeGreaterThanTest",
    },
    ReviewedPublicFields {
        fields: &["create", "variant", "len_greater_than"],
        path_suffix: "pg_crud/pg_crud_common/src/lib.rs",
        reason: "public generator test descriptor is constructed by downstream generated test crates",
        struct_name: "PgTypeLenGreaterThanTest",
    },
    ReviewedPublicFields {
        fields: &["column", "order"],
        path_suffix: "pg_crud/pg_crud_common/src/lib.rs",
        reason: "generated query code constructs this public typed ordering contract",
        struct_name: "OrderBy",
    },
    ReviewedPublicFields {
        fields: &["v"],
        path_suffix: "pg_crud/pg_crud_common/src/lib.rs",
        reason: "generated filter code constructs this public generic value contract",
        struct_name: "V",
    },
    ReviewedPublicFields {
        fields: &[
            "cors_allow_origin",
            "content_security_policy",
            "database_url",
            "admin_jwt_secret",
            "admin_token_audience",
            "admin_token_issuer",
            "trusted_proxy_ranges_text",
            "admin_access_token_ttl_seconds",
            "admin_password_hash_concurrency",
            "admin_refresh_token_ttl_seconds",
            "admin_session_limit",
            "admin_sign_in_rate_limit",
            "pg_pool_acquire_timeout_seconds",
            "pg_pool_idle_timeout_seconds",
            "pg_pool_max_lifetime_seconds",
            "maximum_size_of_http_body_in_bytes",
            "service_socket_address",
            "pg_pool_max_connections",
            "pg_pool_min_connections",
            "timezone",
            "src_place_type",
            "tracing_level",
            "tracing_format",
            "enable_api_git_commit_check",
            "admin_cookie_secure",
            "admin_swagger_enabled",
            "http_gzip_enabled",
        ],
        path_suffix: "server_config/src/lib.rs",
        reason: "service entry points consume the validated immutable workspace configuration contract",
        struct_name: "Config",
    },
    ReviewedPublicFields {
        fields: &["id", "user_id", "role_id", "created_at"],
        path_suffix: "server_admin/src/generated_tables.rs",
        reason: "generated database row model is a public serialization and query contract",
        struct_name: "AdminUserRoles",
    },
    ReviewedPublicFields {
        fields: &["id", "role_id", "permission_id", "created_at"],
        path_suffix: "server_admin/src/generated_tables.rs",
        reason: "generated database row model is a public serialization and query contract",
        struct_name: "AdminRolePermissions",
    },
    ReviewedPublicFields {
        fields: &["id", "name", "is_system", "created_at", "updated_at"],
        path_suffix: "server_admin/src/generated_tables.rs",
        reason: "generated database row model is a public serialization and query contract",
        struct_name: "AdminRoles",
    },
    ReviewedPublicFields {
        fields: &["id", "name", "created_at"],
        path_suffix: "server_admin/src/generated_tables.rs",
        reason: "generated database row model is a public serialization and query contract",
        struct_name: "AdminPermissions",
    },
    ReviewedPublicFields {
        fields: &[
            "id",
            "site_name",
            "tab_title",
            "main_logo",
            "primary_color",
            "default_admin_route",
            "organization_name",
            "organization_contacts",
            "support_url",
            "updated_at",
        ],
        path_suffix: "server_admin/src/generated_tables.rs",
        reason: "generated database row model is a public serialization and query contract",
        struct_name: "AdminSystemSettings",
    },
];
struct ReviewedPublicFields {
    fields: &'static [&'static str],
    path_suffix: &'static str,
    reason: &'static str,
    struct_name: &'static str,
}
#[test]
fn all_files_are_english_only() {
    let mut ers = super::snapshot::with_codebase_snapshot(|snapshot| {
        rayon::iter::ParallelIterator::reduce(
            rayon::iter::ParallelIterator::map(
                rayon::iter::IntoParallelRefIterator::par_iter(snapshot.project_source_files()),
                |source_file| {
                    super::collect_non_english_symbol_ers(
                        super::types::StdPathRef::from(source_file.path().as_ref()),
                        super::types::SourceTextRef::from(source_file.content().as_ref()),
                    )
                    .into_iter()
                    .collect::<Vec<String>>()
                },
            ),
            Vec::new,
            |mut accumulator, mut item| {
                accumulator.append(&mut item);
                accumulator
            },
        )
    });
    ers.sort();
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(str_constants::VALUE_8DB37A2F),
        super::types::SourceTextRef::from(str_constants::NON_ENGLISH_SYMBOLS),
    );
}
#[test]
fn check_expect_contains_only_unique_uuid_v4() {
    super::check_expect_or_panic_contains_only_unique_uuid_v4(super::ExpectOrPanic::Expect);
}
#[test]
fn check_panic_contains_only_unique_uuid_v4() {
    super::check_expect_or_panic_contains_only_unique_uuid_v4(super::ExpectOrPanic::Panic);
}
#[test]
fn check_rs_files_contains_only_unique_uuid_v4() {
    let regex = regex::Regex::new(str_constants::B_0_9A_FA_F_8_0_9A_FA_F_4_4).expect("e098a1ff");
    let mut seen = std::collections::HashSet::new();
    super::for_each_rs_file_content(|_, v| {
        regex.find_iter(v).for_each(|element_714b3d9c| {
            let uuid = uuid::Uuid::parse_str(element_714b3d9c.as_str()).expect("c9711efd");
            assert!(uuid.get_version_num() == 4, "49b49b21");
            assert!(seen.insert(uuid), "4cf9d239");
        });
    });
}
#[test]
fn no_dbg_macro_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::F1C7A4E3),
        super::types::SourceTextRef::from(str_constants::DBG_FOUND),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::DbgVisitor {
                    found: super::types::AnalyzerBool::default(),
                },
            );
            if visitor.found.get() {
                ers.push(format!("{}: contains dbg!()", path.display()));
            }
        },
    );
}
#[test]
fn no_for_loops_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::F4C2A9E1),
        super::types::SourceTextRef::from(
            str_constants::FOR_LOOPS_FOUND_USE_ITERATOR_METHODS_SUCH_AS_MAP_FILTER_FOLD_TRY,
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::ForLoopVisitor {
                    found_count: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from(
                    str_constants::CONTAINS_FOR_LOOP_USE_ITERATOR_METHODS_INSTEAD,
                ),
                visitor.found_count,
            );
        },
    );
}

#[test]
fn map_err_does_not_discard_source_with_wildcard() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|source_file| {
                let mut visitor = super::SourceDroppingMapErrVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                (visitor.found_count.get() != 0usize).then(|| {
                    format!(
                        "{} discards a map_err source with a wildcard",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn numeric_conversions_do_not_use_as_casts() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|source_file| {
                let mut visitor = super::NumericAsCastVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                (visitor.found_count.get() != 0usize).then(|| {
                    format!(
                        "{} contains {} numeric as cast(s)",
                        source_file.path().as_ref().display(),
                        visitor.found_count.get()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn runtime_struct_fields_do_not_expose_untyped_json_values() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = super::SerdeJsonValueFieldVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.violations.into_iter().map(|item| {
                    format!(
                        "{} exposes serde_json::Value in {item}",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn new_runtime_structs_keep_fields_private() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::<(String, String)>::new();
        let mut violations = Vec::new();
        snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !super::is_test_crate_source_path(super::types::StdPathRef::from(
                    source_file.path().as_ref(),
                ))
                .get()
            })
            .for_each(|source_file| {
                let mut visitor = super::PublicStructFieldVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.violations.into_iter().for_each(|item| {
                    let path = source_file.path().as_ref();
                    let reviewed_match = REVIEWED_PUBLIC_FIELDS.iter().find(|reviewed| {
                        path.ends_with(reviewed.path_suffix)
                            && reviewed
                                .fields
                                .iter()
                                .any(|field| item == format!("{}::{field}", reviewed.struct_name))
                    });
                    if let Some(reviewed) = reviewed_match {
                        let _inserted =
                            matched.insert((reviewed.path_suffix.to_owned(), item.clone()));
                    } else {
                        violations.push(format!(
                            "{} exposes an unreviewed public field in {item}",
                            path.display()
                        ));
                    }
                });
            });
        let expected = REVIEWED_PUBLIC_FIELDS
            .iter()
            .flat_map(|reviewed| {
                reviewed.fields.iter().map(|field| {
                    (
                        reviewed.path_suffix.to_owned(),
                        format!("{}::{field}", reviewed.struct_name),
                    )
                })
            })
            .collect::<std::collections::BTreeSet<(String, String)>>();
        if matched != expected {
            violations.push(format!(
                "public field exception inventory is stale; expected={expected:#?}, matched={matched:#?}"
            ));
        }
        REVIEWED_PUBLIC_FIELDS
            .iter()
            .filter(|reviewed| reviewed.reason.trim().is_empty())
            .for_each(|reviewed| {
                violations.push(format!(
                    "{}::{} public field exception has no reason",
                    reviewed.path_suffix, reviewed.struct_name
                ));
            });
        assert!(violations.is_empty(), "{violations:#?}");
    });
}
#[test]
fn spawned_tasks_must_retain_an_owner() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::VALUE_5D0D5BF0),
        super::types::SourceTextRef::from(str_constants::SPAWNED_TASK_HANDLES_ARE_DISCARDED),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::LostSpawnVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn direct_environment_and_filesystem_access_stays_at_owned_boundaries() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::VALUE_321360D4),
        super::types::SourceTextRef::from(str_constants::DIRECT_ENVIRONMENT_OR_FILESYSTEM_ACCESS_EXISTS_OUTSIDE_APPROVED_CONFIGURATION_TOOLING_TEST_AND),
        |path, ast, ers| {
            if super::is_test_crate_source_path(super::types::StdPathRef::from(path)).get()
                || super::is_direct_fs_owner_source_path(super::types::StdPathRef::from(path)).get()
                || path.ends_with(str_constants::SERVER_RUNTIME_SRC_BOUNDED_READ_RS)
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::DirectPathCallVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(visitor.calls.into_iter().filter_map(|call| {
                (call.starts_with(str_constants::STD_PATH_ENV_PATH)
                    || call.starts_with(str_constants::STD_PATH_FS_PATH)
                    || call.starts_with(str_constants::TOKIO_PATH_FS_PATH))
                .then(|| format!("{}: direct `{call}`", path.display()))
            }));
        },
    );
}
#[test]
fn direct_filesystem_owner_inventory_is_exact_justified_and_current() {
    assert_eq!(
        str_constants::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES.len(),
        str_constants::CODE_STYLE_DIRECT_FS_OWNER_REASONS.len(),
        "6e1a9c30"
    );
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("4d82f1b7");
    let missing_or_unjustified = str_constants::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
        .iter()
        .zip(str_constants::CODE_STYLE_DIRECT_FS_OWNER_REASONS)
        .filter(|(suffix, reason)| {
            reason.trim().is_empty()
                || !workspace_root
                    .join(suffix.trim_start_matches('/'))
                    .is_file()
        })
        .collect::<Vec<(&&str, &str)>>();
    assert!(
        missing_or_unjustified.is_empty(),
        "c70b25ea {missing_or_unjustified:?}"
    );
    assert!(
        super::is_direct_fs_owner_source_path(super::types::StdPathRef::from(
            std::path::Path::new("../workspace_scaffold/src/main.rs")
        ))
        .get(),
        "b39e07d4"
    );
    assert!(
        !super::is_direct_fs_owner_source_path(super::types::StdPathRef::from(
            std::path::Path::new("../workspace_scaffold/src/unrelated.rs")
        ))
        .get(),
        "f1428b6c"
    );
}
#[test]
fn retained_path_exception_inventories_are_exact_justified_unique_and_current() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("28d4f1a9");
    let validate = |suffixes: &[&str], reasons: &[&str]| {
        assert_eq!(suffixes.len(), reasons.len(), "5c71e8b3");
        assert_eq!(
            suffixes
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            suffixes.len(),
            "ac308f64"
        );
        let invalid = suffixes
            .iter()
            .zip(reasons)
            .filter(|(suffix, reason)| {
                let relative = suffix.trim_start_matches("../").trim_start_matches('/');
                reason.trim().is_empty() || !workspace_root.join(relative).is_file()
            })
            .collect::<Vec<(&&str, &&str)>>();
        assert!(invalid.is_empty(), "e91b2d7c {invalid:?}");
    };
    validate(
        str_constants::CODE_STYLE_RUNTIME_TEST_HELPER_SUFFIXES.as_slice(),
        str_constants::CODE_STYLE_RUNTIME_TEST_HELPER_REASONS.as_slice(),
    );
    validate(
        str_constants::CODE_STYLE_RUNTIME_ARC_OWNER_SUFFIXES.as_slice(),
        str_constants::CODE_STYLE_RUNTIME_ARC_OWNER_REASONS.as_slice(),
    );
    validate(
        str_constants::CODE_STYLE_FACADE_REEXPORT_SUFFIXES.as_slice(),
        str_constants::CODE_STYLE_FACADE_REEXPORT_REASONS.as_slice(),
    );
    validate(
        str_constants::CODE_STYLE_LEPTOS_PRELUDE_SUFFIXES.as_slice(),
        str_constants::CODE_STYLE_LEPTOS_PRELUDE_REASONS.as_slice(),
    );
    validate(
        str_constants::CODE_STYLE_SINGLE_SOURCE_OWNER_SUFFIXES.as_slice(),
        str_constants::CODE_STYLE_SINGLE_SOURCE_OWNER_REASONS.as_slice(),
    );
    assert!(
        !str_constants::CODE_STYLE_LOCATION_TEST_REASON.is_empty()
            && workspace_root
                .join(str_constants::CODE_STYLE_LOCATION_TEST_SRC.trim_start_matches("../"),)
                .is_dir(),
        "d47c0e26"
    );
    assert!(
        !str_constants::CODE_STYLE_PG_CRUD_COMMON_BENCHES_REASON.is_empty()
            && workspace_root
                .join(str_constants::CODE_STYLE_PG_CRUD_COMMON_BENCHES.trim_start_matches("../"),)
                .is_dir(),
        "631fb5d8"
    );
    assert_eq!(
        str_constants::CODE_STYLE_TEST_CRATE_NAMES.len(),
        str_constants::CODE_STYLE_TEST_CRATE_REASONS.len(),
        "fa64b9c1"
    );
    let workspace_crates = super::workspace_crate_names();
    let invalid_test_crates = str_constants::CODE_STYLE_TEST_CRATE_NAMES
        .iter()
        .zip(str_constants::CODE_STYLE_TEST_CRATE_REASONS)
        .filter(|(name, reason)| reason.trim().is_empty() || !workspace_crates.contains(**name))
        .collect::<Vec<(&&str, &str)>>();
    assert!(
        invalid_test_crates.is_empty(),
        "b208d75e {invalid_test_crates:?}"
    );
}
#[test]
fn runtime_data_reads_are_bounded() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::VALUE_37B593CE),
        super::types::SourceTextRef::from(
            str_constants::RUNTIME_CODE_PERFORMS_AN_UNBOUNDED_FILE_OR_HTTP_RESPONSE_READ,
        ),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if super::is_test_crate_source_path(super::types::StdPathRef::from(path)).get()
                || str_constants::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES
                    .iter()
                    .any(|suffix| path_text.ends_with(suffix))
                || path_text.ends_with(str_constants::SERVER_RUNTIME_SRC_BOUNDED_READ_RS)
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::UnboundedReadVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .calls
                    .into_iter()
                    .map(|call| format!("{}: unbounded `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn environment_initializer_is_in_bounded_read_policy_scope() {
    assert!(
        !str_constants::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES
            .iter()
            .any(|suffix| suffix.contains(str_constants::INITIALIZE_ENVIRONMENT_FILES)),
        "920fde35"
    );
}
#[test]
fn workspace_scaffold_is_in_bounded_read_policy_scope() {
    assert!(
        !str_constants::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES
            .contains(&str_constants::CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX),
        "54b718ca"
    );
}
#[test]
fn bounded_read_policy_has_no_whole_file_owner_exceptions() {
    assert!(
        str_constants::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES.is_empty(),
        "b71f043c"
    );
}
#[test]
fn raw_runtime_sql_identifier_inventory_matches_reviewed_baseline() {
    let mut observed = std::collections::BTreeMap::<String, usize>::new();
    super::for_each_rs_file_content(|path, content| {
        let path_text = path.to_string_lossy();
        if super::is_test_crate_source_path(super::types::StdPathRef::from(path)).get()
            || path_text.ends_with(str_constants::CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX)
            || path_text.ends_with(str_constants::PG_CRUD_PG_CRUD_COMMON_SRC_SQL_IDENTIFIER_RS)
            || path_text.ends_with(str_constants::STR_CONSTANTS_SRC_LIB_RS)
        {
            return;
        }
        let count = [
            str_constants::FROM,
            str_constants::INTO,
            str_constants::UPDATE,
        ]
        .into_iter()
        .map(|pattern| content.matches(pattern).count())
        .sum::<usize>();
        if count != 0usize {
            let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("19512c63");
            let relative = path
                .strip_prefix(workspace_root)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            let _previous = observed.insert(relative, count);
        }
    });
    let expected = std::collections::BTreeMap::new();
    assert_eq!(observed, expected, "raw SQL identifier baseline changed");
}
#[test]
fn production_pg_error_classification_is_centralized() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                let path = source_file.path().as_ref().to_string_lossy();
                !super::is_test_crate_source_path(super::types::StdPathRef::from(
                    source_file.path().as_ref(),
                ))
                .get()
                    && !path.ends_with(str_constants::PG_CRUD_COMMON_SRC_PG_ERROR_RS)
                    && !path.ends_with(str_constants::STR_CONSTANTS_SRC_LIB_RS)
                    && (source_file
                        .content()
                        .as_ref()
                        .contains(str_constants::IS_UNIQUE_VIOLATION_CALL)
                        || source_file
                            .content()
                            .as_ref()
                            .contains(str_constants::PG_SQLSTATE_PREFIX))
            })
            .map(|source_file| {
                format!(
                    "{} classifies PostgreSQL errors outside pg_crud_common",
                    source_file.path().as_ref().display()
                )
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}
#[test]
fn direct_process_command_creation_stays_in_shared_tooling() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::F170AA14),
        super::types::SourceTextRef::from(str_constants::DIRECT_COMMAND_PATH_NEW_USAGE_EXISTS_OUTSIDE_MACROS_HELPERS_PATH_TOOL_COMMAND),
        |path, ast, ers| {
            if path.ends_with(str_constants::MACROS_HELPERS_SRC_TOOL_COMMAND_RS) {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::DirectPathCallVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .calls
                    .into_iter()
                    .filter(|call| call == str_constants::STD_PATH_PROCESS_PATH_COMMAND_PATH_NEW)
                    .map(|call| format!("{}: direct `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn abort_and_transmute_calls_match_reviewed_baseline() {
    let mut observed_abort_paths = Vec::new();
    let mut ers = Vec::new();
    super::for_each_rs_syn_file(|path, ast| {
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(ast),
            super::DirectPathCallVisitor {
                calls: super::types::DiagnosticMsgs::default(),
            },
        );
        visitor.calls.into_iter().for_each(|call| {
            if call == str_constants::STD_PATH_PROCESS_PATH_ABORT {
                observed_abort_paths.push(path.to_string_lossy().to_string());
            }
            if call.ends_with(str_constants::PATH_TRANSMUTE) {
                ers.push(format!("{}: forbidden `{call}`", path.display()));
            }
        });
    });
    observed_abort_paths.sort();
    if !observed_abort_paths.is_empty() {
        ers.push(format!(
            "process abort calls are forbidden; observed={observed_abort_paths:?}"
        ));
    }
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(str_constants::F87F82B6),
        super::types::SourceTextRef::from(str_constants::ABORT_TRANSMUTE_POLICY_VIOLATIONS),
    );
}
#[test]
fn unit_tests_use_deterministic_time_and_randomness_patterns() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::VALUE_821D4A76),
        super::types::SourceTextRef::from(str_constants::UNIT_TESTS_USE_NONDETERMINISTIC_TIME_SLEEP_OR_RANDOMNESS_WITHOUT_A_REVIEWED_OWNER),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::TestNondeterminismVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                    test_depth: super::types::AnalyzerCount::default(),
                },
            );
            visitor
                .calls
                .into_iter()
                .for_each(|call| ers.push(format!("{}: nondeterministic `{call}`", path.display())));
        },
    );
}
#[test]
fn unit_test_nondeterminism_visitor_rejects_sleep_and_random_uuid() {
    let ast = syn::parse_file(
        "
#[test]
fn nondeterministic_test() {
    tokio::time::sleep(std::time::Duration::from_secs(1));
    uuid::Uuid::new_v4();
}
",
    )
    .expect("9354f086");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::TestNondeterminismVisitor {
            calls: super::types::DiagnosticMsgs::default(),
            test_depth: super::types::AnalyzerCount::default(),
        },
    );
    assert_eq!(
        visitor.calls.as_slice(),
        [
            str_constants::TOKIO_PATH_TIME_PATH_SLEEP,
            str_constants::UUID_PATH_UUID_PATH_NEW_V4,
        ],
        "fa8d2bb1"
    );
}
#[test]
fn no_todo_or_unimplemented_macro_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::C4E9A2D7),
        super::types::SourceTextRef::from(str_constants::TODO_UNIMPLEMENTED_FOUND),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::TodoUnimplVisitor {
                    todo_found: super::types::AnalyzerCount::default(),
                    unimplemented_found: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from(str_constants::CONTAINS_TODO),
                visitor.todo_found,
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from(str_constants::CONTAINS_UNIMPLEMENTED),
                visitor.unimplemented_found,
            );
        },
    );
}
#[test]
fn no_macro_rules_in_source_code() {
    let macro_name = str_constants::MACRO_RULES;
    let forbidden = format!("{macro_name}!");
    let mut ers = Vec::new();
    super::for_each_rs_file_content(|path, v| {
        if v.contains(&forbidden) {
            ers.push(format!(
                "{}: contains {forbidden}; use a workspace proc-macro crate instead",
                path.display()
            ));
        }
    });
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(str_constants::B6E2A9F4),
        super::types::SourceTextRef::from(
            str_constants::MACRO_RULES_FOUND_USE_WORKSPACE_PROC_MACRO_CRATES_INSTEAD,
        ),
    );
}
#[test]
fn no_include_asset_macros_outside_allowlist() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::A6D4F2C9),
        super::types::SourceTextRef::from(str_constants::INCLUDE_STR_OR_INCLUDE_BYTES_FOUND_OUTSIDE_EXPLICIT_GENERATED_TEST_FIXTURE_ALLOWLIST),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::IncludeAssetMacroVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn no_non_public_use_imports_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::B4E7C2A9),
        super::types::SourceTextRef::from(str_constants::USE_IMPORTS_FOUND_OUTSIDE_EXPLICIT_FACADE_RE_EXPORT_FILES_PREFER_EXPLICIT_PATHS),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            let allows_leptos_prelude_import =
                str_constants::CODE_STYLE_LEPTOS_PRELUDE_SUFFIXES
                    .iter()
                    .any(|suffix| path_text.ends_with(suffix));
            let allows_public_reexports = str_constants::CODE_STYLE_FACADE_REEXPORT_SUFFIXES
                .iter()
                .any(|suffix| path_text.ends_with(suffix));
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::UseImportVisitor {
                    allow_leptos_prelude_import: super::types::AnalyzerBool::from(
                        allows_leptos_prelude_import,
                    ),
                    found_non_public_use_import: super::types::AnalyzerBool::default(),
                    found_use_rename: super::types::AnalyzerBool::default(),
                    public_use_roots: super::types::SourceTextList::default(),
                },
            );
            if visitor.found_non_public_use_import.get() {
                ers.push(format!(
                    "{}: found non-public use import; use the explicit path at the usage site",
                    path.display()
                ));
            }
            if !allows_public_reexports {
                ers.extend(visitor.public_use_roots.iter().map(|public_use_root| {
                        format!(
                        "{}: found public use import rooted at `{public_use_root}`; use the explicit path at the usage site",
                        path.display()
                        )
                    }));
            }
            if visitor.found_use_rename.get() {
                ers.push(format!(
                        "{}: found use rename with `as`; use the original item name or rename the item at its definition",
                        path.display()
                    ));
            }
        },
    );
}
#[test]
fn use_import_policy_narrows_facade_and_leptos_exceptions() {
    let ast = syn::parse_file(
        "use leptos::prelude::{ElementChild};\nuse std::fmt::Debug;\npub use facade::Item;",
    )
    .expect("7b9e6f31");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::UseImportVisitor {
            allow_leptos_prelude_import: super::types::AnalyzerBool::from(true),
            found_non_public_use_import: super::types::AnalyzerBool::default(),
            found_use_rename: super::types::AnalyzerBool::default(),
            public_use_roots: super::types::SourceTextList::default(),
        },
    );
    assert!(visitor.found_non_public_use_import.get(), "ac09626a");
    assert!(!visitor.found_use_rename.get(), "c2bff14e");
    assert_eq!(visitor.public_use_roots.len(), 1usize, "3f4798c8");

    let leptos_ast = syn::parse_file("use leptos::prelude::{ElementChild};").expect("56f86b52");
    let leptos_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&leptos_ast),
        super::UseImportVisitor {
            allow_leptos_prelude_import: super::types::AnalyzerBool::from(true),
            found_non_public_use_import: super::types::AnalyzerBool::default(),
            found_use_rename: super::types::AnalyzerBool::default(),
            public_use_roots: super::types::SourceTextList::default(),
        },
    );
    assert!(
        !leptos_visitor.found_non_public_use_import.get(),
        "5969a9a3"
    );
}
#[test]
fn no_type_aliases_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::C6E4F7A1),
        super::types::SourceTextRef::from(
            str_constants::TYPE_ALIASES_FOUND_USE_EXPLICIT_TYPES_AT_USAGE_SITES,
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::TypeAliasVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn no_simple_constant_aliases_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::A51F0D3B),
        super::types::SourceTextRef::from(
            str_constants::SIMPLE_CONSTANT_ALIASES_FOUND_USE_THE_SOURCE_CONSTANT_DIRECTLY,
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::ConstantAliasVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn tuple_newtypes_derive_from_inner_instead_of_implementing_passthrough_from() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from("ea2406af"),
        super::types::SourceTextRef::from(
            "manual passthrough From implementations found; derive newtype::FromInner instead",
        ),
        |path, ast, ers| {
            let is_required_foundation_impl = [
                (
                    std::path::Path::new("newtype/src/lib.rs"),
                    "a proc-macro crate cannot invoke its own derive macro",
                ),
                (
                    std::path::Path::new("str_constants_macros/src/lib.rs"),
                    "newtype depends transitively on str_constants_macros",
                ),
                (
                    std::path::Path::new("workspace_macro_helpers/src/lib.rs"),
                    "newtype depends directly on workspace_macro_helpers",
                ),
            ]
            .iter()
            .any(|(suffix, reason)| !reason.is_empty() && path.ends_with(suffix));
            if is_required_foundation_impl {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::PassthroughFromVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    inner_types: std::collections::BTreeMap::new(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn tuple_newtypes_derive_display_instead_of_implementing_forwarding_display() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from("6e0cc8df"),
        super::types::SourceTextRef::from(
            "manual forwarding Display implementations found; derive newtype::Display instead",
        ),
        |path, ast, ers| {
            let is_required_foundation_impl = [
                (
                    std::path::Path::new("newtype/src/lib.rs"),
                    "a proc-macro crate cannot invoke its own derive macro",
                ),
                (
                    std::path::Path::new("workspace_macro_helpers/src/lib.rs"),
                    "newtype depends directly on workspace_macro_helpers",
                ),
            ]
            .iter()
            .any(|(suffix, reason)| !reason.is_empty() && path.ends_with(suffix));
            if is_required_foundation_impl {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::ForwardingDisplayVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn tuple_newtypes_derive_deref_inner_instead_of_implementing_forwarding_deref() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from("269004ea"),
        super::types::SourceTextRef::from(
            "manual forwarding Deref implementations found; derive newtype::DerefInner instead",
        ),
        |path, ast, ers| {
            let required_foundation_impl = (
                std::path::Path::new("workspace_macro_helpers/src/lib.rs"),
                "newtype depends directly on workspace_macro_helpers",
            );
            if !required_foundation_impl.1.is_empty() && path.ends_with(required_foundation_impl.0)
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::ForwardingDerefVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    inner_types: std::collections::BTreeMap::new(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn no_duplicated_string_literals_in_non_policy_test_code() {
    let mut literal_locations_by_value = std::collections::BTreeMap::<String, Vec<String>>::new();
    super::for_each_rs_syn_file(|path, ast| {
        let path_text = path.display().to_string();
        if !super::is_non_policy_test_source_path(super::types::StdPathRef::from(path)).get() {
            return;
        }
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(ast),
            super::TestStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            },
        );
        visitor
            .values
            .into_iter()
            .filter(|literal_value| literal_value.len() > 3)
            .for_each(|literal_value| {
                literal_locations_by_value
                    .entry(literal_value)
                    .or_default()
                    .push(path_text.clone());
            });
    });
    let ers = literal_locations_by_value
        .into_iter()
        .filter(|(_, locations)| locations.len() > 1)
        .map(|(literal_value, locations)| {
            format!("duplicated string literal {literal_value:?} in {locations:?}")
        })
        .collect::<Vec<String>>();
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(str_constants::DE729A31),
        super::types::SourceTextRef::from(
            str_constants::DUPLICATED_STRING_LITERALS_FOUND_IN_NON_POLICY_TEST_CODE,
        ),
    );
}
#[test]
fn ordinary_test_fixture_is_in_duplicate_string_policy_scope() {
    assert!(
        super::is_non_policy_test_source_path(super::types::StdPathRef::from(
            std::path::Path::new(str_constants::CODE_STYLE_DOMAIN_FIXTURE_PATH)
        ))
        .get(),
        "f2ec448d"
    );
    assert!(
        !super::is_non_policy_test_source_path(super::types::StdPathRef::from(
            std::path::Path::new(str_constants::TESTS_SRC_CODE_STYLE)
        ))
        .get(),
        "8df61a91"
    );
}
#[test]
fn long_production_string_literals_are_reused() {
    let mut literal_locations_by_crate_and_value =
        std::collections::BTreeMap::<(String, String), Vec<String>>::new();
    super::for_each_rs_syn_file(|path, ast| {
        let path_text = path.display().to_string();
        if super::is_test_crate_source_path(super::types::StdPathRef::from(path)).get()
            || super::is_code_style_meta_harness_source_path(super::types::StdPathRef::from(path))
                .get()
        {
            return;
        }
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(ast),
            super::ProductionStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            },
        );
        visitor
            .values
            .into_iter()
            .filter(|literal_value| literal_value.len() >= 16usize)
            .for_each(|literal_value| {
                let crate_path = path_text
                    .split_once(str_constants::SRC)
                    .map_or(path_text.as_str(), |(prefix, _)| prefix)
                    .to_owned();
                literal_locations_by_crate_and_value
                    .entry((crate_path, literal_value))
                    .or_default()
                    .push(path_text.clone());
            });
    });
    let ers = literal_locations_by_crate_and_value
        .into_iter()
        .filter(|(_, locations)| locations.len() > 1usize)
        .map(|((_crate_path, literal_value), locations)| {
            format!("duplicated long production string literal {literal_value:?} in {locations:?}")
        })
        .collect::<Vec<String>>();
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(str_constants::VALUE_9D1C7E4A),
        super::types::SourceTextRef::from(
            str_constants::LONG_PRODUCTION_STRING_LITERALS_MUST_BE_DEFINED_ONCE_AND_REUSED,
        ),
    );
}
#[test]
fn string_constants_are_declared_only_in_str_constants() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::VALUE_6F2C8A91),
        super::types::SourceTextRef::from(
            str_constants::STRING_CONSTANTS_FOUND_OUTSIDE_STR_CONSTANTS,
        ),
        |path, ast, ers| {
            if path.ends_with(str_constants::STR_CONSTANTS_SRC_LIB_RS) {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::StringConstantVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}

#[test]
fn server_admin_string_constants_reuse_macro_fragments() {
    let source =
        std::fs::read_to_string(str_constants::STR_CONSTANTS_SRC_LIB_RS).expect("4629edbb");
    assert!(!source.contains("pub const SERVER_ADMIN_"));
}

#[test]
fn admin_handlers_do_not_own_admin_sql() {
    let handlers = std::fs::read_to_string(str_constants::SERVER_ADMIN_SRC_AUTH_HANDLERS_RS)
        .expect("353df4df");
    assert!(
        !handlers.contains(str_constants::SERVER_ADMIN_CONSTANT_PREFIX)
            && !handlers.contains(str_constants::SQLX_QUERY_CALL)
    );
}

#[test]
fn str_constants_does_not_own_typed_domain_values() {
    let source =
        std::fs::read_to_string(str_constants::STR_CONSTANTS_SRC_LIB_RS).expect("3caa56a9");
    let ers = [
        concat!("ADMIN_API_", "PATHS_"),
        concat!("ADMIN_", "OPERATION_"),
        concat!("ADMIN_PAGE_", "PATHS_"),
        concat!("ADMIN_PERMISSION_", "VALUES_"),
    ]
    .into_iter()
    .filter(|prefix| source.contains(prefix))
    .map(str::to_owned)
    .collect::<Vec<_>>();
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(str_constants::VALUE_6B7E02A4),
        super::types::SourceTextRef::from(
            str_constants::DOMAIN_VALUES_MUST_BE_DECLARED_BY_THEIR_OWNING_TYPED_API,
        ),
    );
}
#[test]
fn string_constant_visitor_allows_only_reviewed_syntax_boundaries() {
    let ast = syn::parse_file(str_constants::CODE_STYLE_STRING_GUARD_ALLOWED_SYNTAX_FIXTURE)
        .expect("87c9a142");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::StringConstantVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert!(visitor.ers.is_empty());
}
#[test]
fn string_constant_visitor_detects_expression_and_nested_macro_literals() {
    let ast = syn::parse_file(str_constants::CODE_STYLE_STRING_GUARD_DETECTION_FIXTURE)
        .expect("bc91574f");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::StringConstantVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 2usize);
}
#[test]
fn no_unwrap_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::E8B3A6D2),
        super::types::SourceTextRef::from(str_constants::UNWRAP_FOUND),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::UnwrapVisitor {
                    found_count: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from(str_constants::UNWRAP_CALL_ALT),
                visitor.found_count,
            );
        },
    );
}
