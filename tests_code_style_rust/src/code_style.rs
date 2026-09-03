#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum RustOrClippy {
    Clippy,
    Rust,
}
impl RustOrClippy {
    pub(crate) fn name(self) -> crate::types::StaticStr {
        match self {
            Self::Rust => crate::types::StaticStr::from(constants_str::RUST),
            Self::Clippy => crate::types::StaticStr::from(constants_str::CLIPPY),
        }
    }
}
pub(crate) fn declared_children() -> &'static std::collections::BTreeSet<(String, String)> {
    static DECLARED_CHILDREN: std::sync::OnceLock<std::collections::BTreeSet<(String, String)>> =
        std::sync::OnceLock::new();
    DECLARED_CHILDREN.get_or_init(|| {
        fn collect_rs_paths(path: &std::path::Path, vec: &mut Vec<std::path::PathBuf>) {
            let Ok(entries) = std::fs::read_dir(path) else {
                return;
            };
            entries.filter_map(Result::ok).for_each(|entry| {
                let entry_path = entry.path();
                match entry_path.is_dir() {
                    true => collect_rs_paths(entry_path.as_path(), vec),
                    false
                        if entry_path.extension().and_then(std::ffi::OsStr::to_str)
                            == Some(constants_str::RS) =>
                    {
                        vec.push(entry_path);
                    }
                    false => {}
                }
            });
        }
        let workspace_root = std::path::Path::new(constants_str::TEXT_ALT_9);
        let mut paths = Vec::new();
        if let Ok(entries) = std::fs::read_dir(workspace_root) {
            entries.filter_map(Result::ok).for_each(|entry| {
                let crate_root = entry.path();
                if crate_root
                    .join(constants_str::CARGO_TOML)
                    .is_file()
                {
                    collect_rs_paths(
                        crate_root.join(constants_str::SRC_ALT).as_path(),
                        &mut paths,
                    );
                }
            });
        }
        let mut declarations = std::collections::BTreeSet::new();
        while let Some(owner_path) = paths.pop() {
            let Ok(content) = std::fs::read_to_string(owner_path.as_path()) else {
                continue;
            };
            let Ok(ast) = syn::parse_file(content.as_str()) else {
                continue;
            };
            let Ok(owner_rel) = owner_path.strip_prefix(workspace_root) else {
                continue;
            };
            let Some(parent) = owner_rel.parent() else {
                continue;
            };
            #[allow(
                clippy::items_after_statements,
                clippy::needless_for_each,
                reason = "recursive compatibility-module discovery stays beside the parsed owner it processes"
            )]
            fn collect_modules(
                items: &[syn::Item],
                owner_rel: &std::path::Path,
                parent: &std::path::Path,
                workspace_root: &std::path::Path,
                b_tree_set: &mut std::collections::BTreeSet<(String, String)>,
            ) {
                items.iter().for_each(|item| {
                    let syn::Item::Mod(item_mod) = item else {
                        return;
                    };
                    let child_rel = parent
                        .join(item_mod.ident.to_string())
                        .with_extension(constants_str::RS);
                    if workspace_root.join(child_rel.as_path()).is_file() {
                        let _inserted = b_tree_set.insert((
                            owner_rel.to_string_lossy().into_owned(),
                            child_rel.to_string_lossy().into_owned(),
                        ));
                    }
                    item_mod.attrs.iter().for_each(|attr| {
                        if !attr.path().is_ident(constants_str::PATH_ALT_5) {
                            return;
                        }
                        let syn::Meta::NameValue(name_value) = &attr.meta else {
                            return;
                        };
                        let syn::Expr::Lit(expr_lit) = &name_value.value else {
                            return;
                        };
                        let syn::Lit::Str(path_lit) = &expr_lit.lit else {
                            return;
                        };
                        let _inserted = b_tree_set.insert((
                            owner_rel.to_string_lossy().into_owned(),
                            parent.join(path_lit.value()).to_string_lossy().into_owned(),
                        ));
                    });
                    if let Some((_brace, nested_items)) = &item_mod.content {
                        collect_modules(
                            nested_items,
                            owner_rel,
                            parent,
                            workspace_root,
                            b_tree_set,
                        );
                    }
                });
            }
            collect_modules(
                ast.items.as_slice(),
                owner_rel,
                parent,
                workspace_root,
                &mut declarations,
            );
        }
        declarations
    })
}

#[allow(
    clippy::single_call_fn,
    reason = "runtime-source classification keeps test-only child detection independently testable"
)]
pub(crate) fn is_cfg_test_declared_child(path: &std::path::Path) -> bool {
    let Some(file_name) = path.file_name() else {
        return false;
    };
    let Some(parent) = path.parent() else {
        return false;
    };
    [
        parent
            .join(constants_str::LIB)
            .with_extension(constants_str::RS),
        parent
            .join(constants_str::MAIN)
            .with_extension(constants_str::RS),
    ]
    .iter()
    .filter_map(|owner_path| std::fs::read_to_string(owner_path).ok())
    .filter_map(|content| syn::parse_file(content.as_str()).ok())
    .flat_map(|ast| ast.items.into_iter())
    .any(|item| {
        let syn::Item::Mod(item_mod) = item else {
            return false;
        };
        let declared_file_name = std::path::Path::new(item_mod.ident.to_string().as_str())
            .with_extension(constants_str::RS);
        declared_file_name.file_name() == Some(file_name)
            && item_mod.attrs.iter().any(|attr| {
                attr.path().is_ident(constants_str::CFG_ALT)
                    && matches!(
                        &attr.meta,
                        syn::Meta::List(list)
                            if list.tokens.to_string().contains(constants_str::TEST_ALT_3)
                    )
            })
    })
}
pub(crate) fn declared_child_matches(path: &str, owner: &str) -> bool {
    let owner_rel = owner
        .trim_start_matches(constants_str::TEXT_ALT_9)
        .trim_start_matches('/');
    let path_rel = path
        .trim_start_matches(constants_str::TEXT_ALT_9)
        .trim_start_matches('/');
    declared_children().contains(&(owner_rel.to_owned(), path_rel.to_owned())) || {
        let owner_path = std::path::Path::new(owner_rel);
        let child_path = std::path::Path::new(path_rel);
        owner_path.file_stem().and_then(std::ffi::OsStr::to_str)
            == Some(constants_str::DOMAIN_TYPES)
            && owner_path.parent() == child_path.parent()
    }
}
pub(crate) fn unowned_spawn_expr(expr: &syn::Expr) -> bool {
    let syn::Expr::Call(call) = expr else {
        return false;
    };
    let Some(path) = expr_call_path(crate::types::SynExprCallRef::from(call)) else {
        return false;
    };
    let text = path_to_string(path);
    if matches!(
        text.as_ref(),
        constants_str::VALUE_D90EE9CC
            | constants_str::VALUE_AA7752E0
            | constants_str::VALUE_F3FCC9F8
    ) {
        return call.args.first().is_some_and(unowned_spawn_expr);
    }
    matches!(
        text.as_ref(),
        constants_str::TOKIO_PATH_SPAWN
            | constants_str::TOKIO_PATH_TASK_PATH_SPAWN_BLOCKING
            | constants_str::TOKIO_PATH_TASK_PATH_SPAWN_LOCAL
            | constants_str::STD_PATH_THREAD_PATH_SPAWN
    )
}
pub(crate) fn panic_uses_dynamic_diagnostic_id(
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        source_text_ref
            .as_ref()
            .starts_with(constants_str::TEXT_ALT_14)
            || source_text_ref
                .as_ref()
                .starts_with(constants_str::VALUE_81766C62)
            || source_text_ref
                .as_ref()
                .starts_with(constants_str::VALUE_D8C45567)
            || source_text_ref
                .as_ref()
                .starts_with(constants_str::VALUE_9C7DD42A),
    )
}
pub(crate) fn macro_path_is_quote(
    syn_path_ref: crate::types::SynPathRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        syn_path_ref
            .as_ref()
            .segments
            .last()
            .is_some_and(|segment| {
                matches!(
                    segment.ident.to_string().as_str(),
                    constants_str::SHARED_VALUES_QUOTE | constants_str::SHARED_VALUES_QUOTE_SPANNED
                )
            }),
    )
}
pub(crate) fn scan_generated_diagnostic_tokens(
    token_stream: &proc_macro2::TokenStream,
    diagnostic_id_visitor: &mut crate::source_analysis::DiagnosticIdVisitor,
) {
    let trees = token_stream.clone().into_iter().collect::<Vec<_>>();
    trees.iter().enumerate().for_each(|(index, token)| {
        if let proc_macro2::TokenTree::Group(group) = token {
            scan_generated_diagnostic_tokens(&group.stream(), diagnostic_id_visitor);
        }
        let proc_macro2::TokenTree::Ident(identifier) = token else {
            return;
        };
        let identifier_text = identifier.to_string();
        let is_expect = identifier_text == constants_str::CODE_STYLE_EXPECT_METHOD_NAME
            && index.checked_sub(constants_usize::ONE).and_then(|previous| trees.get(previous)).is_some_and(
                |previous| matches!(previous, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '.'),
            );
        let is_panic = identifier_text == constants_str::CODE_STYLE_PANIC_METHOD_NAME
            && trees
                .get(index.saturating_add(constants_usize::ONE))
                .is_some_and(|next| matches!(next, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '!'));
        if !is_expect && !is_panic {
            return;
        }
        let group_index = index.saturating_add(if is_panic { 2usize } else { constants_usize::ONE });
        let Some(proc_macro2::TokenTree::Group(arguments)) = trees.get(group_index) else {
            diagnostic_id_visitor
                .get_errors_mut()
                .push(format!("generated `{identifier_text}` has no argument group"));
            return;
        };
        let argument_tokens = arguments.stream().into_iter().collect::<Vec<_>>();
        let interpolated_identifier = match argument_tokens.as_slice() {
            [
                proc_macro2::TokenTree::Punct(interpolation),
                proc_macro2::TokenTree::Ident(interpolated),
                ..
            ] if interpolation.as_char() == '#' => Some(interpolated.to_string()),
            _ => None,
        };
        let message = argument_tokens.first().and_then(|first| {
            let proc_macro2::TokenTree::Literal(message_literal) = first else {
                return None;
            };
            syn::parse_str::<syn::LitStr>(message_literal.to_string().as_str())
                .ok()
                .map(|parsed_literal| parsed_literal.value())
        });
        match message {
            Some(message_value)
                if is_panic
                    && panic_uses_dynamic_diagnostic_id(crate::types::SourceTextRef::from(
                        message_value.as_str(),
                    ))
                    .get() => {}
            Some(message_value) => diagnostic_id_visitor.record(
                crate::types::SourceTextRef::from(identifier_text.as_str()),
                crate::types::SourceTextRef::from(message_value.as_str()),
            ),
            None => match interpolated_identifier {
                Some(interpolated) => diagnostic_id_visitor.get_errors_mut().push(format!(
                    "generated `{identifier_text}` uses unchecked interpolated diagnostic message `#{interpolated}`"
                )),
                None => diagnostic_id_visitor.get_errors_mut().push(format!(
                    "generated `{identifier_text}` message must begin with a string literal"
                )),
            },
        }
    });
}

