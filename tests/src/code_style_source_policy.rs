#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct ReviewedPublicFields {
    fields: &'static [&'static str],
    path_suffix: &'static str,
    reason: &'static str,
    struct_name: &'static str,
}

#[derive(Default, optimal_memory_layout::OptimalMemoryLayout)]
struct HandwrittenFieldGetterVisitor {
    violations: super::types::SourceTextList,
}

#[derive(Default, optimal_memory_layout::OptimalMemoryLayout)]
struct ModuleWideSingleCallAllowVisitor {
    violations: super::types::SourceTextList,
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for ModuleWideSingleCallAllowVisitor {
    fn visit_attribute(&mut self, i: &'ast_lt syn::Attribute) {
        if matches!(&i.style, syn::AttrStyle::Inner(_))
            && i.path().is_ident(constants_str::VALUE_41008373)
            && matches!(&i.meta, syn::Meta::List(list) if list
                .tokens
                .to_string()
                .split_whitespace()
                .collect::<String>()
                .contains(constants_str::SHARED_VALUES_CLIPPY_SINGLE_CALL_FN))
        {
            self.violations.push(format!(
                "line {}: clippy::single_call_fn must be allowed only on the exact item",
                syn::spanned::Spanned::span(i).start().line
            ));
        }
        syn::visit::visit_attribute(self, i);
    }
}

impl<'ast_lt> syn::visit::Visit<'ast_lt> for HandwrittenFieldGetterVisitor {
    fn visit_item_impl(&mut self, i: &'ast_lt syn::ItemImpl) {
        if i.trait_.is_none() {
            i.items.iter().for_each(|item| {
                let syn::ImplItem::Fn(method) = item else {
                    return;
                };
                if method.sig.inputs.len() == constants_usize::ONE
                    && method
                        .sig
                        .ident
                        .to_string()
                        .starts_with(constants_str::GETTER_PREFIX)
                {
                    self.violations.push(method.sig.ident.to_string());
                }
            });
        }
        syn::visit::visit_item_impl(self, i);
    }
}

#[test]
fn single_call_fn_is_never_allowed_for_a_whole_module() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = ModuleWideSingleCallAllowVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.violations.into_iter().map(|violation| {
                    format!("{}:{violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        super::assert_joined_ers_empty(
            super::types::SourceTextListRef::from(violations.as_slice()),
            super::types::StaticStr::from(
                constants_str::CODE_STYLE_SINGLE_CALL_FN_ITEM_SCOPE_REASON,
            ),
        );
    });
}

#[test]
fn field_getters_are_generated() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = HandwrittenFieldGetterVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.violations.into_iter().map(|method| {
                    format!(
                        "{} contains handwritten field getter `{method}`; derive generate_accessor::Getters",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn struct_fields_do_not_use_crate_visibility() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = super::source_analysis::CrateVisibleStructFieldVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.violations.into_iter().map(|item| {
                    format!(
                        "{} exposes a crate-visible struct field in {item}",
                        source_file.path().as_ref().display()
                    )
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "{violations:#?}");
    });
}

#[test]
fn provider_traits_do_not_use_get_prefix() {
    let pattern = regex::Regex::new(constants_str::VALUE_B2BAA955)
        .expect("cbe7bf15 provider trait regex must compile");
    let mut ers = Vec::new();
    super::for_each_rs_file(|file| {
        pattern
            .captures_iter(file.content().as_ref())
            .for_each(|captures| {
                let Some(name) = captures.get(1usize).map(|value| value.as_str()) else {
                    return;
                };
                ers.push(format!(
                    "{}: provider trait `{name}` must use the `Provider` suffix",
                    file.path().as_ref().display()
                ));
            });
    });
    ers.sort();
    super::assert_joined_ers_empty(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(constants_str::VALUE_669E43DB),
    );
}

#[test]
fn all_files_are_english_only() {
    let mut ers = super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        rayon::iter::ParallelIterator::reduce(
            rayon::iter::ParallelIterator::map(
                rayon::iter::IntoParallelRefIterator::par_iter(snapshot.project_source_files()),
                |source_file| {
                    source_file
                        .content()
                        .as_ref()
                        .lines()
                        .enumerate()
                        .flat_map(|(line_idx, line)| {
                            let line_number = line_idx.saturating_add(1usize);
                            line.chars()
                                .filter(|ch| {
                                    !matches!(ch, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}')
                                        && !ch.is_ascii()
                                })
                                .map(move |ch| {
                                    format!(
                                        "{}:{} non-english symbol `{}` (U+{:04X})",
                                        source_file.path().as_ref().display(),
                                        line_number,
                                        ch,
                                        u32::from(ch)
                                    )
                                })
                        })
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
        super::types::StaticStr::from(constants_str::VALUE_8DB37A2F),
        super::types::SourceTextRef::from(constants_str::NON_ENGLISH_SYMBOLS),
    );
}
#[test]
fn expect_and_panic_messages_start_with_unique_diagnostic_ids() {
    let reviewed_interpolations = [
        (
            constants_str::VALUE_7FE2AF02,
            constants_str::VALUE_265FF5BA,
            constants_str::VALUE_B4F7B36F,
        ),
        (
            constants_str::VALUE_7FE2AF02,
            constants_str::VALUE_A5D61573,
            constants_str::VALUE_B4F7B36F,
        ),
        (
            constants_str::VALUE_D405F3E1,
            constants_str::VALUE_31DDD380,
            constants_str::VALUE_9EB896D7,
        ),
    ];
    let mut all_ids = Vec::new();
    let mut all_ers = Vec::new();
    let mut matched_interpolations = std::collections::BTreeSet::new();
    super::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(ast),
            super::source_analysis::DiagnosticIdVisitor {
                ers: super::types::DiagnosticMsgs::default(),
                ids: super::types::SourceTextList::default(),
            },
        );
        all_ids.extend(visitor.ids);
        visitor.ers.into_iter().for_each(|error| {
            let reviewed =
                reviewed_interpolations
                    .iter()
                    .find(|(path_suffix, reviewed_error, reason)| {
                        let path_text = path.to_string_lossy();
                        let split_owner_matches = path_suffix
                            .strip_suffix(constants_str::RS_EXTENSION)
                            .and_then(|owner_stem| {
                                path_text
                                    .trim_start_matches(constants_str::TEXT_ALT_9)
                                    .strip_prefix(owner_stem)
                            })
                            .is_some_and(|remainder| {
                                remainder.starts_with('_')
                                    && remainder.ends_with(constants_str::RS_EXTENSION)
                            });
                        (path.ends_with(path_suffix)
                            || super::declared_child_matches(path_text.as_ref(), path_suffix)
                            || split_owner_matches)
                            && error == *reviewed_error
                            && !reason.is_empty()
                    });
            if let Some((path_suffix, reviewed_error, _reason)) = reviewed {
                let _inserted = matched_interpolations
                    .insert((path_suffix.to_string(), reviewed_error.to_string()));
            } else {
                all_ers.push(format!("{path:?}: {error}"));
            }
        });
    });
    if matched_interpolations.len() != reviewed_interpolations.len() {
        all_ers.push(format!(
            "stale generated diagnostic interpolation inventory: matched={matched_interpolations:#?}"
        ));
    }
    let mut seen = std::collections::HashSet::new();
    let duplicates = all_ids
        .iter()
        .filter(|identifier| !seen.insert(identifier.as_str()))
        .cloned()
        .collect::<Vec<String>>();
    if !duplicates.is_empty() {
        all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
    }
    assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}");
}
#[test]
fn diagnostic_id_visitor_checks_expect_methods_and_panic_macros() {
    let ast = syn::parse_file(constants_str::VALUE_D1E0CA47)
        .expect("95d174ac fixture invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::DiagnosticIdVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            ids: super::types::SourceTextList::default(),
        },
    );
    assert!(visitor.ers.is_empty());
    assert_eq!(
        visitor.ids.as_slice(),
        [String::from("1a2b3c4d"), String::from("5e6f7a8b")]
    );

    let invalid_ast = syn::parse_file(constants_str::VALUE_BFBFB833)
        .expect("6c3a48f1 fixture invariant must hold");
    let invalid_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&invalid_ast),
        super::source_analysis::DiagnosticIdVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            ids: super::types::SourceTextList::default(),
        },
    );
    assert_eq!(invalid_visitor.ers.len(), 3usize);
}
#[test]
fn diagnostic_id_visitor_checks_generated_expect_and_panic_tokens() {
    let ast = syn::parse_file(constants_str::VALUE_38F6372C)
        .expect("227c291c generate invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::DiagnosticIdVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            ids: super::types::SourceTextList::default(),
        },
    );
    assert_eq!(visitor.ids.len(), 2usize);
    assert_eq!(visitor.ers.len(), 3usize);
}
#[test]
fn check_rs_files_contains_only_unique_uuid_v4() {
    let regex = regex::Regex::new(constants_str::B_0_9A_FA_F_8_0_9A_FA_F_4_4)
        .expect("e098a1ff check_rs_files_contains_only_unique_uuid_v4 invariant must hold");
    let mut seen = std::collections::HashSet::new();
    super::for_each_rs_file(|file| {
        let v = file.content().as_ref();
        regex.find_iter(v).for_each(|element_714b3d9c| {
            let uuid = uuid::Uuid::parse_str(element_714b3d9c.as_str())
                .expect("c9711efd check_rs_files_contains_only_unique_uuid_v4 invariant must hold");
            assert!(uuid.get_version_num() == 4, "49b49b21");
            assert!(seen.insert(uuid), "4cf9d239");
        });
    });
}
#[test]
fn no_dbg_macro_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::F1C7A4E3),
        super::types::SourceTextRef::from(constants_str::DBG_FOUND),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::DbgVisitor {
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
        super::types::StaticStr::from(constants_str::F4C2A9E1),
        super::types::SourceTextRef::from(
            constants_str::FOR_LOOPS_FOUND_USE_ITERATOR_METHODS_SUCH_AS_MAP_FILTER_FOLD_TRY,
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ForLoopVisitor {
                    found_count: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::PathRef::from(path),
                super::types::SourceTextRef::from(
                    constants_str::CONTAINS_FOR_LOOP_USE_ITERATOR_METHODS_INSTEAD,
                ),
                visitor.found_count,
            );
        },
    );
}

