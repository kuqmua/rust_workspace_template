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
        super::types::StaticStr(str_constants::expr::S_0510),
        super::types::SourceTextRef::from(str_constants::expr::S_1551),
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
    let regex = regex::Regex::new(str_constants::expr::S_0855).expect("e098a1ff");
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
        super::types::StaticStr(str_constants::expr::S_1269),
        super::types::SourceTextRef::from(str_constants::expr::S_1157),
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
        super::types::StaticStr(str_constants::expr::S_1284),
        super::types::SourceTextRef::from(str_constants::expr::S_1333),
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
                super::types::SourceTextRef::from(str_constants::expr::S_1106),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn spawned_tasks_must_retain_an_owner() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_0378),
        super::types::SourceTextRef::from(str_constants::expr::S_1745),
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
        super::types::StaticStr(str_constants::expr::S_0273),
        super::types::SourceTextRef::from(str_constants::expr::S_1192),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if path_text.contains(str_constants::expr::S_0102)
                || path_text.contains(str_constants::expr::S_0109)
                || path_text.contains(str_constants::expr::S_0110)
                || path_text.contains(str_constants::expr::S_0126)
                || path_text.contains(str_constants::expr::S_0134)
                || path_text.contains(str_constants::expr::S_0106)
                || path_text.ends_with(str_constants::expr::S_1729)
                || path_text.ends_with(str_constants::expr::S_1728)
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
                (call.starts_with(str_constants::expr::S_1760)
                    || call.starts_with(str_constants::expr::S_1761)
                    || call.starts_with(str_constants::expr::S_1828))
                .then(|| format!("{}: direct `{call}`", path.display()))
            }));
        },
    );
}
#[test]
fn runtime_data_reads_are_bounded() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_0283),
        super::types::SourceTextRef::from(str_constants::expr::S_1695),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if path_text.contains(str_constants::expr::S_0126)
                || path_text.contains(str_constants::expr::S_0110)
                || path_text.contains(str_constants::expr::S_0109)
                || path_text.contains(str_constants::expr::S_0134)
                || path_text.contains(str_constants::expr::S_0106)
                || path_text.ends_with(str_constants::expr::S_1729)
                || path_text.ends_with(str_constants::expr::S_1728)
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
        if path_text.contains(str_constants::expr::S_0126)
            || path_text.ends_with(str_constants::expr::S_1603)
        {
            return;
        }
        let count = [
            str_constants::expr::S_0011,
            str_constants::expr::S_0012,
            str_constants::expr::S_0808,
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
    let expected = std::collections::BTreeMap::from([
        (str_constants::expr::S_0072.to_owned(), 7usize),
        (
            str_constants::code_style::STR_CONSTANTS_EXPR_PATH.to_owned(),
            96usize,
        ),
    ]);
    assert_eq!(observed, expected, "raw SQL identifier baseline changed");
}
#[test]
fn direct_process_command_creation_stays_in_shared_tooling() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_1267),
        super::types::SourceTextRef::from(str_constants::expr::S_1191),
        |path, ast, ers| {
            if path.ends_with(str_constants::expr::S_1479) {
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
                    .filter(|call| call == str_constants::expr::S_1762)
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
            if call == str_constants::expr::S_1763 {
                observed_abort_paths.push(path.to_string_lossy().to_string());
            }
            if call.ends_with(str_constants::expr::S_0574) {
                ers.push(format!("{}: forbidden `{call}`", path.display()));
            }
        });
    });
    observed_abort_paths.sort();
    let expected_abort_suffixes = [str_constants::expr::S_1478, str_constants::expr::S_1607];
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
        super::types::StaticStr(str_constants::expr::S_1303),
        super::types::SourceTextRef::from(str_constants::expr::S_0910),
    );
}
#[test]
fn unit_tests_use_deterministic_time_and_randomness_patterns() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_0471),
        super::types::SourceTextRef::from(str_constants::expr::S_1853),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::TestNondeterminismVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                    test_depth: super::types::AnalyzerCount::default(),
                },
            );
            visitor.calls.into_iter().for_each(|call| {
                let reviewed = path.ends_with(str_constants::expr::S_1730)
                    && call == str_constants::expr::S_1829
                    || path.ends_with(str_constants::expr::S_1602)
                        && call == str_constants::expr::S_1887;
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
        super::types::StaticStr(str_constants::expr::S_1043),
        super::types::SourceTextRef::from(str_constants::expr::S_1825),
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
                super::types::SourceTextRef::from(str_constants::expr::S_1107),
                visitor.todo_found,
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from(str_constants::expr::S_1108),
                visitor.unimplemented_found,
            );
        },
    );
}
#[test]
fn no_macro_rules_in_source_code() {
    let macro_name = str_constants::expr::S_1477;
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
        super::types::StaticStr(str_constants::expr::S_0993),
        super::types::SourceTextRef::from(str_constants::expr::S_1476),
    );
}
#[test]
fn no_include_asset_macros_outside_allowlist() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_0890),
        super::types::SourceTextRef::from(str_constants::expr::S_1416),
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
        super::types::StaticStr(str_constants::expr::S_0989),
        super::types::SourceTextRef::from(str_constants::expr::S_1878),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if path_text.ends_with(str_constants::expr::S_1724)
                || path_text.ends_with(str_constants::expr::S_1725)
                || path_text.ends_with(str_constants::expr::S_1727)
                || path_text.ends_with(str_constants::expr::S_1726)
                || path_text.ends_with(str_constants::expr::S_1340)
                || path_text.ends_with(str_constants::expr::S_1602)
                || path_text.ends_with(str_constants::expr::S_1604)
                || path_text.ends_with(str_constants::expr::S_1605)
                || path_text.ends_with(str_constants::expr::S_1606)
                || path_text.ends_with(str_constants::expr::S_1722)
                || path_text.ends_with(str_constants::expr::S_1731)
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
        super::types::StaticStr(str_constants::expr::S_1050),
        super::types::SourceTextRef::from(str_constants::expr::S_1841),
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
        super::types::StaticStr(str_constants::expr::S_0885),
        super::types::SourceTextRef::from(str_constants::expr::S_1740),
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
        if !path_text.contains(str_constants::expr::S_0127)
            || path_text.contains(str_constants::expr::S_0128)
            || path_text.ends_with(str_constants::expr::S_0129)
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
        super::types::StaticStr(str_constants::expr::S_1163),
        super::types::SourceTextRef::from(str_constants::expr::S_1203),
    );
}
#[test]
fn long_production_string_literals_are_reused() {
    let mut literal_locations_by_crate_and_value =
        std::collections::BTreeMap::<(String, String), Vec<String>>::new();
    super::for_each_rs_syn_file(|path, ast| {
        let path_text = path.display().to_string();
        if path_text.contains(str_constants::expr::S_0126)
            || path_text.contains(str_constants::expr::S_0128)
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
                    .split_once(str_constants::expr::S_0121)
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
        super::types::StaticStr(str_constants::expr::S_0559),
        super::types::SourceTextRef::from(str_constants::expr::S_1469),
    );
}
#[test]
fn string_constants_are_declared_only_in_str_constants() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_0421),
        super::types::SourceTextRef::from(str_constants::expr::S_1766),
        |path, ast, ers| {
            if path.to_string_lossy().contains(str_constants::expr::S_0123) {
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
fn string_constant_visitor_allows_only_reviewed_syntax_boundaries() {
    let ast = syn::parse_file(str_constants::code_style::STRING_GUARD_ALLOWED_SYNTAX_FIXTURE)
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
    let ast = syn::parse_file(str_constants::code_style::STRING_GUARD_DETECTION_FIXTURE)
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
        super::types::StaticStr(str_constants::expr::S_1228),
        super::types::SourceTextRef::from(str_constants::expr::S_1866),
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
                super::types::SourceTextRef::from(str_constants::expr::S_1865),
                visitor.found_count,
            );
        },
    );
}