pub(crate) fn assert_workspace_lints_match(
    rust_or_clippy: RustOrClippy,
    tool: crate::types::StaticStr,
    analyzer_bool: crate::types::AnalyzerBool,
    exp_id: crate::types::StaticStr,
    static_str_slice_ref: crate::types::StaticStrSliceRef<'_>,
) {
    let workspace = workspace_table_from_cargo_toml();
    let lints = toml_val_as_table_ref(
        crate::types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(constants_str::LINTS)
                .expect(constants_str::DIAGNOSTIC_82EAEA37),
        ),
        crate::types::StaticStr::from(constants_str::CAE226CD),
    );
    let lint_table = toml_val_as_table_ref(
        crate::types::TomlValueRef::from(
            lints
                .as_ref()
                .get(rust_or_clippy.name().get())
                .expect(constants_str::DIAGNOSTIC_DBD02F72),
        ),
        crate::types::StaticStr::from(constants_str::VALUE_6F4580CE),
    );
    let cargo_lints = crate::types::SourceTextList::from(
        lint_table.as_ref().keys().cloned().collect::<Vec<String>>(),
    );
    let output = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(tool.get()),
    )
    .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
        [constants_str::W, constants_str::HELP].as_slice(),
    ))
    .output()
    .unwrap_or_else(|_| std::panic::panic_any(exp_id.get().to_owned()));
    assert!(output.status.success(), "95d4595a");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.trim().is_empty(), "cc4670a2");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let pattern = if analyzer_bool.get() {
        regex::Regex::new(constants_str::QUESTION_M_S_ASTERISK_CLIPPY_PATH_A_Z0_9_A_Z0_9)
            .expect(constants_str::DIAGNOSTIC_FBF14346)
    } else {
        regex::Regex::new(constants_str::QUESTION_M_S_ASTERISK_A_Z0_9_A_Z0_9_PLUS_S)
            .expect(constants_str::DIAGNOSTIC_60D99C87)
    };
    let command_lints = crate::types::SourceTextList::from(
        pattern
            .captures_iter(&stdout)
            .map(|captures| {
                String::from(
                    crate::types::SourceText::try_from(
                        captures[1].replace('-', constants_str::UNDERSCORE),
                    )
                    .expect(constants_str::DIAGNOSTIC_F3D821A6),
                )
            })
            .collect::<Vec<String>>(),
    );
    let rust_or_clippy_name = rust_or_clippy.name().get();
    let lints_vec_from_cargo_toml = crate::types::SourceTextListRef::from(cargo_lints.as_slice());
    let lints_from_cmd = crate::types::SourceTextListRef::from(command_lints.as_slice());
    let lints_from_cargo_set = str_set(lints_vec_from_cargo_toml);
    let lints_from_cmd_set = str_set(lints_from_cmd);
    let lints_exceptions_set = static_str_slice_ref
        .get()
        .iter()
        .copied()
        .collect::<std::collections::HashSet<&str>>();
    let stale_exceptions = static_str_slice_ref
        .get()
        .iter()
        .copied()
        .filter(|lint| !lints_from_cmd_set.as_ref().contains(*lint))
        .collect::<Vec<&str>>();
    assert!(stale_exceptions.is_empty(), "31c5955d {stale_exceptions:?}");
    let (_reviewed_missing_lints, lints_not_in_cargo_toml) = collect_missing_items(
        lints_from_cmd,
        crate::types::SourceTextRefHashSet::from(lints_from_cargo_set.as_ref()),
    )
    .into_iter()
    .partition::<Vec<String>, _>(|lint| lints_exceptions_set.contains(lint.as_str()));
    assert!(
        lints_not_in_cargo_toml.is_empty(),
        "d2b7ba9f {rust_or_clippy_name} {lints_not_in_cargo_toml:?}"
    );
    let outdated_lints_in_file = collect_missing_items(
        lints_vec_from_cargo_toml,
        crate::types::SourceTextRefHashSet::from(lints_from_cmd_set.as_ref()),
    );
    assert!(outdated_lints_in_file.is_empty(), "93787d2d");
}