#[test]
fn map_err_does_not_discard_source_with_wildcard() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|source_file| {
                let mut visitor = super::source_analysis::SourceDroppingMapErrVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                (visitor.found_count.get() != constants_usize::ZERO).then(|| {
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
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter_map(|source_file| {
                let mut visitor = super::source_analysis::NumericAsCastVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                (visitor.found_count.get() != constants_usize::ZERO).then(|| {
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
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let mut visitor = super::source_analysis::SerdeJsonValueFieldVisitor::default();
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
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        assert_eq!(
            constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_SETS.len(),
            constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_SUFFIXES.len()
        );
        assert_eq!(
            constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_SETS.len(),
            constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_REASONS.len()
        );
        assert_eq!(
            constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_SETS.len(),
            constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_STRUCT_NAMES.len()
        );
        let reviewed_public_fields = constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_SETS
            .iter()
            .zip(constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_SUFFIXES)
            .zip(constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_REASONS)
            .zip(constants_str::CODE_STYLE_REVIEWED_PUBLIC_FIELD_STRUCT_NAMES)
            .map(
                |(((fields, path_suffix), reason), struct_name)| ReviewedPublicFields {
                    fields,
                    path_suffix,
                    reason,
                    struct_name,
                },
            )
            .collect::<Vec<ReviewedPublicFields>>();
        let mut matched = std::collections::BTreeSet::<(String, String)>::new();
        let mut violations = Vec::new();
        snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !super::is_test_crate_source_path(super::types::PathRef::from(
                    source_file.path().as_ref(),
                ))
                .get()
            })
            .for_each(|source_file| {
                let mut visitor = super::source_analysis::PublicStructFieldVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, source_file.ast().as_ref());
                visitor.violations.into_iter().for_each(|item| {
                    let path = source_file.path().as_ref();
                    let reviewed_match = reviewed_public_fields.iter().find(|reviewed| {
                        (path.ends_with(reviewed.path_suffix)
                            || super::declared_child_matches(
                                path.to_string_lossy().as_ref(),
                                reviewed.path_suffix,
                            ))
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
                            "{} exposes an unreviewed non-private field in {item}; keep the field private and expose access through a getter method, preferably generated with #[derive(generate_accessor::Getters)]",
                            path.display()
                        ));
                    }
                });
            });
        let expected = reviewed_public_fields
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
        reviewed_public_fields
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
fn struct_field_visibility_policy_rejects_restricted_visibility() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRUCT_FIELD_VISIBILITY_FIXTURE)
        .expect("8c99de4e struct field visibility fixture must parse");
    let mut visitor = super::source_analysis::PublicStructFieldVisitor::default();
    syn::visit::Visit::visit_file(&mut visitor, &ast);
    assert_eq!(
        visitor.violations.as_slice(),
        [
            "Example::parent",
            "Example::workspace",
            "Example::restricted",
            "Example::public",
        ],
        "e69e2e99"
    );
}
#[test]
fn spawned_tasks_must_retain_an_owner() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_5D0D5BF0),
        super::types::SourceTextRef::from(constants_str::SPAWNED_TASKS_ARE_DISCARDED),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::LostSpawnVisitor {
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
fn spawned_task_policy_rejects_bare_wildcard_and_ignored_bindings() {
    let ast = syn::parse_file(constants_str::VALUE_EBB24851)
        .expect("94b344d7 spawn_tasks invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::LostSpawnVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 4usize);
}
#[test]
fn direct_environment_and_filesystem_access_stays_at_owned_boundaries() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_321360D4),
        super::types::SourceTextRef::from(constants_str::DIRECT_ENVIRONMENT_OR_FILESYSTEM_ACCESS_EXISTS_OUTSIDE_APPROVED_CONFIGURATION_TOOLING_TEST_AND),
        |path, ast, ers| {
            if super::is_test_crate_source_path(super::types::PathRef::from(path)).get()
                || super::is_direct_fs_owner_source_path(super::types::PathRef::from(path)).get()
                || path.ends_with(constants_str::SERVER_RUNTIME_SRC_BOUNDED_READ_RS)
                || super::declared_child_matches(
                    path.to_string_lossy().as_ref(),
                    constants_str::SERVER_RUNTIME_SRC_BOUNDED_READ_RS,
                )
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::DirectPathCallVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(visitor.calls.into_iter().filter_map(|call| {
                (call.starts_with(constants_str::STD_PATH_ENV_PATH)
                    || call.starts_with(constants_str::STD_PATH_FS_PATH)
                    || call.starts_with(constants_str::TOKIO_PATH_FS_PATH))
                .then(|| format!("{}: direct `{call}`", path.display()))
            }));
        },
    );
}
#[test]
fn direct_filesystem_owner_inventory_is_exact_justified_and_current() {
    assert_eq!(
        constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES.len(),
        constants_str::CODE_STYLE_DIRECT_FS_OWNER_REASONS.len(),
        "6e1a9c30"
    );
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("4d82f1b7 direct_filesystem_owner_inventory_is_exact_justified_and_current invariant must hold");
    let missing_or_unjustified = constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
        .iter()
        .zip(constants_str::CODE_STYLE_DIRECT_FS_OWNER_REASONS)
        .filter(|(suffix, reason)| {
            let reviewed_path = workspace_root.join(suffix.trim_start_matches('/'));
            let split_owner_exists = reviewed_path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .zip(reviewed_path.parent())
                .is_some_and(|(stem, parent)| parent.join(stem).is_dir());
            reason.trim().is_empty() || (!reviewed_path.is_file() && !split_owner_exists)
        })
        .collect::<Vec<(&&str, &str)>>();
    assert!(
        missing_or_unjustified.is_empty(),
        "c70b25ea {missing_or_unjustified:?}"
    );
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(source_file.ast().as_ref()),
                super::source_analysis::DirectPathCallVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            let has_direct_access = visitor.calls.iter().any(|call| {
                call.starts_with(constants_str::STD_PATH_ENV_PATH)
                    || call.starts_with(constants_str::STD_PATH_FS_PATH)
                    || call.starts_with(constants_str::TOKIO_PATH_FS_PATH)
            });
            if !has_direct_access {
                return;
            }
            let path = source_file.path().as_ref().to_string_lossy();
            constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
                .iter()
                .filter(|suffix| {
                    path.ends_with(**suffix) || super::declared_child_matches(path.as_ref(), suffix)
                })
                .for_each(|suffix| {
                    let _inserted = matched.insert(*suffix);
                });
        });
        let stale = constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
            .iter()
            .filter(|suffix| !matched.contains(**suffix))
            .collect::<Vec<&&str>>();
        assert!(
            stale.is_empty(),
            "3c9e41b7 stale direct filesystem owners: {stale:?}"
        );
    });
    assert!(
        super::is_direct_fs_owner_source_path(super::types::PathRef::from(std::path::Path::new(
            "../workspace_scaffold/src/template_fs_copy_template_tree.rs"
        )))
        .get(),
        "5b71e44a"
    );
    assert!(
        !super::is_direct_fs_owner_source_path(super::types::PathRef::from(std::path::Path::new(
            "../workspace_scaffold/src/unrelated.rs"
        )))
        .get(),
        "f1428b6c"
    );
}
#[test]
fn runtime_data_reads_are_bounded() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_37B593CE),
        super::types::SourceTextRef::from(
            constants_str::RUNTIME_CODE_PERFORMS_AN_UNBOUNDED_FILE_OR_HTTP_RESPONSE_READ,
        ),
        |path, ast, ers| {
            let path_text = path.to_string_lossy();
            if super::is_test_crate_source_path(super::types::PathRef::from(path)).get()
                || constants_str::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES
                    .iter()
                    .any(|suffix| path_text.ends_with(suffix))
                || path_text.ends_with(constants_str::SERVER_RUNTIME_SRC_BOUNDED_READ_RS)
                || super::declared_child_matches(
                    path_text.as_ref(),
                    constants_str::SERVER_RUNTIME_SRC_BOUNDED_READ_RS,
                )
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::UnboundedReadVisitor {
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
        !constants_str::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES
            .iter()
            .any(|suffix| suffix.contains(constants_str::INITIALIZE_ENVIRONMENT_FILES)),
        "920fde35"
    );
}
#[test]
fn workspace_scaffold_is_in_bounded_read_policy_scope() {
    assert!(
        !constants_str::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES
            .contains(&constants_str::CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX),
        "54b718ca"
    );
}
#[test]
fn bounded_read_policy_has_no_whole_file_owner_exceptions() {
    assert!(
        constants_str::CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES.is_empty(),
        "b71f043c"
    );
}
#[test]
fn raw_runtime_sql_identifier_inventory_matches_reviewed_baseline() {
    let mut observed = std::collections::BTreeMap::<String, usize>::new();
    super::for_each_rs_file(|file| {
        let (path, content) = (file.path().as_ref(), file.content().as_ref());
        let path_text = path.to_string_lossy();
        if super::is_test_crate_source_path(super::types::PathRef::from(path)).get()
            || path_text.ends_with(constants_str::CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX)
            || path_text.ends_with(constants_str::PG_CRUD_PG_CRUD_COMMON_SRC_SQL_IDENTIFIER_RS)
            || super::is_str_constants_source_path(super::types::PathRef::from(path)).get()
        {
            return;
        }
        let count = [
            constants_str::FROM,
            constants_str::INTO,
            constants_str::UPDATE,
        ]
        .into_iter()
        .map(|pattern| content.matches(pattern).count())
        .sum::<usize>();
        if count != constants_usize::ZERO {
            let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("19512c63 raw_runtime_sql_identifier_inventory_matches_reviewed_baseline invariant must hold");
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
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                let path = source_file.path().as_ref().to_string_lossy();
                !super::is_test_crate_source_path(super::types::PathRef::from(
                    source_file.path().as_ref(),
                ))
                .get()
                    && !path.ends_with(constants_str::PG_CRUD_COMMON_SRC_PG_ERROR_RS)
                    && !super::is_str_constants_source_path(super::types::PathRef::from(
                        source_file.path().as_ref(),
                    ))
                    .get()
                    && (source_file
                        .content()
                        .as_ref()
                        .contains(constants_str::IS_UNIQUE_VIOLATION_CALL)
                        || source_file
                            .content()
                            .as_ref()
                            .contains(constants_str::PG_SQLSTATE_PREFIX))
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
        super::types::StaticStr::from(constants_str::F170AA14),
        super::types::SourceTextRef::from(constants_str::DIRECT_COMMAND_PATH_NEW_USAGE_EXISTS_OUTSIDE_MACRO_HELPERS_PATH_TOOL_COMMAND),
        |path, ast, ers| {
            if path.ends_with(constants_str::MACRO_HELPERS_SRC_TOOL_COMMAND_RS) {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::DirectPathCallVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .calls
                    .into_iter()
                    .filter(|call| call == constants_str::STD_PATH_PROCESS_PATH_COMMAND_PATH_NEW)
                    .map(|call| format!("{}: direct `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn abort_and_transmute_calls_match_reviewed_baseline() {
    let mut observed_abort_paths = Vec::new();
    let mut ers = Vec::new();
    super::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(ast),
            super::source_analysis::DirectPathCallVisitor {
                calls: super::types::DiagnosticMsgs::default(),
            },
        );
        visitor.calls.into_iter().for_each(|call| {
            if call == constants_str::STD_PATH_PROCESS_PATH_ABORT {
                observed_abort_paths.push(path.to_string_lossy().to_string());
            }
            if call.ends_with(constants_str::PATH_TRANSMUTE) {
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
        super::types::StaticStr::from(constants_str::F87F82B6),
        super::types::SourceTextRef::from(constants_str::ABORT_TRANSMUTE_POLICY_VIOLATIONS),
    );
}
#[test]
fn every_workspace_struct_and_enum_derives_optimal_memory_layout() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_71790FED),
        super::types::SourceTextRef::from(constants_str::VALUE_6264CCC9),
        |path, ast, ers| {
            if path.ends_with(constants_str::VALUE_30B1AC8C) {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::OptimalMemoryLayoutVisitor::default(),
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
fn optimal_memory_layout_derive_visitor_checks_structs_and_enums() {
    let ast = syn::parse_file(
        constants_str::VALUE_936BA38B,
    )
    .expect("34fb5a61 optimal_memory_layout_derive_visitor_checks_structs_and_enums invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::OptimalMemoryLayoutVisitor::default(),
    );
    assert_eq!(
        visitor.ers.as_slice(),
        [
            "enum `MissingEnum` must derive `optimal_memory_layout::OptimalMemoryLayout`",
            "struct `MissingStruct` must derive `optimal_memory_layout::OptimalMemoryLayout`",
        ],
        "42dc6e3b"
    );
}
#[test]
fn unit_tests_use_deterministic_time_and_randomness_patterns() {
    let reviewed_calls = [(
        constants_str::VALUE_4B68F077,
        constants_str::STD_PATH_TIME_PATH_INSTANT_PATH_NOW,
        constants_str::VALUE_14AF303B,
    )];
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_821D4A76),
        super::types::SourceTextRef::from(constants_str::UNIT_TESTS_USE_NONDETERMINISTIC_TIME_SLEEP_OR_RANDOMNESS_WITHOUT_A_REVIEWED_OWNER),
        |path, ast, ers| {
            let scan_entire_file = super::is_test_source_path(super::types::PathRef::from(path))
                .get()
                && !path.ends_with(constants_str::VALUE_4A3D63F7)
                && !path
                    .components()
                    .any(|component| component.as_os_str() == constants_str::CODE_STYLE);
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::TestNondeterminismVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                    test_depth: super::types::AnalyzerCount::from(usize::from(scan_entire_file)),
                },
            );
            visitor.calls.into_iter().for_each(|call| {
                let reviewed = reviewed_calls.iter().any(|(suffix, reviewed_call, reason)| {
                    path.ends_with(suffix) && call == *reviewed_call && !reason.is_empty()
                });
                if !reviewed {
                    ers.push(format!("{}: nondeterministic `{call}`", path.display()));
                }
            });
        },
    );
}
#[test]
fn unit_test_nondeterminism_visitor_rejects_sync_async_time_and_randomness() {
    let ast = syn::parse_file(constants_str::VALUE_402DAFF0)
        .expect("9354f086 integration_test_helper invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::TestNondeterminismVisitor {
            calls: super::types::DiagnosticMsgs::default(),
            test_depth: super::types::AnalyzerCount::default(),
        },
    );
    assert_eq!(
        visitor.calls.as_slice(),
        [
            constants_str::TOKIO_PATH_TIME_PATH_SLEEP,
            constants_str::UUID_PATH_UUID_PATH_NEW_V4,
            constants_str::STD_PATH_TIME_PATH_SYSTEMTIME_PATH_NOW,
            constants_str::STD_PATH_TIME_PATH_INSTANT_PATH_NOW,
            constants_str::RAND_PATH_RNG,
            constants_str::GETRANDOM_PATH_FILL,
            constants_str::RAND_PATH_RNGS_PATH_OS_RNG,
        ],
        "fa8d2bb1"
    );
    let integration_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::TestNondeterminismVisitor {
            calls: super::types::DiagnosticMsgs::default(),
            test_depth: super::types::AnalyzerCount::from(constants_usize::ONE),
        },
    );
    assert_eq!(integration_visitor.calls.len(), 8usize, "78fde80e");
}
#[test]
fn generated_source_templates_do_not_embed_random_test_values() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_1491FF0E),
        super::types::SourceTextRef::from(constants_str::VALUE_920FAF03),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::GeneratedRandomnessVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .calls
                    .into_iter()
                    .map(|call| format!("{}: generated `{call}`", path.display())),
            );
        },
    );
}
#[test]
fn generated_randomness_policy_inspects_quote_token_streams() {
    let source = [
        constants_str::VALUE_B04CA9E8,
        constants_str::VALUE_C7C4300B,
        constants_str::VALUE_2328A0D2,
        constants_str::VALUE_D10B36AA,
    ]
    .join(constants_str::NEWLINE);
    let ast = syn::parse_file(source.as_str()).expect("04e98f91 generated invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::GeneratedRandomnessVisitor {
            calls: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.calls.len(), 2usize);
}
#[test]
fn process_static_state_matches_reviewed_inventory() {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct StaticStateException {
        identifier: &'static str,
        path_suffix: &'static str,
        reason: &'static str,
    }
    let exceptions = [
        StaticStateException {
            identifier: constants_str::VALUE_4E9D8B24,
            path_suffix: constants_str::VALUE_9DDB2371,
            reason: constants_str::VALUE_9B0E1F72,
        },
        StaticStateException {
            identifier: constants_str::VALUE_F783DB26,
            path_suffix: constants_str::VALUE_865824F9,
            reason: constants_str::VALUE_946801A9,
        },
        StaticStateException {
            identifier: constants_str::VALUE_F783DB26,
            path_suffix: constants_str::VALUE_F67EAA19,
            reason: constants_str::VALUE_946801A9,
        },
        StaticStateException {
            identifier: constants_str::VALUE_1CA9CD1C,
            path_suffix: constants_str::VALUE_959AEDDC,
            reason: constants_str::VALUE_C677F169,
        },
        StaticStateException {
            identifier: constants_str::VALUE_5A6DD0A3,
            path_suffix: constants_str::VALUE_959AEDDC,
            reason: constants_str::VALUE_CAD59B9B,
        },
        StaticStateException {
            identifier: constants_str::VALUE_00B29514,
            path_suffix: constants_str::VALUE_96554632,
            reason: constants_str::VALUE_FB4C1B30,
        },
        StaticStateException {
            identifier: constants_str::VALUE_3623F7E2,
            path_suffix: constants_str::VALUE_392D41BA,
            reason: constants_str::VALUE_B2FEB0FD,
        },
        StaticStateException {
            identifier: constants_str::DECLARED_CHILDREN,
            path_suffix: constants_str::VALUE_4A3D63F7,
            reason:
                constants_str::MODULE_DECLARATION_GRAPH_IS_CACHED_FOR_REPOSITORY_POLICY_MATCHING,
        },
    ];
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_118C4174),
        super::types::SourceTextRef::from(constants_str::VALUE_9EC9C4B2),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::StaticStateVisitor {
                    identifiers: super::types::SourceTextList::default(),
                },
            );
            visitor.identifiers.into_iter().for_each(|identifier| {
                let reviewed = exceptions.iter().any(|exception| {
                    (path.ends_with(exception.path_suffix)
                        || super::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            exception.path_suffix,
                        ))
                        && exception.identifier == identifier
                        && !exception.reason.is_empty()
                });
                if !reviewed {
                    ers.push(format!(
                        "{}: unreviewed static `{identifier}`",
                        path.display()
                    ));
                }
            });
        },
    );
}
#[test]
fn library_sources_do_not_use_print_macros() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_776EEBB3),
        super::types::SourceTextRef::from(constants_str::VALUE_9908E138),
        |path, ast, ers| {
            let is_library_source = path
                .ancestors()
                .find(|ancestor| {
                    ancestor
                        .file_name()
                        .is_some_and(|name| name == constants_str::SRC_ALT)
                })
                .is_some_and(|source_directory| {
                    source_directory
                        .join(constants_str::VALUE_0544FC95)
                        .exists()
                });
            if !is_library_source {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::PrintMacroVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            visitor.calls.into_iter().for_each(|call| {
                ers.push(format!("{}: library `{call}!`", path.display()));
            });
        },
    );
}
#[test]
fn production_code_does_not_use_line_print_macros() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_018B0C9F),
        super::types::SourceTextRef::from(constants_str::VALUE_70D9A674),
        |path, ast, ers| {
            if super::is_test_crate_source_path(super::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ProductionLinePrintMacroVisitor {
                    calls: super::types::DiagnosticMsgs::default(),
                },
            );
            visitor.calls.into_iter().for_each(|call| {
                ers.push(format!(
                    "{}: `{call}!`: {}",
                    path.display(),
                    constants_str::VALUE_70D9A674
                ));
            });
        },
    );
}
#[test]
fn module_and_function_names_use_single_underscores() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_AE652DDA),
        super::types::SourceTextRef::from(constants_str::VALUE_63194000),
        |path, ast, ers| {
            if path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|name| {
                    name.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE)
                })
            {
                ers.push(format!(
                    "{}: module filename contains a double underscore",
                    path.display()
                ));
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::DoubleUnderscoreNamingVisitor {
                    identifiers: super::types::DiagnosticMsgs::default(),
                },
            );
            visitor.identifiers.into_iter().for_each(|identifier| {
                ers.push(format!(
                    "{}: `{identifier}` contains a double underscore",
                    path.display()
                ));
            });
        },
    );
}
#[test]
fn module_and_function_names_do_not_use_unclear_short_forms() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_AE652DDA),
        super::types::SourceTextRef::from(constants_str::VALUE_63194000),
        |path, ast, ers| {
            if path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|name| {
                    name.split('_')
                        .any(|part| part == constants_str::WORKSPACE_SHORT_HELPER_TOKEN)
                })
            {
                ers.push(format!(
                    "{}: module filename abbreviates helper as hlp",
                    path.display()
                ));
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ShortFunctionNamingVisitor {
                    identifiers: super::types::DiagnosticMsgs::default(),
                },
            );
            visitor.identifiers.into_iter().for_each(|identifier| {
                ers.push(format!(
                    "{}: `{identifier}` abbreviates make as mk",
                    path.display()
                ));
            });
        },
    );
}
#[test]
fn production_line_print_macro_policy_allows_test_code_and_rejects_production_code() {
    let ast = syn::parse_file(constants_str::VALUE_606F2B07)
        .expect("a508c55d production_line_print_macro_policy fixture must parse");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::ProductionLinePrintMacroVisitor {
            calls: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(
        visitor.calls.as_slice(),
        ["println".to_owned(), "eprintln".to_owned()]
    );
    assert_eq!(
        constants_str::VALUE_70D9A674,
        "instead of using println! and eprintln!, use tracing/telemetry"
    );
}
#[test]
fn sensitive_text_wrappers_do_not_derive_unredacted_debug_or_display() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_2E395A49),
        super::types::SourceTextRef::from(constants_str::VALUE_4C5A6F95),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::SensitiveTextDebugDeriveVisitor {
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
fn sensitive_text_debug_policy_distinguishes_redacted_derives() {
    let ast = syn::parse_file(constants_str::VALUE_BC13B693).expect(
        "3d72b9e0 sensitive_text_debug_policy_distinguishes_redacted_derives invariant must hold",
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::SensitiveTextDebugDeriveVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 4usize);
    assert!(
        visitor
            .ers
            .iter()
            .any(|error| error.contains("ApiTokenRef"))
    );
    assert!(
        visitor
            .ers
            .iter()
            .any(|error| error.contains("ApiKeyBytes"))
    );
    assert!(
        visitor
            .ers
            .iter()
            .any(|error| error.contains("PasswordHash"))
    );
}
#[test]
fn error_formatters_do_not_expose_sensitive_fields() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_10E432C3),
        super::types::SourceTextRef::from(constants_str::VALUE_ED81BDD6),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::SensitiveErrorFormatVisitor {
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
fn sensitive_error_format_policy_rejects_named_and_tuple_placeholders() {
    let ast = syn::parse_file(
        constants_str::VALUE_2CC8E3AF,
    )
    .expect("d8cc09ca sensitive_error_format_policy_rejects_named_and_tuple_placeholders invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::SensitiveErrorFormatVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 2usize);
}
#[test]
fn no_todo_or_unimplemented_macro_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::C4E9A2D7),
        super::types::SourceTextRef::from(constants_str::TODO_UNIMPLEMENTED_FOUND),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::TodoUnimplVisitor {
                    todo_found: super::types::AnalyzerCount::default(),
                    unimplemented_found: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::PathRef::from(path),
                super::types::SourceTextRef::from(constants_str::CONTAINS_TODO),
                visitor.todo_found,
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::PathRef::from(path),
                super::types::SourceTextRef::from(constants_str::CONTAINS_UNIMPLEMENTED),
                visitor.unimplemented_found,
            );
        },
    );
}
#[test]
fn source_lint_suppressions_have_explicit_reasons() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_7410D6B1),
        super::types::SourceTextRef::from(constants_str::VALUE_2DAB1928),
        |path, ast, ers| {
            let source = std::fs::read_to_string(path).expect(
                "8d3bca08 source_lint_suppressions_have_explicit_reasons invariant must hold",
            );
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::AllowReasonVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    lines: super::types::SourceTextList::from(
                        source.lines().map(str::to_owned).collect::<Vec<String>>(),
                    ),
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
fn source_lint_reason_policy_accepts_argument_and_comment_reasons() {
    let source = constants_str::VALUE_1D86D8F2;
    let ast = syn::parse_file(source).expect("ec218827 argument_reason invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::AllowReasonVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            lines: super::types::SourceTextList::from(
                source.lines().map(str::to_owned).collect::<Vec<String>>(),
            ),
        },
    );
    assert_eq!(visitor.ers.len(), constants_usize::ONE);
}
#[test]
fn route_operation_error_policy_rejects_shared_types() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_ROUTE_ENDPOINT_OPERATION_ERROR_FIXTURE)
        .expect("752fbb70 route_operation_error_policy_rejects_shared_types invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::RouteOperationErrorVisitor::default(),
    );
    assert_eq!(visitor.ers.len(), 2usize);
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "iterator form is required by the workspace no-for-loop policy"
)]
fn admin_route_errors_do_not_wrap_a_shared_operation_error() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let auth = snapshot
            .rs_files()
            .iter()
            .find(|file| file.path().as_ref().ends_with(constants_str::VALUE_0690A45F))
            .expect("9585d60c admin_route_errors_do_not_wrap_a_shared_operation_error invariant must hold")
            .content()
            .as_ref();
        let macros = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_00ABFB22)
            })
            .expect("890b3180 admin_route_errors_do_not_wrap_a_shared_operation_error invariant must hold")
            .content()
            .as_ref();
        assert!(!auth.contains("Operation(AdminError)"), "7c9f1bb0");
        assert!(
            auth.contains("frontend_contract::api_operation_error!"),
            "166dc25a"
        );
        assert!(macros.contains("pub fn api_operation_error"), "259e7ebd");
        [
            constants_str::VALUE_D7A45F10,
            constants_str::VALUE_1467E095,
            constants_str::VALUE_C2E67087,
            constants_str::VALUE_71657339,
            constants_str::VALUE_DB71AF6A,
            constants_str::VALUE_B9AFDC8D,
            constants_str::VALUE_00E5A912,
            constants_str::VALUE_EAB76571,
            constants_str::VALUE_91F980AF,
            constants_str::VALUE_0D833D68,
            constants_str::VALUE_682B824C,
            constants_str::VALUE_075F10C0,
        ]
        .iter()
        .for_each(|variant| {
            assert!(
                macros.contains(variant),
                "927e5901: admin route error macro is missing `{variant}`"
            );
        });
    });
}
#[test]
fn source_does_not_retain_commented_debug_statements() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_16B3BD74),
        super::types::SourceTextRef::from(constants_str::VALUE_353E0299),
        |path, _, ers| {
            let source = std::fs::read_to_string(path).expect(
                "2b06297b source_does_not_retain_commented_debug_statements invariant must hold",
            );
            ers.extend(
                super::commented_debug_statements(super::types::SourceTextRef::from(
                    source.as_str(),
                ))
                .into_iter()
                .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn commented_debug_statement_policy_rejects_debug_macros_only() {
    let source = [
        concat!("/", "/", " println!(\"debug\");"),
        concat!("/", "/", " dbg!(value);"),
        concat!("/", "/", " explanation of println usage"),
        constants_str::VALUE_F7D8E121,
    ]
    .join(constants_str::NEWLINE);
    let violations =
        super::commented_debug_statements(super::types::SourceTextRef::from(source.as_str()));
    assert_eq!(violations.len(), 2usize);
}
#[test]
fn project_text_files_have_stable_line_endings_and_no_trailing_whitespace() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("fd1294e4 project_text_files_have_stable_line_endings_and_no_trailing_whitespace invariant must hold");
    let mut command = macro_helpers::domain_types::tool_command::ToolCommand::new(
        macro_helpers::domain_types::tool_command::ToolProgramRef::from(constants_str::GIT_PROGRAM),
    );
    let _command = command
        .current_dir(macro_helpers::domain_types::tool_command::PathRef::from(
            repository_root,
        ))
        .args(
            macro_helpers::domain_types::tool_command::ToolArgsRef::from(
                constants_str::GIT_LS_FILES_ARGS.as_slice(),
            ),
        );
    let output = command.output().expect("e6c52471 project_text_files_have_stable_line_endings_and_no_trailing_whitespace invariant must hold");
    assert!(output.status.success(), "9041df16");
    let tracked_paths = std::str::from_utf8(&output.stdout).expect("b9976fe8 project_text_files_have_stable_line_endings_and_no_trailing_whitespace invariant must hold");
    let mut violations = Vec::<String>::new();
    tracked_paths
        .split_terminator('\0')
        .for_each(|relative_path| {
            let path = repository_root.join(relative_path);
            let bytes = match std::fs::read(path.as_path()) {
                Ok(bytes) => bytes,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
                Err(error) => panic!("d808e460: {error}"),
            };
            let Ok(source) = std::str::from_utf8(bytes.as_slice()) else {
                return;
            };
            violations.extend(
                super::text_content_hygiene_ers(super::types::SourceTextRef::from(source))
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        });
    assert!(violations.is_empty(), "8c22bed1 {violations:#?}");
}
#[test]
fn text_content_hygiene_policy_rejects_all_line_ending_violations() {
    let mut source = String::from(constants_str::FIRST_ALT);
    source.push(' ');
    source.push('\r');
    source.push('\n');
    source.push_str(constants_str::VALUE_3547CB11);
    let violations =
        super::text_content_hygiene_ers(super::types::SourceTextRef::from(source.as_str()));
    assert_eq!(violations.len(), 3usize);
}
#[test]
fn no_macro_rules_in_source_code() {
    let macro_name = constants_str::MACRO_RULES;
    let forbidden = format!("{macro_name}!");
    let mut ers = Vec::new();
    super::for_each_rs_file(|file| {
        let (path, v) = (file.path().as_ref(), file.content().as_ref());
        if v.contains(&forbidden) {
            ers.push(format!(
                "{}: contains {forbidden}; use a workspace proc-macro crate instead",
                path.display()
            ));
        }
    });
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(constants_str::B6E2A9F4),
        super::types::SourceTextRef::from(
            constants_str::MACRO_RULES_FOUND_USE_WORKSPACE_PROC_MACRO_CRATES_INSTEAD,
        ),
    );
}
#[test]
fn no_include_asset_macros_outside_allowlist() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::A6D4F2C9),
        super::types::SourceTextRef::from(constants_str::INCLUDE_STR_OR_INCLUDE_BYTES_FOUND_OUTSIDE_EXPLICIT_GENERATED_TEST_FIXTURE_ALLOWLIST),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::IncludeAssetMacroVisitor {
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
fn append_non_public_use_import_er(
    path: &std::path::Path,
    found_non_public_use_import: super::types::AnalyzerBool,
    ers: &mut Vec<String>,
) {
    if found_non_public_use_import.get() {
        ers.push(format!(
            "{}: found non-public use import; use the explicit path at the usage site",
            path.display()
        ));
    }
}
fn append_public_use_import_ers(
    path: &std::path::Path,
    public_use_roots: &super::types::SourceTextList,
    ers: &mut Vec<String>,
) {
    ers.extend(public_use_roots.iter().map(|public_use_root| {
        format!(
            "{}: found public use import rooted at `{public_use_root}`; use the explicit path at the usage site",
            path.display()
        )
    }));
}
#[test]
#[allow(clippy::wildcard_enum_match_arm)] // syn::Item is non-exhaustive; only modules are relevant
fn public_reexports_are_forbidden_and_private_imports_are_restricted() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::B4E7C2A9),
        super::types::SourceTextRef::from(
            constants_str::FORBIDDEN_PUBLIC_REEXPORTS_OR_PRIVATE_IMPORTS_FOUND_PREFER_EXPLICIT_PATHS,
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::UseImportVisitor {
                    found_non_public_use_import: super::types::AnalyzerBool::default(),
                    found_use_rename: super::types::AnalyzerBool::default(),
                    public_use_roots: super::types::SourceTextList::default(),
                },
            );
            append_non_public_use_import_er(
                path,
                visitor.found_non_public_use_import,
                ers,
            );
            append_public_use_import_ers(path, &visitor.public_use_roots, ers);
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
fn declared_child_does_not_bypass_non_public_use_import_policy() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_DECLARED_CHILD_USE_FIXTURE).expect(
        "b67d5cf1 declared_child_does_not_bypass_non_public_use_import_policy invariant must hold",
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::UseImportVisitor {
            found_non_public_use_import: super::types::AnalyzerBool::default(),
            found_use_rename: super::types::AnalyzerBool::default(),
            public_use_roots: super::types::SourceTextList::default(),
        },
    );
    let mut ers = Vec::<String>::new();
    append_non_public_use_import_er(
        std::path::Path::new(constants_str::CODE_STYLE_DECLARED_CHILD_FIXTURE_PATH),
        visitor.found_non_public_use_import,
        &mut ers,
    );
    append_non_public_use_import_er(
        std::path::Path::new(constants_str::CODE_STYLE_NESTED_OWNER_USE_FIXTURE_PATH),
        visitor.found_non_public_use_import,
        &mut ers,
    );
    assert_eq!(ers.len(), constants_usize::TWO, "e23d18a4");
}
#[test]
fn use_import_policy_detects_private_imports_and_public_reexports() {
    let ast = syn::parse_file(constants_str::VALUE_B2B1AD10).expect(
        "7b9e6f31 use_import_policy_narrows_facade_and_leptos_exceptions invariant must hold",
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::UseImportVisitor {
            found_non_public_use_import: super::types::AnalyzerBool::default(),
            found_use_rename: super::types::AnalyzerBool::default(),
            public_use_roots: super::types::SourceTextList::default(),
        },
    );
    assert!(visitor.found_non_public_use_import.get(), "ac09626a");
    assert!(!visitor.found_use_rename.get(), "c2bff14e");
    assert_eq!(
        visitor.public_use_roots.len(),
        constants_usize::ONE,
        "3f4798c8"
    );

    let leptos_ast = syn::parse_file(constants_str::VALUE_444213A9).expect(
        "56f86b52 use_import_policy_narrows_facade_and_leptos_exceptions invariant must hold",
    );
    let leptos_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&leptos_ast),
        super::source_analysis::UseImportVisitor {
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
fn cfg_test_modules_do_not_hide_forbidden_public_reexports() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_REEXPORT_WITH_LOGIC_FIXTURE)
        .expect("12d3ea75 public re-export fixture must parse");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::UseImportVisitor {
            found_non_public_use_import: super::types::AnalyzerBool::default(),
            found_use_rename: super::types::AnalyzerBool::default(),
            public_use_roots: super::types::SourceTextList::default(),
        },
    );
    let mut ers = Vec::<String>::new();
    append_public_use_import_ers(
        std::path::Path::new(constants_str::CODE_STYLE_DECLARED_CHILD_FIXTURE_PATH),
        &visitor.public_use_roots,
        &mut ers,
    );
    assert_eq!(ers.len(), 2usize, "654501aa");
}
#[test]
fn no_type_aliases_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::C6E4F7A1),
        super::types::SourceTextRef::from(
            constants_str::TYPE_ALIASES_FOUND_USE_EXPLICIT_TYPES_AT_USAGE_SITES,
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::TypeAliasVisitor {
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
fn no_empty_enums_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_19A18AE4),
        super::types::SourceTextRef::from(constants_str::VALUE_721EDC25),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::EmptyEnumVisitor {
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
fn empty_enum_policy_checks_items_and_attribute_payloads() {
    let source = [
        constants_str::VALUE_BFF335C4,
        constants_str::VALUE_D10B36AA,
        constants_str::VALUE_68E5AB24,
        constants_str::VALUE_1A46177C,
        constants_str::VALUE_9DC6533C,
        constants_str::VALUE_1A9A6650,
    ]
    .join(constants_str::NEWLINE);
    let ast = syn::parse_file(&source).expect(
        "e52f247c empty_enum_policy_checks_items_and_attribute_payloads invariant must hold",
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::EmptyEnumVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 2usize);
    assert!(
        visitor
            .ers
            .iter()
            .any(|error| error.contains("DirectlyEmpty"))
    );
    assert!(
        visitor
            .ers
            .iter()
            .any(|error| error.contains("EmptyMarker"))
    );
}
#[test]
fn infallible_functions_return_concrete_types() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_6E60D726),
        super::types::SourceTextRef::from(constants_str::VALUE_4BAB9A8D),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::InfallibleResultVisitor {
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
fn infallible_result_policy_rejects_wrappers_and_free_function_results() {
    let source = [
        constants_str::VALUE_117099FD,
        constants_str::VALUE_35B13C3B,
        constants_str::VALUE_356A6CFB,
        constants_str::VALUE_73D6360C,
        constants_str::VALUE_41324880,
    ]
    .join(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
    let ast = syn::parse_file(&source).expect("aa0bacf7 concrete invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::InfallibleResultVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 2usize);
}
#[test]
fn no_simple_constant_aliases_in_rust_sources() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::A51F0D3B),
        super::types::SourceTextRef::from(
            constants_str::SIMPLE_CONSTANT_ALIASES_FOUND_USE_THE_SOURCE_CONSTANT_DIRECTLY,
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ConstantAliasVisitor {
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
        super::types::StaticStr::from(constants_str::VALUE_CF47C890),
        super::types::SourceTextRef::from(constants_str::VALUE_43AA05FB),
        |path, ast, ers| {
            let is_required_foundation_impl = [
                (
                    std::path::Path::new(constants_str::VALUE_E24F0FD4),
                    constants_str::VALUE_403B3BAE,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_C809930D),
                    constants_str::VALUE_D0D0184F,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_2900052A),
                    constants_str::VALUE_E5996CB1,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_1354D9A9),
                    constants_str::VALUE_2A080280,
                ),
            ]
            .iter()
            .any(|(suffix, reason)| {
                !reason.is_empty()
                    && (path.ends_with(suffix)
                        || super::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            suffix.to_string_lossy().as_ref(),
                        ))
            });
            if is_required_foundation_impl {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::PassthroughFromVisitor {
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
fn tuple_newtypes_derive_into_inner_from_instead_of_implementing_passthrough_from() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_A1DD158B),
        super::types::SourceTextRef::from(constants_str::VALUE_E8DA133A),
        |path, ast, ers| {
            let is_required_foundation_impl = [
                (
                    std::path::Path::new(constants_str::VALUE_E24F0FD4),
                    constants_str::VALUE_403B3BAE,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_C809930D),
                    constants_str::VALUE_D0D0184F,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_2900052A),
                    constants_str::VALUE_E5996CB1,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_1354D9A9),
                    constants_str::VALUE_2A080280,
                ),
            ]
            .iter()
            .any(|(suffix, reason)| {
                !reason.is_empty()
                    && (path.ends_with(suffix)
                        || super::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            suffix.to_string_lossy().as_ref(),
                        ))
            });
            if is_required_foundation_impl {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::PassthroughIntoInnerFromVisitor {
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
fn tuple_newtypes_derive_into_iterator_instead_of_forwarding_into_iter() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_37F4CEF1),
        super::types::SourceTextRef::from(constants_str::VALUE_7B891FFF),
        |path, ast, ers| {
            let required_foundation_impl = (
                std::path::Path::new(constants_str::VALUE_2900052A),
                constants_str::VALUE_E5996CB1,
            );
            if !required_foundation_impl.1.is_empty()
                && (path.ends_with(required_foundation_impl.0)
                    || super::declared_child_matches(
                        path.to_string_lossy().as_ref(),
                        required_foundation_impl.0.to_string_lossy().as_ref(),
                    ))
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingIntoIteratorVisitor {
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
fn tuple_newtypes_derive_display_instead_of_implementing_forwarding_display() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_DBD6E9F5),
        super::types::SourceTextRef::from(constants_str::VALUE_C333E174),
        |path, ast, ers| {
            let is_required_foundation_impl = [
                (
                    std::path::Path::new(constants_str::VALUE_E24F0FD4),
                    constants_str::VALUE_403B3BAE,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_C809930D),
                    constants_str::VALUE_D0D0184F,
                ),
                (
                    std::path::Path::new(constants_str::VALUE_2900052A),
                    constants_str::VALUE_E5996CB1,
                ),
            ]
            .iter()
            .any(|(suffix, reason)| {
                !reason.is_empty()
                    && (path.ends_with(suffix)
                        || super::declared_child_matches(
                            path.to_string_lossy().as_ref(),
                            suffix.to_string_lossy().as_ref(),
                        ))
            });
            if is_required_foundation_impl {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingDisplayVisitor {
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
fn error_implementations_derive_thiserror_error() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_3FCD79E4),
        super::types::SourceTextRef::from(constants_str::VALUE_18D7D5AB),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ManualErrorImplVisitor {
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
fn json_api_error_responses_originate_from_thiserror_enums() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_1800EA0D),
        super::types::SourceTextRef::from(constants_str::VALUE_FBB3C40C),
        |path, ast, ers| {
            let thiserror_enums = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ThiserrorEnumVisitor::default(),
            );
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::JsonIntoResponseErrorVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    thiserror_enum_names: &thiserror_enums.names,
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
fn json_api_error_response_policy_rejects_structs_and_accepts_thiserror_enums() {
    let ast =
        syn::parse_file(constants_str::CODE_STYLE_JSON_API_ERROR_ENUM_FIXTURE).expect("e45c8f09 json_api_error_response_policy_rejects_structs_and_accepts_thiserror_enums invariant must hold");
    let thiserror_enums = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::ThiserrorEnumVisitor::default(),
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::JsonIntoResponseErrorVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            thiserror_enum_names: &thiserror_enums.names,
        },
    );
    assert_eq!(visitor.ers.len(), 2usize);
}
#[test]
fn api_response_errors_keep_source_locations_out_of_public_error_enums() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_0B0251B3),
        super::types::SourceTextRef::from(constants_str::VALUE_9DE68BBD),
        |path, ast, ers| {
            let thiserror_enums = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ThiserrorEnumVisitor::default(),
            );
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ApiErrorLocationVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    thiserror_location_enum_names: &thiserror_enums.location_names,
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
fn api_response_location_policy_rejects_location_fields() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_JSON_API_ERROR_ENUM_FIXTURE).expect(
        "6d8c50f1 api_response_location_policy_rejects_location_fields invariant must hold",
    );
    let thiserror_enums = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::ThiserrorEnumVisitor::default(),
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::ApiErrorLocationVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            thiserror_location_enum_names: &thiserror_enums.location_names,
        },
    );
    assert_eq!(visitor.ers.len(), 2usize);
}
#[test]
fn api_response_error_sources_use_observed_error() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_44E97EA0),
        super::types::SourceTextRef::from(constants_str::VALUE_7CB245E4),
        |path, ast, ers| {
            let response_types = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::IntoResponseTypeVisitor::default(),
            );
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ApiErrorSourceVisitor {
                    api_error_names: &response_types.names,
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
fn api_response_error_source_policy_rejects_raw_sources() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_JSON_API_ERROR_ENUM_FIXTURE).expect(
        "b26f4527 api_response_error_source_policy_rejects_raw_sources invariant must hold",
    );
    let response_types = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::IntoResponseTypeVisitor::default(),
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::ApiErrorSourceVisitor {
            api_error_names: &response_types.names,
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), constants_usize::ONE);
}
#[test]
#[allow(clippy::needless_for_each)] // workspace policy intentionally avoids for loops
#[allow(clippy::option_if_let_else)] // preserves ownership of the path buffer in the fallback
fn every_fallible_typed_route_operation_has_its_own_error_type() {
    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut groups = std::collections::BTreeMap::<
            String,
            super::source_analysis::RouteOperationErrorVisitor,
        >::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let path = source_file.path().as_ref();
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(source_file.ast().as_ref()),
                super::source_analysis::RouteOperationErrorVisitor::default(),
            );
            let path_text = path.to_string_lossy();
            let normalized_path = path_text
                .trim_start_matches(constants_str::TEXT_ALT_9)
                .trim_start_matches('/');
            let declared_owner = super::declared_children()
                .iter()
                .find_map(|(owner, child)| (child == normalized_path).then_some(owner.as_str()))
                .map(|mut owner| {
                    while let Some(parent) = super::declared_children()
                        .iter()
                        .find_map(|(parent, child)| (child == owner).then_some(parent.as_str()))
                    {
                        owner = parent;
                    }
                    super::types::SourceTextRef::from(owner)
                });
            let group = match declared_owner {
                Some(owner) => owner.get().to_owned(),
                None => path_text.into_owned(),
            };
            let aggregate = groups.entry(group).or_default();
            aggregate.ers.extend(visitor.ers);
            aggregate.registered.extend(visitor.registered);
            aggregate.operations.extend(visitor.operations);
        });
        let mut ers = Vec::new();
        groups.into_iter().for_each(|(path, visitor)| {
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{path}: {error}")),
            );
            visitor
                .registered
                .difference(&visitor.operations)
                .for_each(|endpoint| {
                    ers.push(format!(
                        "{path}: registered endpoint `{endpoint}` must declare its route operation"
                    ));
                });
        });
        super::assert_joined_ers_empty_with_ctx(
            super::types::SourceTextListRef::from(ers.as_slice()),
            super::types::StaticStr::from(constants_str::VALUE_D1557BA1),
            super::types::SourceTextRef::from(constants_str::VALUE_50C1CC72),
        );
    });
}
#[test]
fn typed_route_operation_error_policy_rejects_shared_types() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_ROUTE_OPERATION_ERROR_FIXTURE).expect(
        "60ff98c7 typed_route_operation_error_policy_rejects_shared_types invariant must hold",
    );
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::RouteOperationErrorVisitor::default(),
    );
    assert_eq!(visitor.ers.len(), constants_usize::ONE);
}
#[test]
fn error_implementation_source_uses_only_thiserror_derive() {
    let forbidden_newtype_derive = concat!("newtype::", "Error");
    let forbidden_manual_impl = concat!("impl std::error::", "Error for");
    let mut ers = Vec::new();
    super::for_each_rs_file(|file| {
        let (path, source) = (file.path().as_ref(), file.content().as_ref());
        [forbidden_newtype_derive, forbidden_manual_impl]
            .into_iter()
            .filter(|forbidden| source.contains(forbidden))
            .for_each(|forbidden| {
                ers.push(format!(
                    "{}: contains forbidden `{forbidden}`",
                    path.display()
                ));
            });
    });
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(constants_str::VALUE_7729AA39),
        super::types::SourceTextRef::from(constants_str::VALUE_2B539A50),
    );
}
#[test]
fn tuple_newtypes_derive_not_inner_instead_of_implementing_not() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_0E9309F2),
        super::types::SourceTextRef::from(constants_str::VALUE_00F4142B),
        |path, ast, ers| {
            let foundation_owner = constants_str::VALUE_2900052A;
            if path.ends_with(std::path::Path::new(foundation_owner))
                || super::declared_child_matches(path.to_string_lossy().as_ref(), foundation_owner)
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ManualNotImplVisitor {
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
fn constant_display_implementations_derive_display_const() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_2D6FAA55),
        super::types::SourceTextRef::from(constants_str::VALUE_A788CCC5),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ConstDisplayImplVisitor {
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
        super::types::StaticStr::from(constants_str::VALUE_24B5ACA8),
        super::types::SourceTextRef::from(constants_str::VALUE_801C5785),
        |path, ast, ers| {
            let required_foundation_impl = (
                std::path::Path::new(constants_str::VALUE_2900052A),
                constants_str::VALUE_E5996CB1,
            );
            if !required_foundation_impl.1.is_empty()
                && (path.ends_with(required_foundation_impl.0)
                    || super::declared_child_matches(
                        path.to_string_lossy().as_ref(),
                        required_foundation_impl.0.to_string_lossy().as_ref(),
                    ))
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingDerefVisitor {
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
fn tuple_newtypes_derive_borrow_instead_of_implementing_forwarding_borrow() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_1259718C),
        super::types::SourceTextRef::from(constants_str::VALUE_38822A0E),
        |path, ast, ers| {
            let required_foundation_impl = (
                std::path::Path::new(constants_str::VALUE_E24F0FD4),
                constants_str::VALUE_403B3BAE,
            );
            if !required_foundation_impl.1.is_empty() && path.ends_with(required_foundation_impl.0)
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::ForwardingBorrowVisitor {
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
    super::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let path_text = path.display().to_string();
        if !super::is_non_policy_test_source_path(super::types::PathRef::from(path)).get() {
            return;
        }
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(ast),
            super::source_analysis::TestStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            },
        );
        visitor
            .values
            .into_iter()
            .filter(|literal_value| !literal_value.is_empty())
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
        super::types::StaticStr::from(constants_str::DE729A31),
        super::types::SourceTextRef::from(
            constants_str::DUPLICATED_STRING_LITERALS_FOUND_IN_NON_POLICY_TEST_CODE,
        ),
    );
}
#[test]
fn ordinary_test_fixture_is_in_duplicate_string_policy_scope() {
    assert!(
        super::is_non_policy_test_source_path(super::types::PathRef::from(std::path::Path::new(
            constants_str::CODE_STYLE_DOMAIN_FIXTURE_PATH
        )))
        .get(),
        "f2ec448d"
    );
    assert!(
        !super::is_non_policy_test_source_path(super::types::PathRef::from(std::path::Path::new(
            constants_str::TESTS_SRC_CODE_STYLE
        )))
        .get(),
        "8df61a91"
    );
}
#[test]
fn production_string_literals_are_reused() {
    let mut literal_locations_by_value = std::collections::BTreeMap::<String, Vec<String>>::new();
    super::for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let path_text = path.display().to_string();
        if super::is_test_crate_source_path(super::types::PathRef::from(path)).get()
            || super::is_code_style_meta_harness_source_path(super::types::PathRef::from(path))
                .get()
            || super::is_str_constants_source_path(super::types::PathRef::from(path)).get()
        {
            return;
        }
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(ast),
            super::source_analysis::ProductionStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            },
        );
        visitor
            .values
            .into_iter()
            .filter(|literal_value| !literal_value.is_empty())
            .for_each(|literal_value| {
                literal_locations_by_value
                    .entry(literal_value)
                    .or_default()
                    .push(path_text.clone());
            });
    });
    let ers = literal_locations_by_value
        .into_iter()
        .filter(|(_, locations)| locations.len() > constants_usize::ONE)
        .map(|(literal_value, locations)| {
            format!("duplicated production string literal {literal_value:?} in {locations:?}")
        })
        .collect::<Vec<String>>();
    super::assert_joined_ers_empty_with_ctx(
        super::types::SourceTextListRef::from(ers.as_slice()),
        super::types::StaticStr::from(constants_str::VALUE_9D1C7E4A),
        super::types::SourceTextRef::from(
            constants_str::PRODUCTION_STRING_LITERALS_MUST_BE_DEFINED_ONCE_AND_REUSED,
        ),
    );
}
#[test]
#[allow(
    clippy::needless_for_each,
    clippy::useless_concat,
    reason = "the negative ownership policy must construct forbidden identifiers without declaring them in constants_str"
)]
fn domain_owned_string_catalogs_do_not_return_to_str_constants() {
    let source = std::fs::read_to_string(constants_str::STR_CONSTANTS_SRC_LIB_RS).expect(
        "84c15a0e domain_owned_string_catalogs_do_not_return_to_str_constants invariant must hold",
    );
    [
        concat!("ADMIN_SETTING_DEFAULT_ROUTE_LABEL"),
        concat!("ADMIN_DATABASE_ERROR_CODE"),
        concat!("NOTIFICATION_PERSISTENCE_ERROR_CODE"),
        concat!("SERVER_ADMIN_DB_SCHEMA_VALUE_"),
    ]
    .into_iter()
    .for_each(|identifier| assert!(!source.contains(identifier), "{identifier}"));
}

