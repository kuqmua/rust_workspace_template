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
        super::types::StaticStr("8db37a2f"),
        super::types::SourceTextRef::from("non-english symbols:"),
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
    let regex = regex::Regex::new(
        r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b",
    )
    .expect("e098a1ff");
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
        super::types::StaticStr("f1c7a4e3"),
        super::types::SourceTextRef::from("dbg!() found:"),
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
        super::types::StaticStr("f4c2a9e1"),
        super::types::SourceTextRef::from(
            "for loops found; use iterator methods such as `map`, `filter`, `fold`, `try_fold`, `for_each`, or `try_for_each` instead:",
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
                    "contains `for` loop; use iterator methods instead",
                ),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn spawned_tasks_must_retain_an_owner() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("5d0d5bf0"),
        super::types::SourceTextRef::from("spawned task handles are discarded:"),
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
        super::types::StaticStr("321360d4"),
        super::types::SourceTextRef::from(
            "direct environment or filesystem access exists outside approved configuration, tooling, test, and persistence boundaries:",
        ),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if path_text.contains("/config_lib/")
                || path_text.contains("/macro_clippy_check_common/")
                || path_text.contains("/macros_helpers/")
                || path_text.contains("/tests/")
                || path_text.contains("/workspace_test_runner/")
                || path_text.contains("/initialize_environment_files/")
                || path_text.ends_with("server_runtime/src/bounded_read.rs")
                || path_text.ends_with("server_admin_frontend/src/lib.rs")
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
                (call.starts_with("std::env::")
                    || call.starts_with("std::fs::")
                    || call.starts_with("tokio::fs::"))
                .then(|| format!("{}: direct `{call}`", path.display()))
            }));
        },
    );
}
#[test]
fn runtime_data_reads_are_bounded() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("37b593ce"),
        super::types::SourceTextRef::from(
            "runtime code performs an unbounded file or HTTP response read:",
        ),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if path_text.contains("/tests/")
                || path_text.contains("/macros_helpers/")
                || path_text.contains("/macro_clippy_check_common/")
                || path_text.contains("/workspace_test_runner/")
                || path_text.contains("/initialize_environment_files/")
                || path_text.ends_with("server_runtime/src/bounded_read.rs")
                || path_text.ends_with("server_admin_frontend/src/lib.rs")
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
fn raw_runtime_sql_identifier_inventory_matches_reviewed_baseline() {
    let mut observed = std::collections::BTreeMap::<String, usize>::new();
    super::for_each_rs_file_content(|path, content| {
        let path_text = path.to_string_lossy();
        if path_text.contains("/tests/")
            || path_text.ends_with("pg_crud/pg_crud_common/src/sql_identifier.rs")
        {
            return;
        }
        let count = [" FROM ", " INTO ", "UPDATE "]
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
    let expected = std::collections::BTreeMap::from([
        ("../pg_crud/pg_table/src/lib.rs".to_owned(), 7usize),
        ("../server_admin/src/auth.rs".to_owned(), 7usize),
        ("../server_admin/src/auth/audit.rs".to_owned(), 2usize),
        ("../server_admin/src/auth/handlers.rs".to_owned(), 46usize),
        ("../server_admin/src/auth/rate_limit.rs".to_owned(), 2usize),
        ("../server_admin/src/auth/session.rs".to_owned(), 6usize),
        ("../server_admin/src/cleanup.rs".to_owned(), 10usize),
        ("../server_admin/src/migrations.rs".to_owned(), 4usize),
    ]);
    assert_eq!(observed, expected, "raw SQL identifier baseline changed");
}
#[test]
fn direct_process_command_creation_stays_in_shared_tooling() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("f170aa14"),
        super::types::SourceTextRef::from(
            "direct Command::new usage exists outside macros_helpers::tool_command:",
        ),
        |path, ast, ers| {
            if path.ends_with("macros_helpers/src/tool_command.rs") {
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
                    .filter(|call| call == "std::process::Command::new")
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
            if call == "std::process::abort" {
                observed_abort_paths.push(path.to_string_lossy().to_string());
            }
            if call.ends_with("::transmute") {
                ers.push(format!("{}: forbidden `{call}`", path.display()));
            }
        });
    });
    observed_abort_paths.sort();
    let expected_abort_suffixes = [
        "macros_helpers/src/panic_if_err.rs",
        "pg_crud/where_filters/src/lib.rs",
    ];
    let baseline_matches = observed_abort_paths.len() == expected_abort_suffixes.len()
        && expected_abort_suffixes.iter().all(|suffix| {
            observed_abort_paths
                .iter()
                .any(|path| path.ends_with(suffix))
        });
    if !baseline_matches {
        ers.push(format!(
            "abort inventory changed; reviewed suffixes={expected_abort_suffixes:?}, observed={observed_abort_paths:?}"
        ));
    }
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr("f87f82b6"),
        super::types::SourceTextRef::from("abort/transmute policy violations:"),
    );
}
#[test]
fn unit_tests_use_deterministic_time_and_randomness_patterns() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("821d4a76"),
        super::types::SourceTextRef::from(
            "unit tests use nondeterministic time, sleep, or randomness without a reviewed owner:",
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::TestNondeterminismVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                    test_depth: super::types::AnalyzerCount::default(),
                },
            );
            visitor.calls.into_iter().for_each(|call| {
                let reviewed = path.ends_with("server_runtime/src/health.rs")
                    && call == "tokio::time::sleep"
                    || path.ends_with("pg_crud/pg_crud_common/src/lib.rs")
                        && call == "uuid::Uuid::new_v4";
                if !reviewed {
                    ers.push(format!("{}: nondeterministic `{call}`", path.display()));
                }
            });
        },
    );
}
#[test]
fn no_todo_or_unimplemented_macro_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("c4e9a2d7"),
        super::types::SourceTextRef::from("todo!/unimplemented! found:"),
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
                super::types::SourceTextRef::from("contains todo!()"),
                visitor.todo_found,
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from("contains unimplemented!()"),
                visitor.unimplemented_found,
            );
        },
    );
}
#[test]
fn no_macro_rules_in_source_code() {
    let macro_name = "macro_rules";
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
        super::types::StaticStr("b6e2a9f4"),
        super::types::SourceTextRef::from(
            "macro_rules found; use workspace proc-macro crates instead:",
        ),
    );
}
#[test]
fn no_include_asset_macros_outside_allowlist() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("a6d4f2c9"),
        super::types::SourceTextRef::from(
            "include_str!() or include_bytes!() found outside explicit generated/test fixture allowlist:",
        ),
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
        super::types::StaticStr("b4e7c2a9"),
        super::types::SourceTextRef::from(
            "use imports found outside explicit facade re-export files; prefer explicit paths at usage sites:",
        ),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if path_text.ends_with("server_admin_frontend/src/app.rs")
                || path_text.ends_with("server_admin_frontend/src/app/forms.rs")
                || path_text.ends_with("server_admin_frontend/src/app/tables.rs")
                || path_text.ends_with("server_admin_frontend/src/app/pages.rs")
                || path_text.ends_with("frontend_contract/src/lib.rs")
                || path_text.ends_with("pg_crud/pg_crud_common/src/lib.rs")
                || path_text.ends_with("pg_crud/pg_table/generate_pg_table_src/src/lib.rs")
                || path_text.ends_with("pg_crud/pg_types/generate_pg_types_src/src/lib.rs")
                || path_text
                    .ends_with("pg_crud/where_filters/generate_where_filters_src/src/lib.rs")
                || path_text.ends_with("server_admin/src/lib.rs")
                || path_text.ends_with("server_runtime/src/lib.rs")
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::UseImportVisitor {
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
            ers.extend(
                visitor
                    .public_use_roots
                    .iter()
                    .map(|public_use_root| {
                        format!(
                            "{}: found public use import rooted at `{public_use_root}`; use the explicit path at the usage site",
                            path.display()
                        )
                    }),
            );
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
fn no_type_aliases_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("c6e4f7a1"),
        super::types::SourceTextRef::from("type aliases found; use explicit types at usage sites:"),
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
        super::types::StaticStr("a51f0d3b"),
        super::types::SourceTextRef::from(
            "simple constant aliases found; use the source constant directly:",
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
fn no_duplicated_string_literals_in_non_policy_test_code() {
    let mut literal_locations_by_value = std::collections::BTreeMap::<String, Vec<String>>::new();
    super::for_each_rs_syn_file(|path, ast| {
        let path_text = path.display().to_string();
        if !path_text.contains("/tests/src/")
            || path_text.contains("/tests/src/code_style/")
            || path_text.ends_with("/tests/src/lib.rs")
        {
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
        super::types::StaticStr("de729a31"),
        super::types::SourceTextRef::from(
            "duplicated string literals found in non-policy test code:",
        ),
    );
}
#[test]
fn no_unwrap_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("e8b3a6d2"),
        super::types::SourceTextRef::from("unwrap() found:"),
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
                super::types::SourceTextRef::from("unwrap() call"),
                visitor.found_count,
            );
        },
    );
}