pub(crate) fn validate_workspace_dep_default_features(
    toml_table_ref: crate::types::TomlTableRef<'_>,
) {
    match toml_table_ref
        .get()
        .get(constants_str::DEFAULT_FEATURES)
        .expect(constants_str::DIAGNOSTIC_D2A8C4E1)
    {
        &toml::Value::Boolean(false) => (),
        &toml::Value::Boolean(true) => std::panic::panic_any(constants_str::PANIC_847A138F),
        &toml::Value::String(_)
        | &toml::Value::Table(_)
        | &toml::Value::Integer(_)
        | &toml::Value::Float(_)
        | &toml::Value::Datetime(_)
        | &toml::Value::Array(_) => std::panic::panic_any(constants_str::PANIC_E5F7B1C3),
    }
}
pub(crate) fn workspace_dep_disables_default_features(
    toml_value_ref: crate::types::TomlValueRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        toml_value_ref
            .as_ref()
            .as_table()
            .and_then(|table| table.get(constants_str::DEFAULT_FEATURES))
            == Some(&toml::Value::Boolean(false)),
    )
}
pub(crate) fn unjustified_workspace_lint_allows(
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::DiagnosticMessages {
    let mut in_workspace_lints = false;
    crate::types::DiagnosticMessages::from(
        source_text_ref
            .as_ref()
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let trimmed = line.trim();
                if trimmed.starts_with('[') {
                    in_workspace_lints = matches!(
                        trimmed,
                        constants_str::VALUE_AC763BA9 | constants_str::VALUE_EA8957C1
                    );
                    return None;
                }
                if !in_workspace_lints {
                    return None;
                }
                let (setting, comment) = line
                    .split_once('#')
                    .map_or((line, None), |(setting, comment)| (setting, Some(comment)));
                (setting.trim_end().ends_with(constants_str::VALUE_9F4AF5DD)
                    && comment.is_none_or(|reason| reason.trim().is_empty()))
                .then(|| {
                    format!(
                        "line {}: {}",
                        index.saturating_add(constants_usize::ONE),
                        trimmed
                    )
                })
            })
            .collect::<Vec<String>>(),
    )
}
pub(crate) fn commented_debug_statements(
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::DiagnosticMessages {
    crate::types::DiagnosticMessages::from(
        source_text_ref
            .as_ref()
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let comment = line
                    .trim_start()
                    .strip_prefix(constants_str::VALUE_A2C23396)?
                    .trim_start();
                [
                    constants_str::VALUE_EB2E6B1F,
                    constants_str::VALUE_0981EB3C,
                    constants_str::VALUE_2FFB2CC3,
                    constants_str::VALUE_04CB3FD5,
                    constants_str::VALUE_2933C4F3,
                ]
                .into_iter()
                .any(|macro_name| comment.starts_with(macro_name))
                .then(|| {
                    format!(
                        "line {}: {}",
                        index.saturating_add(constants_usize::ONE),
                        line.trim()
                    )
                })
            })
            .collect::<Vec<String>>(),
    )
}
pub(crate) fn text_content_hygiene_errors(
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::DiagnosticMessages {
    let mut errors = crate::types::DiagnosticMessages::default();
    if !source_text_ref.as_ref().is_empty() && !source_text_ref.as_ref().ends_with('\n') {
        errors.push(constants_str::VALUE_C2BE29D9.to_owned());
    }
    if source_text_ref.as_ref().contains('\r') {
        errors.push(constants_str::VALUE_A8C54D74.to_owned());
    }
    source_text_ref
        .as_ref()
        .lines()
        .enumerate()
        .filter(|(_, line)| line.ends_with([' ', '\t']))
        .for_each(|(index, _)| {
            errors.push(format!(
                "line {} contains trailing whitespace",
                index.saturating_add(constants_usize::ONE)
            ));
        });
    errors
}

pub(crate) fn macro_rules_errors(
    path_ref: crate::types::PathRef<'_>,
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::DiagnosticMessages {
    let forbidden = format!("{}!", constants_str::MACRO_RULES);
    if source_text_ref.as_ref().contains(forbidden.as_str()) {
        let mut error = path_ref.as_ref().display().to_string();
        error.push_str(constants_str::PATH_ERROR_SEPARATOR);
        error.push_str(
            constants_str::MACRO_RULES_FOUND_USE_WORKSPACE_PROC_MACRO_CRATES_INSTEAD
                .trim_end_matches(':'),
        );
        vec![error].into()
    } else {
        crate::types::DiagnosticMessages::default()
    }
}

pub(crate) fn env_keys_from_file(
    static_str: crate::types::StaticStr,
) -> crate::types::SourceTextList {
    std::fs::read_to_string(static_str.get())
        .expect(constants_str::DIAGNOSTIC_B3A7C1E4)
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            (!trimmed.is_empty() && !trimmed.starts_with('#'))
                .then(|| trimmed.split_once('=').map(|(key, _)| key))
                .flatten()
        })
        .map(str::to_owned)
        .collect::<Vec<String>>()
        .into()
}
pub(crate) fn collect_missing_items(
    source_text_list_ref: crate::types::SourceTextListRef<'_>,
    source_text_ref_hash_set: crate::types::SourceTextRefHashSet<'_>,
) -> crate::types::SourceTextList {
    crate::types::SourceTextList::from(
        source_text_list_ref
            .get()
            .iter()
            .map(String::as_str)
            .filter(|item| !source_text_ref_hash_set.as_ref().contains(item))
            .map(str::to_owned)
            .collect::<Vec<String>>(),
    )
}
pub(crate) fn collect_missing_key_errors(
    source_text_list_ref: crate::types::SourceTextListRef<'_>,
    source_text_ref_hash_set: crate::types::SourceTextRefHashSet<'_>,
    source_file: crate::types::StaticStr,
    target_file: crate::types::StaticStr,
) -> crate::types::SourceTextList {
    crate::types::SourceTextList::from(
        collect_missing_items(source_text_list_ref, source_text_ref_hash_set)
            .into_iter()
            .map(|key| {
                format!(
                    "key `{key}` in {} but missing from {}",
                    source_file.get(),
                    target_file.get()
                )
            })
            .collect::<Vec<String>>(),
    )
}

pub(crate) fn assert_cargo_toml_errors_empty(
    static_str: crate::types::StaticStr,
    mut make_errors: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) {
    let mut raw_errors = Vec::new();
    for_each_crate_manifest_file(|path| {
        let Some(parsed) = read_toml_table(crate::types::PathRef::from(path)) else {
            return;
        };
        make_errors(path, parsed.as_ref(), &mut raw_errors);
    });
    let errors = crate::types::SourceTextList::from(raw_errors);
    assert_joined_errors_empty(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        static_str,
    );
}
pub(crate) fn assert_crate_manifest_cargo_policy(
    static_str: crate::types::StaticStr,
    mut make_errors: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) {
    assert_cargo_toml_errors_empty(static_str, |path, parsed, errors| {
        make_errors(path, parsed, errors);
    });
}
pub(crate) fn assert_joined_errors_empty(
    source_text_list_ref: crate::types::SourceTextListRef<'_>,
    static_str: crate::types::StaticStr,
) {
    assert_joined_errors_empty_with_context(
        source_text_list_ref,
        static_str,
        crate::types::SourceTextRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
    );
}
pub(crate) fn assert_joined_errors_empty_with_context(
    source_text_list_ref: crate::types::SourceTextListRef<'_>,
    static_str: crate::types::StaticStr,
    source_text_ref: crate::types::SourceTextRef<'_>,
) {
    if source_text_ref.as_ref().is_empty() {
        assert!(
            source_text_list_ref.as_ref().is_empty(),
            "{}\n{}",
            static_str.get(),
            source_text_list_ref.as_ref().join("\n")
        );
    } else {
        assert!(
            source_text_list_ref.as_ref().is_empty(),
            "{} {}\n{}",
            static_str.get(),
            source_text_ref.as_ref(),
            source_text_list_ref.as_ref().join("\n")
        );
    }
}
pub(crate) fn assert_joined_errors_empty_sorted(
    mut diagnostic_messages_mut_ref: crate::types::DiagnosticMessagesMutRef<'_>,
    static_str: crate::types::StaticStr,
) {
    diagnostic_messages_mut_ref.sort();
    assert_joined_errors_empty(
        crate::types::SourceTextListRef::from(diagnostic_messages_mut_ref.as_slice()),
        static_str,
    );
}
pub(crate) fn str_set(
    source_text_list_ref: crate::types::SourceTextListRef<'_>,
) -> crate::types::SourceTextHashSet<'_> {
    crate::types::SourceTextHashSet::from(
        source_text_list_ref
            .get()
            .iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<&str>>(),
    )
}
pub(crate) fn visit_syn_file<V>(syn_file_ref: crate::types::SynFileRef<'_>, mut v: V) -> V
where
    V: for<'ast> syn::visit::Visit<'ast>,
{
    syn::visit::Visit::visit_file(&mut v, syn_file_ref.as_ref());
    v
}
pub(crate) fn assert_rs_ast_errors_empty_with_context(
    static_str: crate::types::StaticStr,
    source_text_ref: crate::types::SourceTextRef<'_>,
    mut make_errors: impl FnMut(&std::path::Path, &syn::File, &mut Vec<String>),
) {
    let mut raw_errors = Vec::new();
    for_each_rs_file(|file| {
        make_errors(file.path().as_ref(), file.ast().as_ref(), &mut raw_errors);
    });
    let errors = crate::types::SourceTextList::from(raw_errors);
    assert_joined_errors_empty_with_context(
        crate::types::SourceTextListRef::from(errors.as_slice()),
        static_str,
        source_text_ref,
    );
}
pub(crate) fn read_toml_table(
    path_ref: crate::types::PathRef<'_>,
) -> Option<crate::types::TomlTable> {
    crate::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.read_toml_table(path_ref)
    })
}