#[test]
fn server_admin_string_constants_reuse_macro_fragments() {
    let source = std::fs::read_to_string(constants_str::STR_CONSTANTS_SRC_LIB_RS)
        .expect("4629edbb server_admin_string_constants_reuse_macro_fragments invariant must hold");
    assert!(!source.contains("pub const SERVER_ADMIN_"));
}

#[test]
fn str_constants_does_not_own_typed_domain_values() {
    let source = std::fs::read_to_string(constants_str::STR_CONSTANTS_SRC_LIB_RS)
        .expect("3caa56a9 str_constants_does_not_own_typed_domain_values invariant must hold");
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
        super::types::StaticStr::from(constants_str::VALUE_6B7E02A4),
        super::types::SourceTextRef::from(
            constants_str::DOMAIN_VALUES_MUST_BE_DECLARED_BY_THEIR_OWNING_TYPED_API,
        ),
    );
}
#[test]
fn string_constant_visitor_checks_test_code_and_allows_reviewed_syntax_boundaries() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRING_GUARD_ALLOWED_SYNTAX_FIXTURE)
        .expect("87c9a142 string_constant_visitor_allows_only_reviewed_syntax_boundaries invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), constants_usize::TWO);
}
#[test]
fn string_constant_visitor_detects_expression_and_nested_macro_literals() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRING_GUARD_DETECTION_FIXTURE)
        .expect("bc91574f string_constant_visitor_detects_expression_and_nested_macro_literals invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 2usize);
}
#[test]
fn all_string_constants_are_declared_in_str_constants() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::VALUE_6C2711FA),
        super::types::SourceTextRef::from(
            constants_str::STRING_CONSTANTS_FOUND_OUTSIDE_STR_CONSTANTS,
        ),
        |path, ast, ers| {
            if super::is_str_constants_source_path(super::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::StringConstantVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
            let declaration_visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::StringConstantDeclarationVisitor {
                    allow_generated_string_constants: super::types::AnalyzerBool::from(
                        path.ends_with(
                            constants_str::CONSTANTS_STR_MACROS_SRC_DEFINE_STR_CONSTANTS_INPUT_RS,
                        ),
                    ),
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                declaration_visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn string_constant_policy_has_only_the_constants_crate_source_directory_exception() {
    assert!(
        super::is_str_constants_source_path(super::types::PathRef::from(std::path::Path::new(
            constants_str::STR_CONSTANTS_SRC_LIB_RS,
        )))
        .get()
    );
    assert!(
        [
            "../copy/constants_str/src/lib.rs",
            "constants_str/src/lib.rs",
        ]
        .into_iter()
        .all(|path| {
            !super::is_str_constants_source_path(super::types::PathRef::from(std::path::Path::new(
                path,
            )))
            .get()
        })
    );
    assert!(
        super::is_str_constants_source_path(super::types::PathRef::from(std::path::Path::new(
            "../constants_str/src/catalog.rs",
        )))
        .get()
    );
}
#[test]
fn string_constant_declaration_policy_ignores_runtime_literals_and_rejects_all_const_forms() {
    let ast = syn::parse_file(constants_str::CODE_STYLE_STRING_CONSTANT_DECLARATION_FIXTURE)
        .expect("02ec1d16 string_constant_declaration_policy_ignores_runtime_literals_and_rejects_all_const_forms invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantDeclarationVisitor {
            allow_generated_string_constants: super::types::AnalyzerBool::default(),
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 13usize);
}
#[test]
fn string_constant_declaration_policy_rejects_aliases_to_exported_constants() {
    let ast =
        syn::parse_file(constants_str::CODE_STYLE_STRING_CONSTANT_ALIAS_FIXTURE).expect("56f8e2c1 string_constant_declaration_policy_rejects_aliases_to_exported_constants invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::source_analysis::StringConstantDeclarationVisitor {
            allow_generated_string_constants: super::types::AnalyzerBool::default(),
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), constants_usize::ONE);
}
#[test]
fn no_unwrap_in_source_code() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::E8B3A6D2),
        super::types::SourceTextRef::from(constants_str::UNWRAP_FOUND),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::source_analysis::UnwrapVisitor {
                    found_count: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::PathRef::from(path),
                super::types::SourceTextRef::from(constants_str::UNWRAP_CALL_ALT),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn repository_identifiers_use_explicit_resource_names() {
    #[derive(Default, optimal_memory_layout::OptimalMemoryLayout)]
    struct ExplicitResourceNameVisitor {
        violations: super::types::SourceTextList,
    }

    impl<'ast_lt> syn::visit::Visit<'ast_lt> for ExplicitResourceNameVisitor {
        fn visit_ident(&mut self, i: &'ast_lt syn::Ident) {
            let name = i.to_string();
            let Some(vague_fragment) = stringify!(JoinHandle).strip_prefix(stringify!(Join)) else {
                return;
            };
            if name.to_ascii_lowercase().contains(vague_fragment)
                && !matches!(
                    name.as_str(),
                    stringify!(Handler)
                        | stringify!(JoinHandle)
                        | stringify!(PrometheusHandle)
                        | stringify!(ScopedJoinHandle)
                        | stringify!(handle)
                        | stringify!(handler)
                )
            {
                self.violations.push(name);
            }
            syn::visit::visit_ident(self, i);
        }
    }

    super::code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|file| {
                let mut visitor = ExplicitResourceNameVisitor::default();
                syn::visit::Visit::visit_file(&mut visitor, file.ast().as_ref());
                visitor
                    .violations
                    .into_iter()
                    .map(|identifier| format!("{}: {identifier}", file.path().as_ref().display()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "repository identifiers must name their concrete resource or endpoint role:\n{}",
            violations.join("\n")
        );
    });
}
