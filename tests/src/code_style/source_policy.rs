#[test]
fn all_files_are_english_only() {
    let exceptions = [
        "../pg_crud/pg_crud_cmn/src/lib.rs", //contain utf-8 String test
        "../CODE_IMPROVEMENT_PLAN.md",
        "../DEVELOPMENT_PLAN.md",
    ];
    let paths = super::snapshot::project_dir()
        .into_iter()
        .filter_entry(|el_6870bc3d| !super::is_ignored_dir_entry_name(el_6870bc3d.file_name()))
        .filter_map(Result::ok)
        .map(walkdir::DirEntry::into_path)
        .collect::<Vec<std::path::PathBuf>>();
    let mut ers = rayon::iter::ParallelIterator::reduce(
        rayon::iter::ParallelIterator::map(
            rayon::iter::IntoParallelRefIterator::par_iter(&paths),
            |path| {
                if !super::is_allowed_english_check_file(path)
                    || super::is_exception(path, &exceptions)
                {
                    return Vec::new();
                }
                let Ok(v) = std::fs::read_to_string(path) else {
                    return Vec::new();
                };
                super::collect_non_english_symbol_ers(path, &v)
            },
        ),
        Vec::new,
        |mut acc, mut item| {
            acc.append(&mut item);
            acc
        },
    );
    ers.sort();
    super::assert_joined_ers_empty_with_ctx(&ers, "8db37a2f", "non-english symbols:");
}
#[test]
fn check_expect_contains_only_unq_uuid_v4() {
    super::check_expect_or_panic_contains_only_unq_uuid_v4(super::ExpectOrPanic::Expect);
}
#[test]
fn check_panic_contains_only_unq_uuid_v4() {
    super::check_expect_or_panic_contains_only_unq_uuid_v4(super::ExpectOrPanic::Panic);
}
#[test]
fn check_rs_files_contains_only_unq_uuid_v4() {
    let rgx = regex::Regex::new(
        r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b",
    )
    .expect("e098a1ff");
    let mut seen = std::collections::HashSet::new();
    super::for_each_rs_file_content(|_, v| {
        rgx.find_iter(v).for_each(|el_714b3d9c| {
            let uuid = uuid::Uuid::parse_str(el_714b3d9c.as_str()).expect("c9711efd");
            assert!(uuid.get_version_num() == 4, "49b49b21");
            assert!(seen.insert(uuid), "4cf9d239");
        });
    });
}
#[test]
fn no_dbg_macro_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx("f1c7a4e3", "dbg!() found:", |path, ast, ers| {
        let visitor = super::visit_syn_file(ast, super::DbgVisitor { found: false });
        if visitor.found {
            ers.push(format!("{}: contains dbg!()", path.display()));
        }
    });
}
#[test]
fn no_for_loops_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "f4c2a9e1",
        "for loops found; use iterator methods such as `map`, `filter`, `fold`, `try_fold`, `for_each`, or `try_for_each` instead:",
        |path, ast, ers| {
            if super::is_exception(path, &super::FOR_LOOP_SOURCE_EXCEPTIONS) {
                return;
            }
            let visitor = super::visit_syn_file(ast, super::ForLoopVisitor { found_count: 0 });
            super::push_repeated_file_er(
                ers,
                path,
                "contains `for` loop; use iterator methods instead",
                visitor.found_count,
            );
        },
    );
}
#[test]
fn no_empty_lines_in_rust_files() {
    let mut ers = Vec::new();
    super::for_each_rs_file_content(|path, v| {
        ers.extend(super::collect_empty_line_ers(path, v));
    });
    super::assert_joined_ers_empty_with_ctx(&ers, "3d2fc8a1", "empty lines found in Rust files:");
}
#[test]
fn no_todo_or_unimplemented_macro_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "c4e9a2d7",
        "todo!/unimplemented! found:",
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                ast,
                super::TodoUnimplVisitor {
                    todo_found: 0,
                    unimplemented_found: 0,
                },
            );
            super::push_repeated_file_er(ers, path, "contains todo!()", visitor.todo_found);
            super::push_repeated_file_er(
                ers,
                path,
                "contains unimplemented!()",
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
        &ers,
        "b6e2a9f4",
        "macro_rules found; use workspace proc-macro crates instead:",
    );
}
#[test]
fn no_include_asset_macros_outside_allowlist() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "a6d4f2c9",
        "include_str!() or include_bytes!() found outside explicit generated/test fixture allowlist:",
        |path, ast, ers| {
            if super::is_exception(path, &super::INCLUDE_ASSET_MACRO_SOURCE_EXCEPTIONS) {
                return;
            }
            let visitor =
                super::visit_syn_file(ast, super::IncludeAssetMacroVisitor { ers: Vec::new() });
            ers.extend(visitor.ers.into_iter().map(|er| {
                    format!(
                        "{}: {er}; add only generated/test fixture files to super::INCLUDE_ASSET_MACRO_SOURCE_EXCEPTIONS",
                        path.display()
                    )
                }));
        },
    );
}
#[test]
fn no_non_public_use_imports_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "b4e7c2a9",
        "use imports found outside explicit facade re-export files; prefer explicit paths at usage sites:",
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                ast,
                super::UseImportVisitor {
                    found_non_public_use_import: false,
                    found_use_rename: false,
                    public_use_roots: Vec::new(),
                },
            );
            if visitor.found_non_public_use_import {
                ers.push(format!(
                    "{}: found non-public use import; use the explicit path at the usage site",
                    path.display()
                ));
            }
            let local_mod_names = ast
                .items
                .iter()
                .filter_map(|item| {
                    if let syn::Item::Mod(item_mod) = item {
                        Some(item_mod.ident.to_string())
                    } else {
                        None
                    }
                })
                .collect::<std::collections::HashSet<_>>();
            if !super::is_public_reexport_source_path(path) {
                ers.extend(
                        visitor
                            .public_use_roots
                            .iter()
                            .filter(|public_use_root| !local_mod_names.contains(*public_use_root))
                            .map(|public_use_root| {
                                format!(
                                "{}: found public use import rooted at `{public_use_root}` outside facade re-export allowlist; use the explicit path at the usage site or add only intentional facade files to super::PUBLIC_REEXPORT_SOURCE_INCLUSIONS",
                                path.display()
                                )
                            }),
                    );
            }
            if visitor.found_use_rename {
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
        "c6e4f7a1",
        "type aliases found; use explicit types at usage sites:",
        |path, ast, ers| {
            let visitor = super::visit_syn_file(ast, super::TypeAliasVisitor { ers: Vec::new() });
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
#[test]
fn no_simple_constant_aliases_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "a51f0d3b",
        "simple constant aliases found; use the source constant directly:",
        |path, ast, ers| {
            let visitor =
                super::visit_syn_file(ast, super::ConstantAliasVisitor { ers: Vec::new() });
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
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
        let visitor =
            super::visit_syn_file(ast, super::TestStringLiteralVisitor { values: Vec::new() });
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
        &ers,
        "de729a31",
        "duplicated string literals found in non-policy test code:",
    );
}
#[test]
fn no_unwrap_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx("e8b3a6d2", "unwrap() found:", |path, ast, ers| {
        let visitor = super::visit_syn_file(ast, super::UnwrapVisitor { found_count: 0 });
        super::push_repeated_file_er(ers, path, "unwrap() call", visitor.found_count);
    });
}