#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn cargo_toml_content(
    path_ref: crate::types::PathRef<'_>,
) -> Option<crate::types::SourceText> {
    crate::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.cargo_toml_content(path_ref)
    })
}
pub(crate) fn push_repeated_file_error(
    mut diagnostic_messages_mut_ref: crate::types::DiagnosticMessagesMutRef<'_>,
    path_ref: crate::types::PathRef<'_>,
    source_text_ref: crate::types::SourceTextRef<'_>,
    analyzer_count: crate::types::AnalyzerCount,
) {
    diagnostic_messages_mut_ref.extend(
        std::iter::repeat_with(|| {
            format!(
                "{}: {}",
                path_ref.as_ref().display(),
                source_text_ref.as_ref()
            )
        })
        .take(analyzer_count.get()),
    );
}
pub(crate) fn workspace_crate_names() -> crate::types::SourceTextBTreeSet {
    crate::test_code_style_snapshot::with_codebase_snapshot(
        crate::test_code_style_snapshot::CodebaseSnapshot::workspace_crate_names,
    )
}

pub(crate) fn for_each_crate_manifest_file(on_file: impl FnMut(&std::path::Path)) {
    crate::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.crate_manifest_paths().for_each(on_file);
    });
}
pub(crate) fn path_has_segment(
    syn_path_ref: crate::types::SynPathRef<'_>,
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        syn_path_ref
            .as_ref()
            .segments
            .iter()
            .any(|element| element.ident == source_text_ref.as_ref()),
    )
}

pub(crate) fn item_impl_self_ty_identifier(
    syn_item_impl_ref: crate::types::SynItemImplRef<'_>,
) -> Option<crate::types::SourceText> {
    match syn_item_impl_ref.as_ref().self_ty.as_ref() {
        syn::Type::Path(ty_path) => ty_path.path.segments.last().map(|segment| {
            crate::types::SourceText::try_from(segment.ident.to_string())
                .expect(constants_str::DIAGNOSTIC_6A9F03D2)
        }),
        syn::Type::Array(_)
        | syn::Type::FnPtr(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => None,
    }
}
pub(crate) fn from_trait_arg_is_string(
    syn_path_ref: crate::types::SynPathRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(syn_path_ref.as_ref().segments.last().is_some_and(|segment| {
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        args.args.iter().any(|arg| {
                            matches!(arg, syn::GenericArgument::Type(ty) if type_path_ends_with_identifier(crate::types::SynTypeRef::from(ty), crate::types::SourceTextRef::from(constants_str::STRING)).get())
                        })
                    }
                    syn::PathArguments::Parenthesized(_) | syn::PathArguments::None => false,
                }
            }))
}
pub(crate) fn item_struct_is_single_string_wrapper(
    syn_item_struct_ref: crate::types::SynItemStructRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(match &syn_item_struct_ref.as_ref().fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            fields.unnamed.first().is_some_and(|field| {
                [constants_str::STRING, constants_str::BOUNDEDSTRING]
                    .iter()
                    .any(|identifier| {
                        type_path_ends_with_identifier(
                            crate::types::SynTypeRef::from(&field.ty),
                            crate::types::SourceTextRef::from(*identifier),
                        )
                        .get()
                    })
            })
        }
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
    })
}
pub(crate) fn item_struct_is_single_field_tuple_wrapper(
    syn_item_struct_ref: crate::types::SynItemStructRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        matches!(&syn_item_struct_ref.as_ref().fields, syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1),
    )
}

pub(crate) fn item_impl_input_type_is(
    syn_item_impl_ref: crate::types::SynItemImplRef<'_>,
    ty: &syn::Type,
) -> crate::types::AnalyzerBool {
    let source_type = syn_item_impl_ref
        .as_ref()
        .trait_
        .as_ref()
        .and_then(|(path, _)| {
            let segment = path.segments.last()?;
            let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };
            let argument = arguments.args.first()?;
            let syn::GenericArgument::Type(value) = argument else {
                return None;
            };
            Some(value)
        });
    crate::types::AnalyzerBool::from(source_type.is_some_and(|input_type| input_type == ty))
}
pub(crate) fn item_impl_is_from_or_try_from(
    syn_item_impl_ref: crate::types::SynItemImplRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(syn_item_impl_ref.as_ref().trait_.as_ref().is_some_and(
        |(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::FROM_ALT_3
                    || segment.ident == constants_str::TRYFROM
            })
        },
    ))
}
pub(crate) fn method_is_explicit_wrapper_accessor(
    syn_identifier_ref: crate::types::SynIdentifierRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(matches!(
        syn_identifier_ref.as_ref().to_string().as_str(),
        constants_str::GET_ALT | constants_str::INTO_INNER
    ))
}
pub(crate) fn type_path_ends_with_identifier(
    syn_type_ref: crate::types::SynTypeRef<'_>,
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(match syn_type_ref.as_ref() {
        syn::Type::Path(ty_path) => ty_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == source_text_ref.as_ref()),
        syn::Type::Array(_)
        | syn::Type::FnPtr(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}

pub(crate) fn path_ends_with(
    syn_path_ref: crate::types::SynPathRef<'_>,
    static_str_slice_ref: crate::types::StaticStrSliceRef<'_>,
) -> crate::types::AnalyzerBool {
    let path_ref = syn_path_ref.as_ref();
    crate::types::AnalyzerBool::from(
        path_ref.segments.len() >= static_str_slice_ref.get().len()
            && path_ref
                .segments
                .iter()
                .rev()
                .zip(static_str_slice_ref.get().iter().rev())
                .all(|(got, exp)| got.ident == *exp),
    )
}
pub(crate) fn expr_call_path(
    syn_expr_call_ref: crate::types::SynExprCallRef<'_>,
) -> Option<crate::types::SynPathRef<'_>> {
    match syn_expr_call_ref.get().func.as_ref() {
        syn::Expr::Path(path) => Some(crate::types::SynPathRef::from(&path.path)),
        syn::Expr::Array(_)
        | syn::Expr::Assign(_)
        | syn::Expr::Async(_)
        | syn::Expr::Await(_)
        | syn::Expr::Binary(_)
        | syn::Expr::Block(_)
        | syn::Expr::Break(_)
        | syn::Expr::Call(_)
        | syn::Expr::Cast(_)
        | syn::Expr::Closure(_)
        | syn::Expr::Const(_)
        | syn::Expr::Continue(_)
        | syn::Expr::Field(_)
        | syn::Expr::ForLoop(_)
        | syn::Expr::Group(_)
        | syn::Expr::If(_)
        | syn::Expr::Index(_)
        | syn::Expr::Infer(_)
        | syn::Expr::Let(_)
        | syn::Expr::Lit(_)
        | syn::Expr::Loop(_)
        | syn::Expr::Macro(_)
        | syn::Expr::Match(_)
        | syn::Expr::MethodCall(_)
        | syn::Expr::Paren(_)
        | syn::Expr::Range(_)
        | syn::Expr::RawAddr(_)
        | syn::Expr::Reference(_)
        | syn::Expr::Repeat(_)
        | syn::Expr::Return(_)
        | syn::Expr::Struct(_)
        | syn::Expr::Try(_)
        | syn::Expr::TryBlock(_)
        | syn::Expr::Tuple(_)
        | syn::Expr::Unary(_)
        | syn::Expr::Unsafe(_)
        | syn::Expr::Verbatim(_)
        | syn::Expr::While(_)
        | syn::Expr::Yield(_)
        | _ => None,
    }
}

pub(crate) fn collect_first_macro_identifier_domain_name(
    source_text_ref: crate::types::SourceTextRef<'_>,
    source_text_b_tree_set: &mut crate::types::SourceTextBTreeSet,
) {
    let re = regex::Regex::new(constants_str::S_ASTERISK_A_ZA_Z_A_ZA_Z0_9_ASTERISK_S_ASTERISK)
        .expect(constants_str::DIAGNOSTIC_FC65B7C4);
    if let Some(name) = re
        .captures(source_text_ref.as_ref())
        .and_then(|captures| captures.get(1))
        .map(|name| name.as_str())
    {
        let _: bool = source_text_b_tree_set.insert(name.to_owned());
    }
}
pub(crate) fn len_checked_function_names(
    syn_file_ref: crate::types::SynFileRef<'_>,
) -> crate::types::SourceTextBTreeSet {
    let mut visitor = crate::domain_analysis::LenCheckedFunctionNameVisitor::new(
        crate::types::SourceTextBTreeSet::default(),
    );
    syn::visit::Visit::visit_file(&mut visitor, syn_file_ref.as_ref());
    visitor.get_names().clone()
}
pub(crate) fn string_wrapper_names(
    syn_file_ref: crate::types::SynFileRef<'_>,
) -> crate::types::SourceTextBTreeSet {
    visit_syn_file(
        syn_file_ref,
        crate::domain_analysis::StringWrapperNameVisitor::new(
            crate::types::SourceTextBTreeSet::default(),
        ),
    )
    .get_names()
    .clone()
}
pub(crate) fn domain_type_policy_should_check_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    if path_ref
        .as_ref()
        .components()
        .any(|component| component.as_os_str() == constants_str::BENCHES)
        || path_ref
            .as_ref()
            .to_string_lossy()
            .trim_start_matches(constants_str::TEXT_ALT_9)
            .starts_with(constants_str::CODE_STYLE_MACRO_CLIPPY_CHECK_TEST_COMMON_SRC)
        || (path_ref.as_ref().exists() && is_test_crate_source_path(path_ref).get())
        || path_ref
            .as_ref()
            .starts_with(constants_str::CODE_STYLE_BOUNDED_TYPES_SRC)
        || path_ref
            .as_ref()
            .to_string_lossy()
            .starts_with(constants_str::FRONTEND_SRC_UI)
        || [
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_NAVIGATION_RS,
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_TABLE_RS,
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_ALERT_RS,
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_BADGE_RS,
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_BUTTON_RS,
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_CARD_RS,
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_FIELD_RS,
            constants_str::FRONTEND_SRC_DOMAIN_TYPES_WITH_OWNER_INPUT_RS,
            constants_str::FRONTEND_SRC_WITH_OWNER_RS,
            constants_str::FRONTEND_SRC_ADMIN_FIELD_LABEL_RS,
        ]
        .iter()
        .any(|owner| {
            path_ref.as_ref().ends_with(owner)
                || declared_child_matches(path_ref.as_ref().to_string_lossy().as_ref(), owner)
        })
    {
        return crate::types::AnalyzerBool::default();
    }
    let Some(cargo_toml_path) = nearest_cargo_toml_path(path_ref) else {
        return crate::types::AnalyzerBool::default();
    };
    crate::types::AnalyzerBool::from(cargo_toml_path.as_ref().is_file())
}
pub(crate) fn is_code_style_meta_harness_source_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        path_ref.as_ref().parent().is_some_and(|parent| {
            parent == std::path::Path::new(constants_str::CODE_STYLE_TESTS_SRC_ROOT)
        }) && path_ref.as_ref()
            != std::path::Path::new(constants_str::CODE_STYLE_DOMAIN_FIXTURE_PATH),
    )
}
pub(crate) fn analyzer_state_raw_container_ty(
    syn_type_ref: crate::types::SynTypeRef<'_>,
) -> Option<(crate::types::StaticStr, crate::types::StaticStr)> {
    match syn_type_ref.get() {
        syn::Type::Group(ty_group) => {
            analyzer_state_raw_container_ty(crate::types::SynTypeRef::from(&*ty_group.elem))
        }
        syn::Type::Paren(ty_paren) => {
            analyzer_state_raw_container_ty(crate::types::SynTypeRef::from(&*ty_paren.elem))
        }
        syn::Type::Path(ty_path) => {
            let segment = ty_path.path.segments.last()?;
            let identifier = segment.ident.to_string();
            match identifier.as_str() {
                constants_str::VEC
                    if single_angle_type_arg(crate::types::SynPathArgumentsRef::from(
                        &segment.arguments,
                    ))
                    .is_some_and(|ty| type_is_string(crate::types::SynTypeRef::from(ty.get())).get()) =>
                {
                    Some((
                        crate::types::StaticStr::from(constants_str::VEC_STRING),
                        crate::types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXTLIST),
                    ))
                }
                constants_str::BTREESET
                    if single_angle_type_arg(crate::types::SynPathArgumentsRef::from(
                        &segment.arguments,
                    ))
                    .is_some_and(|ty| type_is_string(crate::types::SynTypeRef::from(ty.get())).get()) =>
                {
                    Some((
                        crate::types::StaticStr::from(constants_str::BTREESET_STRING),
                        crate::types::StaticStr::from(constants_str::TYPES_PATH_STDSOURCETEXTSET),
                    ))
                }
                constants_str::HASHSET
                    if single_angle_type_arg(crate::types::SynPathArgumentsRef::from(
                        &segment.arguments,
                    ))
                    .is_some_and(|ty| type_is_str_ref(crate::types::SynTypeRef::from(ty.get())).get()) =>
                {
                    Some((
                        crate::types::StaticStr::from(constants_str::HASHSET_STR),
                        crate::types::StaticStr::from(
                            constants_str::TYPES_PATH_STDSOURCETEXTHASHSET_OR_TYPES_PATH_STDSOURCETEXTREFSET,
                        ),
                    ))
                }
                constants_str::OPTION
                | constants_str::RESULT
                | constants_str::BOX
                | constants_str::COW
                | constants_str::ARC
                | constants_str::RC
                | constants_str::PIN
                | constants_str::PHANTOMDATA
                | constants_str::HASHMAP
                | constants_str::BTREEMAP => match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        args.args.iter().find_map(|arg| match arg {
                            syn::GenericArgument::Type(ty) => {
                                analyzer_state_raw_container_ty(crate::types::SynTypeRef::from(ty))
                            }
                            syn::GenericArgument::AssocConst(_)
                            | syn::GenericArgument::AssocType(_)
                            | syn::GenericArgument::Constraint(_)
                            | syn::GenericArgument::Const(_)
                            | syn::GenericArgument::Lifetime(_)
                            | _ => None,
                        })
                    }
                    syn::PathArguments::Parenthesized(args) => args
                        .inputs
                        .iter()
                        .find_map(|arg| {
                            analyzer_state_raw_container_ty(crate::types::SynTypeRef::from(&arg.ty))
                        })
                        .or_else(|| match &args.output {
                            syn::ReturnType::Default => None,
                            syn::ReturnType::Type(_, ty) => {
                                analyzer_state_raw_container_ty(crate::types::SynTypeRef::from(&**ty))
                            }
                        }),
                    syn::PathArguments::None => None,
                },
                _ => None,
            }
        }
        syn::Type::Reference(ty_reference) => {
            analyzer_state_raw_container_ty(crate::types::SynTypeRef::from(&*ty_reference.elem))
        }
        syn::Type::Array(_)
        | syn::Type::FnPtr(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Ptr(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => None,
    }
}
pub(crate) fn raw_text_return_ty(
    syn_type_ref: crate::types::SynTypeRef<'_>,
) -> Option<(crate::types::StaticStr, crate::types::StaticStr)> {
    match syn_type_ref.get() {
        syn::Type::Group(ty_group) => {
            raw_text_return_ty(crate::types::SynTypeRef::from(&*ty_group.elem))
        }
        syn::Type::Paren(ty_paren) => {
            raw_text_return_ty(crate::types::SynTypeRef::from(&*ty_paren.elem))
        }
        syn::Type::Path(ty_path) => {
            let segment = ty_path.path.segments.last()?;
            let identifier = segment.ident.to_string();
            match identifier.as_str() {
                constants_str::STRING => Some((
                    crate::types::StaticStr::from(constants_str::STRING),
                    crate::types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXT),
                )),
                constants_str::VEC
                    if single_angle_type_arg(crate::types::SynPathArgumentsRef::from(
                        &segment.arguments,
                    ))
                    .is_some_and(|ty| {
                        type_is_string(crate::types::SynTypeRef::from(ty.get())).get()
                    }) =>
                {
                    Some((
                        crate::types::StaticStr::from(constants_str::VEC_STRING),
                        crate::types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXTLIST),
                    ))
                }
                constants_str::OPTION
                    if single_angle_type_arg(crate::types::SynPathArgumentsRef::from(
                        &segment.arguments,
                    ))
                    .is_some_and(|ty| {
                        type_is_str_ref(crate::types::SynTypeRef::from(ty.get())).get()
                    }) =>
                {
                    Some((
                        crate::types::StaticStr::from(constants_str::OPTION_STR),
                        crate::types::StaticStr::from(
                            constants_str::OPTION_TYPES_PATH_SOURCETEXTREF,
                        ),
                    ))
                }
                constants_str::OPTION
                | constants_str::RESULT
                | constants_str::BOX
                | constants_str::COW
                | constants_str::ARC
                | constants_str::RC
                | constants_str::PIN
                | constants_str::PHANTOMDATA
                | constants_str::HASHMAP
                | constants_str::BTREEMAP
                | constants_str::HASHSET
                | constants_str::BTREESET => match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        args.args.iter().find_map(|arg| match arg {
                            syn::GenericArgument::Type(ty) => {
                                raw_text_return_ty(crate::types::SynTypeRef::from(ty))
                            }
                            syn::GenericArgument::AssocConst(_)
                            | syn::GenericArgument::AssocType(_)
                            | syn::GenericArgument::Constraint(_)
                            | syn::GenericArgument::Const(_)
                            | syn::GenericArgument::Lifetime(_)
                            | _ => None,
                        })
                    }
                    syn::PathArguments::Parenthesized(args) => args
                        .inputs
                        .iter()
                        .find_map(|arg| raw_text_return_ty(crate::types::SynTypeRef::from(&arg.ty)))
                        .or_else(|| match &args.output {
                            syn::ReturnType::Default => None,
                            syn::ReturnType::Type(_, ty) => {
                                raw_text_return_ty(crate::types::SynTypeRef::from(&**ty))
                            }
                        }),
                    syn::PathArguments::None => None,
                },
                _ => None,
            }
        }
        syn::Type::Reference(_) if type_is_str_ref(syn_type_ref).get() => Some((
            crate::types::StaticStr::from(constants_str::STR),
            crate::types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXTREF),
        )),
        syn::Type::Reference(ty_reference) => {
            raw_text_return_ty(crate::types::SynTypeRef::from(&*ty_reference.elem))
        }
        syn::Type::Array(_)
        | syn::Type::FnPtr(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Ptr(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => None,
    }
}
pub(crate) fn single_angle_type_arg(
    syn_path_arguments_ref: crate::types::SynPathArgumentsRef<'_>,
) -> Option<crate::types::SynTypeRef<'_>> {
    let syn::PathArguments::AngleBracketed(args) = syn_path_arguments_ref.get() else {
        return None;
    };
    let mut type_args = args.args.iter().filter_map(|arg| match arg {
        syn::GenericArgument::Type(ty) => Some(crate::types::SynTypeRef::from(ty)),
        syn::GenericArgument::AssocConst(_)
        | syn::GenericArgument::AssocType(_)
        | syn::GenericArgument::Constraint(_)
        | syn::GenericArgument::Const(_)
        | syn::GenericArgument::Lifetime(_)
        | _ => None,
    });
    let first = type_args.next()?;
    if type_args.next().is_some() {
        return None;
    }
    Some(first)
}
pub(crate) fn type_stores_string_text(
    syn_type_ref: crate::types::SynTypeRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(match syn_type_ref.as_ref() {
        syn::Type::Array(array) => {
            type_stores_string_text(crate::types::SynTypeRef::from(array.elem.as_ref())).get()
        }
        syn::Type::Group(group) => {
            type_stores_string_text(crate::types::SynTypeRef::from(group.elem.as_ref())).get()
        }
        syn::Type::Paren(paren) => {
            type_stores_string_text(crate::types::SynTypeRef::from(paren.elem.as_ref())).get()
        }
        syn::Type::Path(path) => path.path.segments.iter().any(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::STR_ALT | constants_str::STRING
            ) || matches!(
                &segment.arguments,
                syn::PathArguments::AngleBracketed(arguments)
                    if arguments.args.iter().any(|argument| {
                        matches!(
                            argument,
                            syn::GenericArgument::Type(argument_type)
                                if type_stores_string_text(
                                    crate::types::SynTypeRef::from(argument_type)
                                )
                                .get()
                        )
                    })
            )
        }),
        syn::Type::Reference(reference) => {
            type_stores_string_text(crate::types::SynTypeRef::from(reference.elem.as_ref())).get()
        }
        syn::Type::Slice(slice) => {
            type_stores_string_text(crate::types::SynTypeRef::from(slice.elem.as_ref())).get()
        }
        syn::Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .any(|element| type_stores_string_text(crate::types::SynTypeRef::from(element)).get()),
        syn::Type::FnPtr(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Ptr(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}
pub(crate) fn type_is_string(
    syn_type_ref: crate::types::SynTypeRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(match syn_type_ref.get() {
        syn::Type::Path(ty_path) => ty_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::STRING),
        syn::Type::Array(_)
        | syn::Type::FnPtr(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}
pub(crate) fn type_is_str_ref(
    syn_type_ref: crate::types::SynTypeRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(match syn_type_ref.get() {
        syn::Type::Reference(ty_reference) => match &*ty_reference.elem {
            syn::Type::Path(ty_path) => ty_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::STR_ALT),
            syn::Type::Array(_)
            | syn::Type::FnPtr(_)
            | syn::Type::Group(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Paren(_)
            | syn::Type::Ptr(_)
            | syn::Type::Reference(_)
            | syn::Type::Slice(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Tuple(_)
            | syn::Type::Verbatim(_)
            | _ => false,
        },
        syn::Type::Array(_)
        | syn::Type::FnPtr(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Path(_)
        | syn::Type::Ptr(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    })
}
pub(crate) fn item_fn_is_proc_macro(
    syn_item_fn_ref: crate::types::SynItemFnRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(syn_item_fn_ref.as_ref().attrs.iter().any(|attr| {
        attr.path().is_ident(constants_str::PROC_MACRO_ALT)
            || attr.path().is_ident(constants_str::PROC_MACRO_DERIVE)
            || attr.path().is_ident(constants_str::PROC_MACRO_ATTRIBUTE)
    }))
}
pub(crate) fn attrs_contain_test_only_cfg(
    syn_attribute_list_ref: crate::types::SynAttributeListRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        syn_attribute_list_ref
            .as_ref()
            .iter()
            .any(|attr| attr_is_test_only_cfg(crate::types::SynAttributeRef::from(attr)).get()),
    )
}
pub(crate) fn item_fn_is_unit_test(
    syn_item_fn_ref: crate::types::SynItemFnRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(syn_item_fn_ref.as_ref().attrs.iter().any(|attr| {
        attr.path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::TEST_ALT_3)
            || attr_is_test_only_cfg(crate::types::SynAttributeRef::from(attr)).get()
    }))
}
pub(crate) fn derive_attr_has_terminal(
    syn_attribute_ref: crate::types::SynAttributeRef<'_>,
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::AnalyzerBool {
    if !syn_attribute_ref
        .as_ref()
        .path()
        .is_ident(constants_str::DERIVE)
    {
        return crate::types::AnalyzerBool::default();
    }
    crate::types::AnalyzerBool::from(
        syn_attribute_ref
            .as_ref()
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|paths| {
                paths.iter().any(|path| {
                    path.segments
                        .last()
                        .is_some_and(|segment| segment.ident == source_text_ref.as_ref())
                })
            }),
    )
}
pub(crate) fn sensitive_text_wrapper_identifier(
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::AnalyzerBool {
    let identifier_text = source_text_ref.as_ref();
    let lowercase = identifier_text.to_ascii_lowercase();
    let non_secret_token_metadata = [
        constants_str::VALUE_55BFC155,
        constants_str::VALUE_73F0D95A,
        constants_str::VALUE_A2E10FB9,
    ]
    .into_iter()
    .any(|fragment| lowercase.contains(fragment));
    crate::types::AnalyzerBool::from(
        [
            constants_str::PASSWORD,
            constants_str::SECRET,
            constants_str::VALUE_E265B6F5,
            constants_str::VALUE_6C793695,
            constants_str::VALUE_9032FF38,
        ]
        .into_iter()
        .any(|fragment| lowercase.contains(fragment))
            || lowercase.contains(constants_str::VALUE_3C469E9D) && !non_secret_token_metadata,
    )
}
pub(crate) fn type_contains_sensitive_text_or_bytes(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Array(array) => type_is_u8(array.elem.as_ref()),
        syn::Type::Path(path) => path.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::STRING | constants_str::STR_ALT | constants_str::VALUE_DFBD6AA3
            ) || segment.ident == constants_str::VEC
                && matches!(
                    &segment.arguments,
                    syn::PathArguments::AngleBracketed(angle_arguments)
                        if angle_arguments.args.iter().any(|argument| {
                            matches!(
                                argument,
                                syn::GenericArgument::Type(element_type)
                                    if type_is_u8(element_type)
                            )
                        })
                )
        }),
        syn::Type::Reference(reference) => {
            type_contains_sensitive_text_or_bytes(reference.elem.as_ref())
        }
        syn::Type::Slice(slice) => type_is_u8(slice.elem.as_ref()),
        syn::Type::Group(group) => type_contains_sensitive_text_or_bytes(group.elem.as_ref()),
        syn::Type::Paren(paren) => type_contains_sensitive_text_or_bytes(paren.elem.as_ref()),
        syn::Type::FnPtr(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Ptr(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_)
        | _ => false,
    }
}
pub(crate) fn type_is_u8(ty: &syn::Type) -> bool {
    matches!(
        ty,
        syn::Type::Path(path)
            if path.path.segments.last().is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_U8)
    )
}
pub(crate) fn path_to_string(
    syn_path_ref: crate::types::SynPathRef<'_>,
) -> crate::types::SourceText {
    crate::types::SourceText::try_from(
        syn_path_ref
            .as_ref()
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<String>>()
            .join(constants_str::PATH_SEPARATOR),
    )
    .expect(constants_str::DIAGNOSTIC_50C1E4A8)
}

pub(crate) fn is_runtime_policy_source_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    if is_test_source_path(path_ref).get() {
        return crate::types::AnalyzerBool::default();
    }
    if is_cfg_test_declared_child(path_ref.as_ref()) {
        return crate::types::AnalyzerBool::default();
    }
    if !path_ref
        .as_ref()
        .components()
        .any(|component| component.as_os_str() == constants_str::SRC_ALT)
    {
        return crate::types::AnalyzerBool::default();
    }
    let Some(cargo_toml_path) = nearest_cargo_toml_path(path_ref) else {
        return crate::types::AnalyzerBool::default();
    };
    let Some(parsed) = read_toml_table(crate::types::PathRef::from(cargo_toml_path.as_ref()))
    else {
        return crate::types::AnalyzerBool::default();
    };
    let is_proc_macro = parsed
        .as_ref()
        .get(constants_str::LIB)
        .and_then(toml::Value::as_table)
        .and_then(|lib| lib.get(constants_str::PROC_MACRO))
        == Some(&toml::Value::Boolean(true));
    crate::types::AnalyzerBool::from(
        !is_proc_macro && !is_test_crate(crate::types::TomlTableRef::from(parsed.as_ref())).get(),
    )
}
pub(crate) fn nearest_cargo_toml_path(
    path_ref: crate::types::PathRef<'_>,
) -> Option<crate::types::OwnedPathBuf> {
    path_ref
        .as_ref()
        .ancestors()
        .map(|ancestor| ancestor.join(constants_str::CARGO_TOML))
        .find(|cargo_toml_path| cargo_toml_path.exists())
        .map(crate::types::OwnedPathBuf::from)
}
pub(crate) fn is_str_constants_source_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    let constants_source_directory = std::path::Path::new(constants_str::STR_CONSTANTS_SRC_LIB_RS)
        .parent()
        .expect(constants_str::DIAGNOSTIC_77E3AB42);
    crate::types::AnalyzerBool::from(path_ref.as_ref().parent() == Some(constants_source_directory))
}
pub(crate) fn is_test_crate(
    toml_table_ref: crate::types::TomlTableRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        toml_table_ref
            .as_ref()
            .get(constants_str::PACKAGE)
            .and_then(toml::Value::as_table)
            .and_then(|package| package.get(constants_str::NAME))
            .and_then(toml::Value::as_str)
            .is_some_and(|name| {
                name == constants_str::TESTS_CODE_STYLE_RUST
                    || name
                        .split('_')
                        .any(|segment| segment == constants_str::TEST_ALT_3)
            }),
    )
}
pub(crate) fn is_test_crate_source_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    if is_test_source_path(path_ref).get() {
        return crate::types::AnalyzerBool::from(true);
    }
    nearest_cargo_toml_path(path_ref)
        .and_then(|cargo_toml_path| {
            read_toml_table(crate::types::PathRef::from(cargo_toml_path.as_ref()))
        })
        .map_or_else(crate::types::AnalyzerBool::default, |manifest| {
            is_test_crate(crate::types::TomlTableRef::from(manifest.as_ref()))
        })
}
pub(crate) fn is_test_source_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        path_ref.as_ref().components().any(|component| {
            component.as_os_str() == constants_str::TESTS_ALT
                || component.as_os_str() == constants_str::TESTS_CODE_STYLE_RUST
        }) || path_ref
            .as_ref()
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .is_some_and(|file_stem| {
                file_stem
                    .split('_')
                    .any(|segment| segment == constants_str::TESTS_ALT)
            }),
    )
}
pub(crate) fn is_non_policy_test_source_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(
        path_ref.as_ref() == std::path::Path::new(constants_str::CODE_STYLE_DOMAIN_FIXTURE_PATH),
    )
}
pub(crate) fn is_direct_fs_owner_source_path(
    path_ref: crate::types::PathRef<'_>,
) -> crate::types::AnalyzerBool {
    let path_text = path_ref.as_ref().to_string_lossy();
    crate::types::AnalyzerBool::from(
        constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
            .iter()
            .any(|suffix| {
                path_text.ends_with(suffix) || declared_child_matches(path_text.as_ref(), suffix)
            }),
    )
}
pub(crate) fn has_test_only_cfg_attr(
    syn_item_ref: crate::types::SynItemRef<'_>,
) -> crate::types::AnalyzerBool {
    crate::types::AnalyzerBool::from(item_attrs(syn_item_ref.as_ref()).is_some_and(|attrs| {
        attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(crate::types::SynAttributeRef::from(attr)).get())
    }))
}
pub(crate) fn cfg_test_attr_count(syn_item_ref: crate::types::SynItemRef<'_>) -> usize {
    item_attrs(syn_item_ref.as_ref()).map_or(constants_usize::ZERO, |attrs| {
        attrs
            .iter()
            .filter(|attr| {
                attr.path().is_ident(constants_str::CFG_ALT)
                    && attr
                        .parse_args::<syn::Ident>()
                        .is_ok_and(|identifier| identifier == constants_str::TEST_ALT_3)
            })
            .count()
    })
}
#[allow(
    clippy::shadow_reuse,
    reason = "the parameter follows its type name while match bindings retain concise variant-local names"
)]
fn item_attrs(item: &syn::Item) -> Option<&[syn::Attribute]> {
    Some(match item {
        syn::Item::Const(item) => item.attrs.as_slice(),
        syn::Item::Enum(item) => item.attrs.as_slice(),
        syn::Item::ExternCrate(item) => item.attrs.as_slice(),
        syn::Item::Fn(item) => item.attrs.as_slice(),
        syn::Item::ForeignMod(item) => item.attrs.as_slice(),
        syn::Item::Impl(item) => item.attrs.as_slice(),
        syn::Item::Macro(item) => item.attrs.as_slice(),
        syn::Item::Mod(item) => item.attrs.as_slice(),
        syn::Item::Static(item) => item.attrs.as_slice(),
        syn::Item::Struct(item) => item.attrs.as_slice(),
        syn::Item::Trait(item) => item.attrs.as_slice(),
        syn::Item::TraitAlias(item) => item.attrs.as_slice(),
        syn::Item::Type(item) => item.attrs.as_slice(),
        syn::Item::Union(item) => item.attrs.as_slice(),
        syn::Item::Use(item) => item.attrs.as_slice(),
        syn::Item::Verbatim(_) | _ => return None,
    })
}
pub(crate) fn attr_is_test_only_cfg(
    syn_attribute_ref: crate::types::SynAttributeRef<'_>,
) -> crate::types::AnalyzerBool {
    let attr_ref = syn_attribute_ref.as_ref();
    if !attr_ref.path().is_ident(constants_str::CFG_ALT) {
        return crate::types::AnalyzerBool::default();
    }
    let mut is_test_only_cfg = false;
    drop(attr_ref.parse_nested_meta(|meta| {
        if meta.path.is_ident(constants_str::TEST_ALT_3) {
            is_test_only_cfg = true;
        }
        if meta.path.is_ident(constants_str::FEATURE) {
            let value = meta.value()?;
            let lit: syn::LitStr = value.parse()?;
            if lit.value() == constants_str::TEST_UTILS {
                is_test_only_cfg = true;
            }
        }
        Ok(())
    }));
    crate::types::AnalyzerBool::from(is_test_only_cfg)
}
pub(crate) fn for_each_rs_file(
    mut on_file: impl FnMut(&crate::test_code_style_snapshot::RsSourceFile),
) {
    crate::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.rs_files().iter().for_each(&mut on_file);
    });
}
pub(crate) fn workspace_table_from_cargo_toml() -> crate::types::TomlTable {
    let mut table = std::fs::read_to_string(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
        .expect(constants_str::DIAGNOSTIC_39A0D238)
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_BEB11586);
    match table
        .remove(constants_str::WORKSPACE)
        .expect(constants_str::DIAGNOSTIC_F728192D)
    {
        toml::Value::Table(t) => crate::types::TomlTable::from(t),
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => std::panic::panic_any(constants_str::PANIC_2BFB0B62),
    }
}
pub(crate) fn toml_val_as_table_ref(
    toml_value_ref: crate::types::TomlValueRef<'_>,
    static_str: crate::types::StaticStr,
) -> crate::types::TomlTableRef<'_> {
    match toml_value_ref.get() {
        toml::Value::Table(t) => crate::types::TomlTableRef::from(t),
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => std::panic::panic_any(static_str.get().to_owned()),
    }
}
pub(crate) fn collect_non_workspace_dep_errors(
    path_ref: crate::types::PathRef<'_>,
    toml_table_ref: crate::types::TomlTableRef<'_>,
    mut diagnostic_messages_mut_ref: crate::types::DiagnosticMessagesMutRef<'_>,
) {
    let root_dependency_tables = [
        constants_str::DEPENDENCIES,
        constants_str::DEV_DEPENDENCIES,
        constants_str::BUILD_DEPENDENCIES,
    ]
    .into_iter()
    .filter_map(|dep_section| {
        toml_table_ref
            .as_ref()
            .get(dep_section)
            .and_then(toml::Value::as_table)
            .map(|deps| (dep_section.to_owned(), deps))
    });
    let target_dependency_tables = toml_table_ref
        .as_ref()
        .get(constants_str::TARGET)
        .and_then(toml::Value::as_table)
        .into_iter()
        .flat_map(toml::Table::iter)
        .flat_map(|(target_name, target_value)| {
            target_value
                .as_table()
                .into_iter()
                .flat_map(move |target_table| {
                    [
                        constants_str::DEPENDENCIES,
                        constants_str::DEV_DEPENDENCIES,
                        constants_str::BUILD_DEPENDENCIES,
                    ]
                    .into_iter()
                    .filter_map(move |dep_section| {
                        target_table
                            .get(dep_section)
                            .and_then(toml::Value::as_table)
                            .map(|deps| {
                                (
                                    format!(
                                        "{}.{target_name}.{dep_section}",
                                        constants_str::TARGET
                                    ),
                                    deps,
                                )
                            })
                    })
                })
        });
    diagnostic_messages_mut_ref.extend(
        root_dependency_tables
            .chain(target_dependency_tables)
            .flat_map(|(dep_section, deps)| {
                deps.iter()
                    .filter(move |(_, dep_value)| {
                        !matches!(
                            dep_value,
                            toml::Value::Table(dep_table)
                                if dep_table.get(constants_str::WORKSPACE)
                                    == Some(&toml::Value::Boolean(true))
                        )
                    })
                    .map(move |(dep_name, _)| {
                        format!(
                            "{}: dependency `{dep_name}` in [{dep_section}] must use `dep = {{ workspace = true }}`",
                            path_ref.as_ref().display(),
                        )
                    })
            }),
    );
}
pub(crate) fn workspace_members_as_strs(
    toml_table_ref: crate::types::TomlTableRef<'_>,
    static_str: crate::types::StaticStr,
) -> crate::types::SourceTextList {
    let Some(members) = toml_table_ref
        .as_ref()
        .get(constants_str::MEMBERS)
        .and_then(toml::Value::as_array)
    else {
        std::panic::panic_any(static_str.get().to_owned());
    };
    members
        .iter()
        .map(|member| {
            member
                .as_str()
                .unwrap_or_else(|| std::panic::panic_any(static_str.get().to_owned()))
                .to_owned()
        })
        .collect::<Vec<String>>()
        .into()
}
