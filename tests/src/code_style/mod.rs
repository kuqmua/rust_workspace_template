mod advanced_policy;
mod cargo_policy;
mod ci_policy;
mod contract_source_policy;
mod deployment_policy;
mod domain_analysis;
mod domain_type_policy;
mod lint_sync;
mod module_policy;
mod reuse_policy;
mod route_contract_policy;
mod runtime_analysis;
mod runtime_policy;
mod secret_policy;
mod snapshot;
mod source_analysis;
mod source_policy;
mod types;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct ExternalLeafWrapperNameException {
    identifier: types::StaticStr,
    reason: types::StaticStr,
}
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
enum RustOrClippy {
    Clippy,
    Rust,
}
impl RustOrClippy {
    fn name(self) -> types::StaticStr {
        match self {
            Self::Rust => types::StaticStr::from(constants_str::RUST),
            Self::Clippy => types::StaticStr::from(constants_str::CLIPPY),
        }
    }
}
#[allow(clippy::single_call_fn)] // named exception source keeps policy data separate from its validation loop
fn external_leaf_wrapper_name_exceptions() -> [ExternalLeafWrapperNameException; 0] {
    []
}
#[allow(clippy::single_call_fn)] // validates every external wrapper naming exception has an explicit reason before matching idents
fn is_external_leaf_wrapper_name_exception(
    identifier: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        external_leaf_wrapper_name_exceptions()
            .iter()
            .any(|exception| {
                assert!(!exception.reason.get().is_empty(), "c7ab0f62");
                exception.identifier.get() == identifier.as_ref()
            }),
    )
}
fn unowned_spawn_expr(expression: &syn::Expr) -> bool {
    let syn::Expr::Call(call) = expression else {
        return false;
    };
    let Some(path) = expr_call_path(types::SynExprCallRef::from(call)) else {
        return false;
    };
    let text = path_to_string(path);
    if matches!(text.as_ref(), "drop" | "std::mem::drop" | "core::mem::drop") {
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
#[allow(clippy::single_call_fn)] // keeps diagnostic-ID syntax validation named and fixture-tested
fn diagnostic_id_prefix(value: types::SourceTextRef<'_>) -> Option<types::SourceTextRef<'_>> {
    value
        .get()
        .get(..8usize)
        .filter(|prefix| {
            prefix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        })
        .filter(|_| {
            value.get().len() == 8usize
                || value
                    .get()
                    .get(8usize..)
                    .is_some_and(|suffix| suffix.starts_with(": ") || suffix.starts_with(' '))
        })
        .map(types::SourceTextRef::from)
}
#[allow(clippy::single_call_fn)] // keeps expect-message context validation separate from shared diagnostic-ID syntax validation
fn diagnostic_id_has_context(value: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    let context = value.get().get(8usize..).and_then(|suffix| {
        suffix
            .strip_prefix(": ")
            .or_else(|| suffix.strip_prefix(' '))
    });
    types::AnalyzerBool::from(
        context.is_some_and(|diagnostic_context| {
            diagnostic_context.split_whitespace().count() >= 2usize
        }),
    )
}
fn panic_uses_dynamic_diagnostic_id(value: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        value.as_ref().starts_with("{}")
            || value.as_ref().starts_with("{error_id}")
            || value.as_ref().starts_with("{exp_id}")
            || value.as_ref().starts_with("{uuid}"),
    )
}
fn macro_path_is_quote(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(path.as_ref().segments.last().is_some_and(|segment| {
        matches!(
            segment.ident.to_string().as_str(),
            "quote" | "quote_spanned"
        )
    }))
}
fn scan_generated_diagnostic_tokens(
    tokens: &proc_macro2::TokenStream,
    visitor: &mut source_analysis::DiagnosticIdVisitor,
) {
    let trees = tokens.clone().into_iter().collect::<Vec<_>>();
    trees.iter().enumerate().for_each(|(index, token)| {
        if let proc_macro2::TokenTree::Group(group) = token {
            scan_generated_diagnostic_tokens(&group.stream(), visitor);
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
            visitor
                .ers
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
                    && panic_uses_dynamic_diagnostic_id(types::SourceTextRef::from(
                        message_value.as_str(),
                    ))
                    .get() => {}
            Some(message_value) => visitor.record(
                types::SourceTextRef::from(identifier_text.as_str()),
                types::SourceTextRef::from(message_value.as_str()),
            ),
            None => match interpolated_identifier {
                Some(interpolated) => visitor.ers.push(format!(
                    "generated `{identifier_text}` uses unchecked interpolated diagnostic message `#{interpolated}`"
                )),
                None => visitor.ers.push(format!(
                    "generated `{identifier_text}` message must begin with a string literal"
                )),
            },
        }
    });
}
#[allow(clippy::single_call_fn)] // centralizes cross-file uniqueness validation behind the public policy test
fn check_expect_and_panic_contain_unique_diagnostic_ids() {
    let reviewed_interpolations = [
        (
            "pg_crud_macros_common/src/domain_types/token_stream_helpers.rs",
            "generated `panic` uses unchecked interpolated diagnostic message `#panic_uuid_token_stream`",
            "PanicUuidRef validates the diagnostic identifier before token generation",
        ),
        (
            "pg_crud_pg_table_generate_src/src/domain_types/source.rs",
            "generated `expect` uses unchecked interpolated diagnostic message `#expect_0`",
            "the generator receives the reviewed diagnostic identifier from its fixture catalog",
        ),
        (
            "pg_crud_pg_table_generate_src/src/domain_types/source.rs",
            "generated `expect` uses unchecked interpolated diagnostic message `#expect_1`",
            "the generator receives the reviewed diagnostic identifier from its fixture catalog",
        ),
        (
            "pg_crud_pg_types_generate_src/src/domain_types/source.rs",
            "generated `expect` uses unchecked interpolated diagnostic message `#id_double_quoted_token_stream`",
            "the date-range fixture passes a reviewed diagnostic identifier into the generator",
        ),
    ];
    let mut all_ids = Vec::new();
    let mut all_ers = Vec::new();
    let mut matched_interpolations = std::collections::BTreeSet::new();
    for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        let visitor = visit_syn_file(
            types::SynFileRef::from(ast),
            source_analysis::DiagnosticIdVisitor {
                ers: types::DiagnosticMsgs::default(),
                ids: types::SourceTextList::default(),
            },
        );
        all_ids.extend(visitor.ids);
        visitor.ers.into_iter().for_each(|error| {
            let reviewed =
                reviewed_interpolations
                    .iter()
                    .find(|(path_suffix, reviewed_error, reason)| {
                        path.ends_with(path_suffix)
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
    let duplicates = find_duplicate_strings(types::SourceTextListRef::from(all_ids.as_slice()));
    if !duplicates.is_empty() {
        all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
    }
    assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}");
}
fn assert_workspace_lints_match(
    rust_or_clippy: RustOrClippy,
    tool: types::StaticStr,
    parse_only_clippy: types::AnalyzerBool,
    exp_id: types::StaticStr,
    exceptions: types::StaticStrSliceRef<'_>,
) {
    let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
    let lints_from_cmd = lints_from_help_cmd(tool, parse_only_clippy, exp_id);
    compare_lints_vecs(
        rust_or_clippy,
        types::SourceTextListRef::from(lints_vec_from_cargo_toml.as_slice()),
        types::SourceTextListRef::from(lints_from_cmd.as_slice()),
        exceptions,
    );
}
#[allow(clippy::single_call_fn)] // helper intentionally stays extracted so command parsing remains decoupled from lint comparison orchestration
fn lints_from_help_cmd(
    tool: types::StaticStr,
    parse_only_clippy: types::AnalyzerBool,
    exp_id: types::StaticStr,
) -> types::SourceTextList {
    let output = macros_helpers::domain_types::tool_command::ToolCommand::new(
        macros_helpers::domain_types::tool_command::ToolProgramRef::from(tool.get()),
    )
    .args(
        macros_helpers::domain_types::tool_command::ToolArgsRef::from(
            [constants_str::W, constants_str::HELP].as_slice(),
        ),
    )
    .output()
    .unwrap_or_else(|_| panic!("{}", exp_id.get()));
    assert_cmd_output_ok(
        types::ProcessOutputRef::from(output.as_ref()),
        types::StaticStr::from(constants_str::VALUE_95D4595A),
        types::StaticStr::from(constants_str::CC4670A2),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let regex = if parse_only_clippy.get() {
        regex::Regex::new(constants_str::QUESTION_M_S_ASTERISK_CLIPPY_PATH_A_Z0_9_A_Z0_9)
            .expect("fbf14346 lints_from_help_cmd invariant must hold")
    } else {
        regex::Regex::new(constants_str::QUESTION_M_S_ASTERISK_A_Z0_9_A_Z0_9_PLUS_S)
            .expect("60d99c87 lints_from_help_cmd invariant must hold")
    };
    regex
        .captures_iter(&stdout)
        .map(|element_70833f93| {
            String::from(normalize_lint_name(types::SourceTextRef::from(
                &element_70833f93[1],
            )))
        })
        .collect::<Vec<String>>()
        .into()
}
#[allow(clippy::single_call_fn)] // shared command-output assertions keep status/stderr checks reusable for command-driven tests
fn assert_cmd_output_ok(
    output: types::ProcessOutputRef<'_>,
    status_exp_id: types::StaticStr,
    stderr_exp_id: types::StaticStr,
) {
    assert!(output.as_ref().status.success(), "{}", status_exp_id.get());
    let stderr = String::from_utf8_lossy(&output.as_ref().stderr);
    assert!(stderr.trim().is_empty(), "{}", stderr_exp_id.get());
}
#[allow(clippy::single_call_fn)] // centralizes lint-name normalization used by command output parsing
fn normalize_lint_name(v: types::SourceTextRef<'_>) -> types::SourceText {
    types::SourceText::try_from(v.as_ref().replace('-', constants_str::UNDERSCORE))
        .expect("f3d821a6 normalize_lint_name invariant must hold")
}
#[allow(clippy::single_call_fn)] // keeps workspace-dependency shape checks reusable and focused in one helper
fn validate_workspace_dep_spec(v: types::TomlValueRef<'_>) {
    let v_table = toml_val_as_table_ref(v, types::StaticStr::from(constants_str::CB693A3F));
    if let Some(path_v) = v_table.get().get(constants_str::PATH_ALT_5) {
        match path_v {
            toml::Value::String(_) => {
                match v_table.get().len() {
                    1 => (),
                    2 => validate_workspace_dep_default_features(v_table),
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
    validate_workspace_dep_version(v_table);
    validate_workspace_dep_default_features(v_table);
    match v_table.get().len() {
        2 => {}
        3 => {
            validate_workspace_dep_features(v_table);
        }
        _ => panic!("f1139378 {v_table:#?}"),
    }
}
fn validate_workspace_dep_default_features(v_table: types::TomlTableRef<'_>) {
    match v_table
        .get()
        .get(constants_str::DEFAULT_FEATURES)
        .expect("d2a8c4e1 validate_workspace_dep_default_features invariant must hold")
    {
        &toml::Value::Boolean(false) => (),
        &toml::Value::Boolean(true) => panic!("847a138f"),
        &toml::Value::String(_)
        | &toml::Value::Table(_)
        | &toml::Value::Integer(_)
        | &toml::Value::Float(_)
        | &toml::Value::Datetime(_)
        | &toml::Value::Array(_) => panic!("e5f7b1c3"),
    }
}
fn workspace_dep_disables_default_features(v: types::TomlValueRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        v.as_ref()
            .as_table()
            .and_then(|table| table.get(constants_str::DEFAULT_FEATURES))
            == Some(&toml::Value::Boolean(false)),
    )
}
fn unjustified_workspace_lint_allows(source: types::SourceTextRef<'_>) -> types::DiagnosticMsgs {
    let mut in_workspace_lints = false;
    types::DiagnosticMsgs::from(
        source
            .as_ref()
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let trimmed = line.trim();
                if trimmed.starts_with('[') {
                    in_workspace_lints = matches!(
                        trimmed,
                        "[workspace.lints.rust]" | "[workspace.lints.clippy]"
                    );
                    return None;
                }
                if !in_workspace_lints {
                    return None;
                }
                let (setting, comment) = line
                    .split_once('#')
                    .map_or((line, None), |(setting, comment)| (setting, Some(comment)));
                (setting.trim_end().ends_with("= \"allow\"")
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
fn commented_debug_statements(source: types::SourceTextRef<'_>) -> types::DiagnosticMsgs {
    types::DiagnosticMsgs::from(
        source
            .as_ref()
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let comment = line.trim_start().strip_prefix("//")?.trim_start();
                ["dbg!", "print!", "println!", "eprint!", "eprintln!"]
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
fn text_content_hygiene_ers(source: types::SourceTextRef<'_>) -> types::DiagnosticMsgs {
    let mut ers = types::DiagnosticMsgs::default();
    if !source.as_ref().is_empty() && !source.as_ref().ends_with('\n') {
        ers.push("missing final newline".to_owned());
    }
    if source.as_ref().contains('\r') {
        ers.push("contains carriage return".to_owned());
    }
    source
        .as_ref()
        .lines()
        .enumerate()
        .filter(|(_, line)| line.ends_with([' ', '\t']))
        .for_each(|(index, _)| {
            ers.push(format!(
                "line {} contains trailing whitespace",
                index.saturating_add(constants_usize::ONE)
            ));
        });
    ers
}
#[allow(clippy::single_call_fn)] // separates version shape assertion from dependency-table flow and keeps IDs stable
fn validate_workspace_dep_version(v_table: types::TomlTableRef<'_>) {
    match v_table
        .get()
        .get(constants_str::VERSION_ALT_3)
        .expect("d5b2b269 validate_workspace_dep_version invariant must hold")
    {
        toml::Value::String(version_string) => {
            assert!(
                is_exact_three_part_version(types::SourceTextRef::from(version_string.as_str()))
                    .get(),
                "6640b9bf"
            );
        }
        toml::Value::Table(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => panic!("a3410a37"),
    }
}
#[allow(clippy::single_call_fn)] // extracted to avoid repeated feature-type checks for dependency tables
fn validate_workspace_dep_features(v_table: types::TomlTableRef<'_>) {
    match v_table
        .get()
        .get(constants_str::FEATURES_ALT)
        .expect("473577d5 validate_workspace_dep_features invariant must hold")
    {
        &toml::Value::Array(_) => (),
        &toml::Value::String(_)
        | &toml::Value::Table(_)
        | &toml::Value::Integer(_)
        | &toml::Value::Float(_)
        | &toml::Value::Boolean(_)
        | &toml::Value::Datetime(_) => panic!("38ba32e9"),
    }
}
#[allow(clippy::single_call_fn)] // isolates exact-version parsing so version-format checks are reusable and testable
fn is_exact_three_part_version(v: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    let Some(rest) = v.as_ref().strip_prefix('=') else {
        return types::AnalyzerBool::default();
    };
    let mut iter = rest.split('.');
    types::AnalyzerBool::from(
        (0..3).all(|_| {
            iter.next()
                .and_then(|part| part.parse::<u64>().ok())
                .is_some()
        }) && iter.next().is_none(),
    )
}
#[allow(clippy::single_call_fn)] // helper intentionally stays extracted so lint diff logic remains reusable and independently readable
fn compare_lints_vecs(
    rust_or_clippy: RustOrClippy,
    lints_vec_from_cargo_toml: types::SourceTextListRef<'_>,
    lints_to_check: types::SourceTextListRef<'_>,
    lints_not_in_cargo_toml_vec_exceptions: types::StaticStrSliceRef<'_>,
) {
    let rust_or_clippy_name = rust_or_clippy.name().get();
    let lints_from_cargo_set = str_set(lints_vec_from_cargo_toml);
    let lints_to_check_set = str_set(lints_to_check);
    let lints_exceptions_set = lints_not_in_cargo_toml_vec_exceptions
        .get()
        .iter()
        .copied()
        .collect::<std::collections::HashSet<&str>>();
    let stale_exceptions = lints_not_in_cargo_toml_vec_exceptions
        .get()
        .iter()
        .copied()
        .filter(|lint| !lints_to_check_set.as_ref().contains(*lint))
        .collect::<Vec<&str>>();
    assert!(stale_exceptions.is_empty(), "31c5955d {stale_exceptions:?}");
    let (lints_not_in_cargo_toml, _reviewed_missing_lints) = split_lints_missing_from_cargo(
        lints_to_check,
        types::SourceTextRefHashSet::from(lints_from_cargo_set.as_ref()),
        types::SourceTextRefHashSet::from(&lints_exceptions_set),
    );
    assert!(
        lints_not_in_cargo_toml.is_empty(),
        "d2b7ba9f {rust_or_clippy_name} {lints_not_in_cargo_toml:?}"
    );
    let outdated_lints_in_file = collect_missing_items(
        lints_vec_from_cargo_toml,
        types::SourceTextRefHashSet::from(lints_to_check_set.as_ref()),
    );
    assert!(outdated_lints_in_file.is_empty(), "93787d2d");
}
#[allow(clippy::single_call_fn)] // shared parser keeps .env line-to-key extraction reusable and test behavior centralized
fn parse_env_key_line(line: types::SourceTextRef<'_>) -> Option<types::SourceTextRef<'_>> {
    let trimmed = line.get().trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    trimmed
        .split_once('=')
        .map(|(key, _)| types::SourceTextRef::from(key))
}
fn env_keys_from_file(path: types::StaticStr) -> types::SourceTextList {
    std::fs::read_to_string(path.get())
        .expect("b3a7c1e4 env_keys_from_file invariant must hold")
        .lines()
        .filter_map(|line| parse_env_key_line(types::SourceTextRef::from(line)))
        .map(|key| key.as_ref().to_owned())
        .collect::<Vec<String>>()
        .into()
}
fn collect_missing_items(
    items: types::SourceTextListRef<'_>,
    present_set: types::SourceTextRefHashSet<'_>,
) -> types::SourceTextList {
    types::SourceTextList::from(
        items
            .get()
            .iter()
            .map(String::as_str)
            .filter(|item| !present_set.as_ref().contains(item))
            .map(str::to_owned)
            .collect::<Vec<String>>(),
    )
}
fn collect_missing_key_ers(
    source_keys: types::SourceTextListRef<'_>,
    target_set: types::SourceTextRefHashSet<'_>,
    source_file: types::StaticStr,
    target_file: types::StaticStr,
) -> types::SourceTextList {
    types::SourceTextList::from(
        collect_missing_items(source_keys, target_set)
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
#[allow(clippy::single_call_fn)] // split keeps lint exception handling explicit while reusing missing-item collection
fn split_lints_missing_from_cargo(
    lints_to_check: types::SourceTextListRef<'_>,
    lints_from_cargo_set: types::SourceTextRefHashSet<'_>,
    lints_exceptions_set: types::SourceTextRefHashSet<'_>,
) -> (types::SourceTextList, types::SourceTextList) {
    let (lints_missing_by_exception, lints_not_in_cargo_toml) =
        collect_missing_items(lints_to_check, lints_from_cargo_set)
            .into_iter()
            .partition::<Vec<String>, _>(|lint| {
                lints_exceptions_set.as_ref().contains(lint.as_str())
            });
    (
        types::SourceTextList::from(lints_not_in_cargo_toml),
        types::SourceTextList::from(lints_missing_by_exception),
    )
}
#[allow(clippy::single_call_fn)] // helper intentionally stays extracted so workspace-lints table parsing remains separate from test driver wiring
fn lints_vec_from_cargo_toml_workspace(rust_or_clippy: RustOrClippy) -> types::SourceTextList {
    let workspace = workspace_table_from_cargo_toml();
    let lints = toml_val_as_table_ref(
        types::TomlValueRef::from(
            workspace
                .as_ref()
                .get(constants_str::LINTS)
                .expect("82eaea37 lints_vec_from_cargo_toml_workspace invariant must hold"),
        ),
        types::StaticStr::from(constants_str::CAE226CD),
    );
    let toml_v_table = toml_val_as_table_ref(
        types::TomlValueRef::from(
            lints
                .as_ref()
                .get(rust_or_clippy.name().get())
                .expect("dbd02f72 lints_vec_from_cargo_toml_workspace invariant must hold"),
        ),
        types::StaticStr::from(constants_str::VALUE_6F4580CE),
    );
    toml_v_table
        .as_ref()
        .keys()
        .cloned()
        .collect::<Vec<String>>()
        .into()
}
#[allow(clippy::single_call_fn)] // reusable collector stays split from assertion helper for callsites that need raw error vectors
fn collect_cargo_toml_ers(
    mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) -> types::SourceTextList {
    let mut ers = Vec::new();
    for_each_crate_manifest_file(|path| {
        let Some(parsed) = read_toml_table(types::PathRef::from(path)) else {
            return;
        };
        mk_ers(path, parsed.as_ref(), &mut ers);
    });
    types::SourceTextList::from(ers)
}
fn assert_cargo_toml_ers_empty(
    exp_id: types::StaticStr,
    mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) {
    let ers = collect_cargo_toml_ers(|path, parsed, ers| {
        mk_ers(path, parsed, ers);
    });
    assert_joined_ers_empty(types::SourceTextListRef::from(ers.as_slice()), exp_id);
}
fn assert_crate_manifest_cargo_policy(
    exp_id: types::StaticStr,
    mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
) {
    assert_cargo_toml_ers_empty(exp_id, |path, parsed, ers| {
        mk_ers(path, parsed, ers);
    });
}
fn assert_joined_ers_empty(ers: types::SourceTextListRef<'_>, exp_id: types::StaticStr) {
    assert_joined_ers_empty_with_ctx(
        ers,
        exp_id,
        types::SourceTextRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
    );
}
fn assert_joined_ers_empty_with_ctx(
    ers: types::SourceTextListRef<'_>,
    exp_id: types::StaticStr,
    ctx: types::SourceTextRef<'_>,
) {
    if ctx.as_ref().is_empty() {
        assert!(
            ers.as_ref().is_empty(),
            "{}\n{}",
            exp_id.get(),
            ers.as_ref().join("\n")
        );
    } else {
        assert!(
            ers.as_ref().is_empty(),
            "{} {}\n{}",
            exp_id.get(),
            ctx.as_ref(),
            ers.as_ref().join("\n")
        );
    }
}
fn assert_joined_ers_empty_sorted(
    mut ers: types::DiagnosticMsgsMutRef<'_>,
    exp_id: types::StaticStr,
) {
    ers.sort();
    assert_joined_ers_empty(types::SourceTextListRef::from(ers.as_slice()), exp_id);
}
fn str_set(v: types::SourceTextListRef<'_>) -> types::SourceTextHashSet<'_> {
    types::SourceTextHashSet::from(
        v.get()
            .iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<&str>>(),
    )
}
#[allow(clippy::single_call_fn)] // shared duplicate finder keeps uniqueness checks reusable and consistent
fn find_duplicate_strings(v: types::SourceTextListRef<'_>) -> types::SourceTextList {
    let mut seen = std::collections::HashSet::new();
    types::SourceTextList::from(
        v.get()
            .iter()
            .filter(|element_45f4b8bc| !seen.insert(element_45f4b8bc.as_str()))
            .cloned()
            .collect::<Vec<String>>(),
    )
}
#[allow(clippy::single_call_fn)] // reusable collector stays available for AST-policy tests and keeps collection logic separate from assertion wrappers
fn collect_rs_ast_ers(
    mut mk_ers: impl FnMut(&std::path::Path, &syn::File, &mut Vec<String>),
) -> types::SourceTextList {
    let mut ers = Vec::new();
    for_each_rs_file(|file| {
        let (path, ast) = (file.path().as_ref(), file.ast().as_ref());
        mk_ers(path, ast, &mut ers);
    });
    types::SourceTextList::from(ers)
}
fn visit_syn_file<V>(ast: types::SynFileRef<'_>, mut visitor: V) -> V
where
    V: for<'ast> syn::visit::Visit<'ast>,
{
    syn::visit::Visit::visit_file(&mut visitor, ast.as_ref());
    visitor
}
fn assert_rs_ast_ers_empty_with_ctx(
    exp_id: types::StaticStr,
    ctx: types::SourceTextRef<'_>,
    mut mk_ers: impl FnMut(&std::path::Path, &syn::File, &mut Vec<String>),
) {
    let ers = collect_rs_ast_ers(|path, ast, ers| {
        mk_ers(path, ast, ers);
    });
    assert_joined_ers_empty_with_ctx(types::SourceTextListRef::from(ers.as_slice()), exp_id, ctx);
}
fn read_toml_table(path: types::PathRef<'_>) -> Option<types::TomlTable> {
    snapshot::with_codebase_snapshot(|snapshot| snapshot.read_toml_table(path))
}
#[allow(clippy::single_call_fn)] // shared lookup avoids rereading crate manifests in text-based Cargo.toml style checks
fn cargo_toml_content(path: types::PathRef<'_>) -> Option<types::SourceText> {
    snapshot::with_codebase_snapshot(|snapshot| snapshot.cargo_toml_content(path))
}
#[allow(clippy::single_call_fn)] // isolates non-english diagnostics so file-level test stays focused on traversal and assertion
fn collect_non_english_symbol_ers(
    path: types::PathRef<'_>,
    v: types::SourceTextRef<'_>,
) -> types::SourceTextList {
    types::SourceTextList::from(
        v.as_ref()
            .lines()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                let line_number = line_idx.saturating_add(1);
                line.chars()
                    .filter(|ch| !is_allowed_english_char(types::AnalyzerChar::from(*ch)).get())
                    .map(move |ch| {
                        format!(
                            "{}:{} non-english symbol `{}` (U+{:04X})",
                            path.as_ref().display(),
                            line_number,
                            ch,
                            u32::from(ch)
                        )
                    })
            })
            .collect::<Vec<String>>(),
    )
}
#[allow(clippy::single_call_fn)] // shared character predicate keeps english-only symbol policy centralized
fn is_allowed_english_char(ch: types::AnalyzerChar) -> types::AnalyzerBool {
    let ch_value = ch.get();
    types::AnalyzerBool::from(
        matches!(ch_value, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}') || ch_value.is_ascii(),
    )
}
fn push_repeated_file_error(
    mut ers: types::DiagnosticMsgsMutRef<'_>,
    path: types::PathRef<'_>,
    message: types::SourceTextRef<'_>,
    times: types::AnalyzerCount,
) {
    ers.extend(
        std::iter::repeat_with(|| format!("{}: {}", path.as_ref().display(), message.as_ref()))
            .take(times.get()),
    );
}
fn workspace_crate_names() -> types::SourceTextBTreeSet {
    snapshot::with_codebase_snapshot(snapshot::CodebaseSnapshot::workspace_crate_names)
}
#[allow(clippy::single_call_fn)] // shared traversal uses cargo metadata so crate manifests match Cargo's view of workspace packages
fn for_each_crate_manifest_file(on_file: impl FnMut(&std::path::Path)) {
    snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.crate_manifest_paths().for_each(on_file);
    });
}
#[allow(clippy::single_call_fn)] // shared extension predicate keeps source-policy file-kind checks consistent
fn is_allowed_english_check_ext(ext: Option<types::SourceTextRef<'_>>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        ext.map(types::SourceTextRef::get),
        Some(
            constants_str::RS
                | constants_str::TOML
                | constants_str::TXT
                | constants_str::YML
                | constants_str::YAML
                | constants_str::JSON
        )
    ))
}
fn path_has_segment(
    path: types::SynPathRef<'_>,
    segment: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .segments
            .iter()
            .any(|element| element.ident == segment.as_ref()),
    )
}
#[allow(clippy::single_call_fn)] // names the From<String> trait-shape check for the string-wrapper policy visitor
fn item_impl_is_from_string(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
        path_ends_with(
            types::SynPathRef::from(path),
            types::StaticStrSliceRef::from([constants_str::FROM_ALT_3].as_slice()),
        )
        .get()
            && from_trait_arg_is_string(types::SynPathRef::from(path)).get()
    }))
}
#[allow(clippy::single_call_fn)] // names the TryFrom<String> trait-shape check for the string-wrapper policy visitor
fn item_impl_is_try_from_string(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
        path_ends_with(
            types::SynPathRef::from(path),
            types::StaticStrSliceRef::from([constants_str::TRYFROM].as_slice()),
        )
        .get()
            && from_trait_arg_is_string(types::SynPathRef::from(path)).get()
    }))
}
#[allow(clippy::single_call_fn)] // keeps length-check detection local to the string-wrapper TryFrom policy
fn item_impl_contains_len_call(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    let mut visitor = domain_analysis::LenMethodCallVisitor {
        found: types::AnalyzerBool::default(),
    };
    syn::visit::Visit::visit_item_impl(&mut visitor, item.as_ref());
    visitor.found
}
fn item_impl_self_ty_identifier(item: types::SynItemImplRef<'_>) -> Option<types::SourceText> {
    match item.as_ref().self_ty.as_ref() {
        syn::Type::Path(ty_path) => ty_path.path.segments.last().map(|segment| {
            types::SourceText::try_from(segment.ident.to_string())
                .expect("6a9f03d2 item_impl_self_ty_identifier invariant must hold")
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
fn from_trait_arg_is_string(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(path.as_ref().segments.last().is_some_and(|segment| {
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        args.args.iter().any(|arg| {
                            matches!(arg, syn::GenericArgument::Type(ty) if type_path_ends_with_identifier(types::SynTypeRef::from(ty), types::SourceTextRef::from(constants_str::STRING)).get())
                        })
                    }
                    syn::PathArguments::Parenthesized(_) | syn::PathArguments::None => false,
                }
            }))
}
fn item_struct_is_single_string_wrapper(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match &item.as_ref().fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            fields.unnamed.first().is_some_and(|field| {
                type_path_ends_with_identifier(
                    types::SynTypeRef::from(&field.ty),
                    types::SourceTextRef::from(constants_str::STRING),
                )
                .get()
            })
        }
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
    })
}
fn item_struct_is_single_field_tuple_wrapper(
    item: types::SynItemStructRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        matches!(&item.as_ref().fields, syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1),
    )
}
#[allow(clippy::single_call_fn)] // isolates tuple field visibility parsing from policy diagnostics
fn item_struct_single_field_is_non_private(
    item: types::SynItemStructRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match &item.as_ref().fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields
            .unnamed
            .first()
            .is_some_and(|field| !matches!(field.vis, syn::Visibility::Inherited)),
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
    })
}
#[allow(clippy::single_call_fn)] // isolated serde derive detection keeps the visitor condition declarative
fn item_struct_derives_deserialize(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        attr.path().is_ident(constants_str::DERIVE)
            && match &attr.meta {
                syn::Meta::List(list) => list
                    .tokens
                    .to_string()
                    .contains(constants_str::CODE_STYLE_DESERIALIZE_DERIVE_NAME),
                syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
            }
    }))
}
#[allow(clippy::single_call_fn)] // isolated serde conversion detection keeps attribute parsing reusable and testable
fn item_struct_deserialize_uses_conversion(
    item: types::SynItemStructRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        if !attr.path().is_ident(constants_str::SERDE) {
            return false;
        }
        match &attr.meta {
            syn::Meta::List(list) => {
                let tokens = list.tokens.to_string();
                tokens.contains(constants_str::CODE_STYLE_SERDE_FROM_ATTR_FRAGMENT)
                    || tokens.contains(constants_str::CODE_STYLE_SERDE_TRY_FROM_ATTR_FRAGMENT)
            }
            syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
        }
    }))
}
#[allow(clippy::single_call_fn)] // conversion derive recognition is kept separate from wrapper collection
fn item_struct_derives_conversion(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        if !attr.path().is_ident(constants_str::DERIVE) {
            return false;
        }
        match &attr.meta {
            syn::Meta::List(list) => {
                let tokens = list.tokens.to_string();
                tokens.contains(constants_str::NEWTYPE_FROM_INNER_DERIVE_NAME)
                    || tokens.contains(constants_str::BOUNDEDSTRING)
                    || tokens.contains(constants_str::TRYFROM)
            }
            syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
        }
    }))
}
#[allow(clippy::single_call_fn)] // keeps FromInner derive detection reusable inside wrapper conversion collection
fn item_struct_derives_from_inner(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        if !attr.path().is_ident(constants_str::DERIVE) {
            return false;
        }
        match &attr.meta {
            syn::Meta::List(list) => list
                .tokens
                .to_string()
                .contains(constants_str::NEWTYPE_FROM_INNER_DERIVE_NAME),
            syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
        }
    }))
}
#[allow(clippy::single_call_fn)] // keeps TryFrom derive detection reusable inside wrapper conversion collection
fn item_struct_derives_try_from(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        if !attr.path().is_ident(constants_str::DERIVE) {
            return false;
        }
        match &attr.meta {
            syn::Meta::List(list) => {
                let tokens = list.tokens.to_string();
                tokens.contains(constants_str::NEWTYPE_TRY_FROM_DERIVE_NAME)
                    || tokens.contains(constants_str::BOUNDEDSTRING)
                    || tokens.contains(constants_str::TRYFROM)
            }
            syn::Meta::NameValue(_) | syn::Meta::Path(_) => false,
        }
    }))
}
#[allow(clippy::single_call_fn)] // isolates `From<T>` impl detection for tuple-wrapper conversion analysis
fn item_impl_is_from(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
        path.segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::FROM_ALT_3)
    }))
}
#[allow(clippy::single_call_fn)] // isolates `TryFrom<T>` impl detection for tuple-wrapper conversion analysis
fn item_impl_is_try_from(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
        path.segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::TRYFROM)
    }))
}
fn item_impl_input_type_is(
    item: types::SynItemImplRef<'_>,
    expected_input_type: &syn::Type,
) -> types::AnalyzerBool {
    let source_type = item.as_ref().trait_.as_ref().and_then(|(path, _)| {
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
    types::AnalyzerBool::from(
        source_type.is_some_and(|input_type| input_type == expected_input_type),
    )
}
fn item_impl_is_from_or_try_from(item: types::SynItemImplRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().trait_.as_ref().is_some_and(|(path, _)| {
        path.segments.last().is_some_and(|segment| {
            segment.ident == constants_str::FROM_ALT_3 || segment.ident == constants_str::TRYFROM
        })
    }))
}
#[allow(clippy::single_call_fn)] // diagnostic conversion errors intentionally carry raw length metadata
fn identifier_is_diagnostic_try_from_string_error(
    identifier: types::SynIdentifierRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        identifier
            .as_ref()
            .to_string()
            .ends_with(constants_str::TRYFROMSTRINGERROR),
    )
}
fn method_is_explicit_wrapper_accessor(
    identifier: types::SynIdentifierRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        identifier.as_ref().to_string().as_str(),
        constants_str::GET_ALT | constants_str::INTO_INNER
    ))
}
#[allow(clippy::single_call_fn)] // keeps the derive-validator boundary exception explicit and narrow
fn method_is_private_newtype_validator(method: &syn::ImplItemFn) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        matches!(method.vis, syn::Visibility::Inherited)
            && method.sig.ident == constants_str::VALIDATE,
    )
}
fn type_path_ends_with_identifier(
    ty: types::SynTypeRef<'_>,
    identifier: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.as_ref() {
        syn::Type::Path(ty_path) => ty_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == identifier.as_ref()),
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
#[allow(clippy::single_call_fn)] // keeps FromInner derive detection reusable inside the string-wrapper policy
fn attr_has_newtype_from_option(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(constants_str::DERIVE) {
        return types::AnalyzerBool::default();
    }
    types::AnalyzerBool::from(attr_ref.meta.require_list().is_ok_and(|list| {
        list.tokens
            .to_string()
            .contains(constants_str::NEWTYPE_FROM_INNER_DERIVE_NAME)
    }))
}
#[allow(clippy::single_call_fn)] // keeps BoundedString derive parsing reusable inside the string-wrapper policy
fn attr_has_bounded_string_derive(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(constants_str::DERIVE) {
        return types::AnalyzerBool::default();
    }
    types::AnalyzerBool::from(attr_ref.meta.require_list().is_ok_and(|list| {
        list.tokens
            .to_string()
            .contains(constants_str::BOUNDEDSTRING)
    }))
}
#[allow(clippy::single_call_fn)] // bounded string wrappers satisfy length policy only when max is explicit
fn attr_has_bounded_string_max_bound(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(constants_str::BOUNDED_STRING) {
        return types::AnalyzerBool::default();
    }
    let mut has_max = false;
    drop(attr_ref.parse_nested_meta(|meta| {
        if meta.path.is_ident(constants_str::MAX) {
            drop(meta.value()?.parse::<syn::Expr>()?);
            has_max = true;
            return Ok(());
        }
        Err(meta.error(constants_str::UNKNOWN_BOUNDED_STRING_OPTION))
    }));
    types::AnalyzerBool::from(has_max)
}
fn path_ends_with(
    path: types::SynPathRef<'_>,
    segments: types::StaticStrSliceRef<'_>,
) -> types::AnalyzerBool {
    let path_ref = path.as_ref();
    types::AnalyzerBool::from(
        path_ref.segments.len() >= segments.get().len()
            && path_ref
                .segments
                .iter()
                .rev()
                .zip(segments.get().iter().rev())
                .all(|(got, exp)| got.ident == *exp),
    )
}
#[allow(clippy::single_call_fn)] // names Self-path handling separately from domain type path traversal
fn path_first_segment_is_self(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .segments
            .first()
            .is_some_and(|segment| segment.ident == constants_str::SELF),
    )
}
fn expr_call_path(call: types::SynExprCallRef<'_>) -> Option<types::SynPathRef<'_>> {
    match call.get().func.as_ref() {
        syn::Expr::Path(path) => Some(types::SynPathRef::from(&path.path)),
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
#[allow(clippy::single_call_fn)] // extracts repo macro domain type discovery from the visitor traversal
fn collect_generate_pg_types_domain_names(
    tokens: types::SourceTextRef<'_>,
    names: &mut types::SourceTextBTreeSet,
) {
    let re = regex::Regex::new(constants_str::A_ZA_Z0_9_PLUS_AS_A_ZA_Z0_9_PLUS)
        .expect("f4e61b29 collect_generate_pg_types_domain_names invariant must hold");
    re.captures_iter(tokens.as_ref())
        .filter_map(|captures| {
            let base = captures.get(1).map(|element| element.as_str())?;
            base.split_once(constants_str::AS)
        })
        .for_each(|(prefix, suffix)| {
            let _: bool = names.insert(format!("{prefix}AsNonNull{suffix}"));
            let _: bool = names.insert(format!("Optional{prefix}AsNullable{suffix}"));
        });
}
#[allow(clippy::single_call_fn)] // config_lib helper macros declare domain wrapper structs from their first argument
fn config_lib_domain_type_macro_path(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path_ends_with(
            path,
            types::StaticStrSliceRef::from(
                [
                    constants_str::CONFIG_LIB_MACROS,
                    constants_str::IMPL_TRY_FROM_NON_EMPTY_STRING,
                ]
                .as_slice(),
            ),
        )
        .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::CONFIG_LIB_MACROS,
                        constants_str::IMPL_TRY_FROM_SECRET_URL,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::CONFIG_LIB_MACROS,
                        constants_str::IMPL_TRY_FROM_PARSE,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::CONFIG_LIB_MACROS,
                        constants_str::IMPL_TRY_FROM_PARSE_STRING_ERROR,
                    ]
                    .as_slice(),
                ),
            )
            .get(),
    )
}
fn collect_first_macro_identifier_domain_name(
    tokens: types::SourceTextRef<'_>,
    names: &mut types::SourceTextBTreeSet,
) {
    let re = regex::Regex::new(constants_str::S_ASTERISK_A_ZA_Z_A_ZA_Z0_9_ASTERISK_S_ASTERISK)
        .expect("fc65b7c4 collect_first_macro_identifier_domain_name invariant must hold");
    if let Some(name) = re
        .captures(tokens.as_ref())
        .and_then(|captures| captures.get(1))
        .map(|name| name.as_str())
    {
        let _: bool = names.insert(name.to_owned());
    }
}
#[allow(clippy::single_call_fn)] // keeps Arc type policy readable apart from syn type matching
fn type_contains_segment(
    ty: types::SynTypeRef<'_>,
    segment: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.as_ref() {
        syn::Type::Path(path) => {
            path_has_segment(types::SynPathRef::from(&path.path), segment).get()
        }
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
#[allow(clippy::single_call_fn)] // names the async-blocking method policy separately from traversal code
fn method_is_blocking_async_call(method: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        method.as_ref(),
        constants_str::BLOCK_ON
            | constants_str::BLOCK_IN_PLACE
            | constants_str::BLOCKING_RECV
            | constants_str::BLOCKING_SEND
    ))
}
#[allow(clippy::single_call_fn)] // names the async-blocking function policy separately from traversal code
fn path_is_blocking_async_call(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    let path_text = path_to_string(path);
    types::AnalyzerBool::from(
        path_ends_with(
            path,
            types::StaticStrSliceRef::from(
                [
                    constants_str::FUTURES,
                    constants_str::EXECUTOR,
                    constants_str::BLOCK_ON,
                ]
                .as_slice(),
            ),
        )
        .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::TOKIO,
                        constants_str::TASK,
                        constants_str::BLOCK_IN_PLACE,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::STD,
                        constants_str::THREAD,
                        constants_str::SLEEP,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || constants_str::BLOCKING_STD_FS_CALLS.contains(&path_text.as_ref())
            || constants_str::BLOCKING_STD_NET_CALLS.contains(&path_text.as_ref()),
    )
}
#[allow(clippy::single_call_fn)] // names the external-service unit-test policy separately from traversal code
fn path_is_external_service_client(path: types::SynPathRef<'_>) -> types::AnalyzerBool {
    let path_text = path_to_string(path);
    types::AnalyzerBool::from(
        path_ends_with(
            path,
            types::StaticStrSliceRef::from(
                [
                    constants_str::REQWEST,
                    constants_str::CLIENT,
                    constants_str::NEW,
                ]
                .as_slice(),
            ),
        )
        .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::STD,
                        constants_str::NET,
                        constants_str::TCPSTREAM,
                        constants_str::CONNECT,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::STD,
                        constants_str::NET,
                        constants_str::TCPLISTENER,
                        constants_str::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::STD,
                        constants_str::NET,
                        constants_str::UDPSOCKET,
                        constants_str::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::TOKIO,
                        constants_str::NET,
                        constants_str::TCPSTREAM,
                        constants_str::CONNECT,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::TOKIO,
                        constants_str::NET,
                        constants_str::TCPLISTENER,
                        constants_str::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || path_ends_with(
                path,
                types::StaticStrSliceRef::from(
                    [
                        constants_str::TOKIO,
                        constants_str::NET,
                        constants_str::UDPSOCKET,
                        constants_str::BIND,
                    ]
                    .as_slice(),
                ),
            )
            .get()
            || [
                "reqwest::get",
                "reqwest::Client::builder",
                "sqlx::PgConnection::connect",
                "sqlx::PgPool::connect",
            ]
            .contains(&path_text.as_ref()),
    )
}
#[allow(clippy::single_call_fn)] // extracted to keep the domain policy test focused on assertion flow
fn declared_domain_type_names() -> types::SourceTextBTreeSet {
    let mut names = std::collections::BTreeSet::new();
    for_each_rs_file(|file| {
        let ast = file.ast().as_ref();
        let visitor = visit_syn_file(
            types::SynFileRef::from(ast),
            domain_analysis::DeclaredDomainTypeVisitor {
                names: types::SourceTextBTreeSet::default(),
            },
        );
        names.extend(visitor.names);
    });
    types::SourceTextBTreeSet::from(names)
}
fn len_checked_function_names(file: types::SynFileRef<'_>) -> types::SourceTextBTreeSet {
    let mut visitor = domain_analysis::LenCheckedFunctionNameVisitor {
        names: types::SourceTextBTreeSet::default(),
    };
    syn::visit::Visit::visit_file(&mut visitor, file.as_ref());
    visitor.names
}
fn string_wrapper_names(ast: types::SynFileRef<'_>) -> types::SourceTextBTreeSet {
    visit_syn_file(
        ast,
        domain_analysis::StringWrapperNameVisitor {
            names: types::SourceTextBTreeSet::default(),
        },
    )
    .names
}
fn domain_type_policy_should_check_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    if path
        .as_ref()
        .starts_with(constants_str::CODE_STYLE_PG_CRUD_COMMON_BENCHES)
        || path
            .as_ref()
            .starts_with(constants_str::CODE_STYLE_LOCATION_TEST_SRC)
        || path
            .as_ref()
            .starts_with(constants_str::CODE_STYLE_BOUNDED_TYPES_SRC)
        || path
            .as_ref()
            .starts_with(constants_str::SERVER_ADMIN_FRONTEND_SRC_UI)
    {
        return types::AnalyzerBool::default();
    }
    let Some(cargo_toml_path) = nearest_cargo_toml_path(path) else {
        return types::AnalyzerBool::default();
    };
    types::AnalyzerBool::from(cargo_toml_path.as_ref().is_file())
}
fn is_code_style_meta_harness_source_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .starts_with(constants_str::TESTS_SRC_CODE_STYLE),
    )
}
#[allow(clippy::single_call_fn)] // keeps transparent container policy separate from path validation
fn is_structural_generic_container(identifier: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(matches!(
        identifier.as_ref(),
        constants_str::OPTION | constants_str::RESULT
    ))
}
fn analyzer_state_raw_container_ty(
    ty: types::SynTypeRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    match ty.get() {
        syn::Type::Group(ty_group) => {
            analyzer_state_raw_container_ty(types::SynTypeRef::from(&*ty_group.elem))
        }
        syn::Type::Paren(ty_paren) => {
            analyzer_state_raw_container_ty(types::SynTypeRef::from(&*ty_paren.elem))
        }
        syn::Type::Path(ty_path) => {
            analyzer_state_raw_container_ty_path(types::SynTypePathRef::from(ty_path))
        }
        syn::Type::Reference(ty_reference) => {
            analyzer_state_raw_container_ty(types::SynTypeRef::from(&*ty_reference.elem))
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
fn raw_text_return_ty(ty: types::SynTypeRef<'_>) -> Option<(types::StaticStr, types::StaticStr)> {
    match ty.get() {
        syn::Type::Group(ty_group) => raw_text_return_ty(types::SynTypeRef::from(&*ty_group.elem)),
        syn::Type::Paren(ty_paren) => raw_text_return_ty(types::SynTypeRef::from(&*ty_paren.elem)),
        syn::Type::Path(ty_path) => raw_text_return_ty_path(types::SynTypePathRef::from(ty_path)),
        syn::Type::Reference(_) if type_is_str_ref(ty).get() => Some((
            types::StaticStr::from(constants_str::STR),
            types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXTREF),
        )),
        syn::Type::Reference(ty_reference) => {
            raw_text_return_ty(types::SynTypeRef::from(&*ty_reference.elem))
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
#[allow(clippy::single_call_fn)] // separates return path matching from nested raw text return traversal
fn raw_text_return_ty_path(
    ty_path: types::SynTypePathRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    let ty_path_ref = ty_path.get();
    let segment = ty_path_ref.path.segments.last()?;
    let identifier = segment.ident.to_string();
    match identifier.as_str() {
        constants_str::STRING => Some((
            types::StaticStr::from(constants_str::STRING),
            types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXT),
        )),
        constants_str::VEC
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_string(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(constants_str::VEC_STRING),
                types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXTLIST),
            ))
        }
        constants_str::OPTION
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_str_ref(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(constants_str::OPTION_STR),
                types::StaticStr::from(constants_str::OPTION_TYPES_PATH_SOURCETEXTREF),
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
        | constants_str::BTREESET => {
            raw_text_return_path_arguments(types::SynPathArgumentsRef::from(&segment.arguments))
        }
        _ => None,
    }
}
#[allow(clippy::single_call_fn)] // keeps nested helper-return traversal independent from field-state diagnostics
fn raw_text_return_path_arguments(
    arguments: types::SynPathArgumentsRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    match arguments.get() {
        syn::PathArguments::AngleBracketed(args) => args.args.iter().find_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => raw_text_return_ty(types::SynTypeRef::from(ty)),
            syn::GenericArgument::AssocConst(_)
            | syn::GenericArgument::AssocType(_)
            | syn::GenericArgument::Constraint(_)
            | syn::GenericArgument::Const(_)
            | syn::GenericArgument::Lifetime(_)
            | _ => None,
        }),
        syn::PathArguments::Parenthesized(args) => args
            .inputs
            .iter()
            .find_map(|arg| raw_text_return_ty(types::SynTypeRef::from(&arg.ty)))
            .or_else(|| match &args.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => raw_text_return_ty(types::SynTypeRef::from(&**ty)),
            }),
        syn::PathArguments::None => None,
    }
}
#[allow(clippy::single_call_fn)] // separates path-shape matching from recursive wrapper/state field traversal
fn analyzer_state_raw_container_ty_path(
    ty_path: types::SynTypePathRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    let ty_path_ref = ty_path.get();
    let segment = ty_path_ref.path.segments.last()?;
    let identifier = segment.ident.to_string();
    match identifier.as_str() {
        constants_str::VEC
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_string(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(constants_str::VEC_STRING),
                types::StaticStr::from(constants_str::TYPES_PATH_SOURCETEXTLIST),
            ))
        }
        constants_str::BTREESET
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_string(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(constants_str::BTREESET_STRING),
                types::StaticStr::from(constants_str::TYPES_PATH_STDSOURCETEXTSET),
            ))
        }
        constants_str::HASHSET
            if single_angle_type_arg(types::SynPathArgumentsRef::from(&segment.arguments))
                .is_some_and(|ty| type_is_str_ref(types::SynTypeRef::from(ty.get())).get()) =>
        {
            Some((
                types::StaticStr::from(constants_str::HASHSET_STR),
                types::StaticStr::from(constants_str::TYPES_PATH_STDSOURCETEXTHASHSET_OR_TYPES_PATH_STDSOURCETEXTREFSET),
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
        | constants_str::BTREEMAP => analyzer_state_raw_container_path_arguments(
            types::SynPathArgumentsRef::from(&segment.arguments),
        ),
        _ => None,
    }
}
#[allow(clippy::single_call_fn)] // keeps nested container traversal readable where state fields are diagnosed
fn analyzer_state_raw_container_path_arguments(
    arguments: types::SynPathArgumentsRef<'_>,
) -> Option<(types::StaticStr, types::StaticStr)> {
    match arguments.get() {
        syn::PathArguments::AngleBracketed(args) => args.args.iter().find_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => {
                analyzer_state_raw_container_ty(types::SynTypeRef::from(ty))
            }
            syn::GenericArgument::AssocConst(_)
            | syn::GenericArgument::AssocType(_)
            | syn::GenericArgument::Constraint(_)
            | syn::GenericArgument::Const(_)
            | syn::GenericArgument::Lifetime(_)
            | _ => None,
        }),
        syn::PathArguments::Parenthesized(args) => args
            .inputs
            .iter()
            .find_map(|arg| analyzer_state_raw_container_ty(types::SynTypeRef::from(&arg.ty)))
            .or_else(|| match &args.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => {
                    analyzer_state_raw_container_ty(types::SynTypeRef::from(&**ty))
                }
            }),
        syn::PathArguments::None => None,
    }
}
fn single_angle_type_arg(
    arguments: types::SynPathArgumentsRef<'_>,
) -> Option<types::SynTypeRef<'_>> {
    let syn::PathArguments::AngleBracketed(args) = arguments.get() else {
        return None;
    };
    let mut type_args = args.args.iter().filter_map(|arg| match arg {
        syn::GenericArgument::Type(ty) => Some(types::SynTypeRef::from(ty)),
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
fn type_stores_string_text(ty: types::SynTypeRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.as_ref() {
        syn::Type::Array(array) => {
            type_stores_string_text(types::SynTypeRef::from(array.elem.as_ref())).get()
        }
        syn::Type::Group(group) => {
            type_stores_string_text(types::SynTypeRef::from(group.elem.as_ref())).get()
        }
        syn::Type::Paren(paren) => {
            type_stores_string_text(types::SynTypeRef::from(paren.elem.as_ref())).get()
        }
        syn::Type::Path(path) => path.path.segments.iter().any(|segment| {
            matches!(segment.ident.to_string().as_str(), "str" | "String")
                || matches!(
                    &segment.arguments,
                    syn::PathArguments::AngleBracketed(arguments)
                        if arguments.args.iter().any(|argument| {
                            matches!(
                                argument,
                                syn::GenericArgument::Type(argument_type)
                                    if type_stores_string_text(
                                        types::SynTypeRef::from(argument_type)
                                    )
                                    .get()
                            )
                        })
                )
        }),
        syn::Type::Reference(reference) => {
            type_stores_string_text(types::SynTypeRef::from(reference.elem.as_ref())).get()
        }
        syn::Type::Slice(slice) => {
            type_stores_string_text(types::SynTypeRef::from(slice.elem.as_ref())).get()
        }
        syn::Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .any(|element| type_stores_string_text(types::SynTypeRef::from(element)).get()),
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
fn type_is_string(ty: types::SynTypeRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.get() {
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
fn type_is_str_ref(ty: types::SynTypeRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match ty.get() {
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
fn item_fn_is_proc_macro(item: types::SynItemFnRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        attr.path().is_ident(constants_str::PROC_MACRO_ALT)
            || attr.path().is_ident(constants_str::PROC_MACRO_DERIVE)
            || attr.path().is_ident(constants_str::PROC_MACRO_ATTRIBUTE)
    }))
}
fn attrs_contain_test_only_cfg(attrs: types::SynAttributeListRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        attrs
            .as_ref()
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
    )
}
fn item_fn_is_unit_test(item: types::SynItemFnRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(item.as_ref().attrs.iter().any(|attr| {
        attr.path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::TEST_ALT_3)
            || attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()
    }))
}
fn derive_attr_has_terminal(
    attr: types::SynAttributeRef<'_>,
    terminal: types::SourceTextRef<'_>,
) -> types::AnalyzerBool {
    if !attr.as_ref().path().is_ident(constants_str::DERIVE) {
        return types::AnalyzerBool::default();
    }
    types::AnalyzerBool::from(
        attr.as_ref()
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|paths| {
                paths.iter().any(|path| {
                    path.segments
                        .last()
                        .is_some_and(|segment| segment.ident == terminal.as_ref())
                })
            }),
    )
}
fn sensitive_text_wrapper_identifier(identifier: types::SourceTextRef<'_>) -> types::AnalyzerBool {
    let identifier_text = identifier.as_ref();
    let lowercase = identifier_text.to_ascii_lowercase();
    let non_secret_token_metadata = ["tokenaudience", "tokenissuer", "tokenpart"]
        .into_iter()
        .any(|fragment| lowercase.contains(fragment));
    types::AnalyzerBool::from(
        ["password", "secret", "credential", "apikey", "cookievalue"]
            .into_iter()
            .any(|fragment| lowercase.contains(fragment))
            || lowercase.contains("token") && !non_secret_token_metadata,
    )
}
#[allow(clippy::single_call_fn)] // limits the secret Debug policy to wrappers that directly contain text or bytes
fn item_struct_wraps_text(item: types::SynItemStructRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        item.as_ref()
            .fields
            .iter()
            .any(|field| type_contains_sensitive_text_or_bytes(&field.ty)),
    )
}
fn type_contains_sensitive_text_or_bytes(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Array(array) => type_is_u8(array.elem.as_ref()),
        syn::Type::Path(path) => path.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                "String" | "str" | "SecretBox"
            ) || segment.ident == "Vec"
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
fn type_is_u8(ty: &syn::Type) -> bool {
    matches!(
        ty,
        syn::Type::Path(path)
            if path.path.segments.last().is_some_and(|segment| segment.ident == "u8")
    )
}
fn path_to_string(path: types::SynPathRef<'_>) -> types::SourceText {
    types::SourceText::try_from(
        path.as_ref()
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<String>>()
            .join(constants_str::PATH_SEPARATOR),
    )
    .expect("50c1e4a8 path_to_string invariant must hold")
}
#[allow(clippy::single_call_fn)] // keeps external-wrapper naming suggestion generation readable at the call site
fn identifier_to_upper_camel_fragment(
    identifier: types::SynIdentifierRef<'_>,
) -> types::SourceText {
    let (out, _) = identifier.as_ref().to_string().chars().fold(
        (String::new(), true),
        |(mut out, mut next_upper), ch| {
            if ch == '_' {
                next_upper = true;
                return (out, next_upper);
            }
            if next_upper {
                ch.to_uppercase().for_each(|upper| out.push(upper));
                next_upper = false;
            } else {
                out.push(ch);
            }
            (out, next_upper)
        },
    );
    types::SourceText::try_from(out)
        .expect("9ea072c4 identifier_to_upper_camel_fragment invariant must hold")
}
fn is_runtime_policy_source_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    let path_text = path.as_ref().to_string_lossy();
    if is_test_source_path(path).get() {
        return types::AnalyzerBool::default();
    }
    if constants_str::CODE_STYLE_RUNTIME_TEST_HELPER_SUFFIXES
        .iter()
        .any(|suffix| path_text.ends_with(suffix))
    {
        return types::AnalyzerBool::default();
    }
    if !path
        .as_ref()
        .components()
        .any(|component| component.as_os_str() == constants_str::SRC_ALT)
    {
        return types::AnalyzerBool::default();
    }
    let Some(cargo_toml_path) = nearest_cargo_toml_path(path) else {
        return types::AnalyzerBool::default();
    };
    let Some(parsed) = read_toml_table(types::PathRef::from(cargo_toml_path.as_ref())) else {
        return types::AnalyzerBool::default();
    };
    types::AnalyzerBool::from(
        !is_proc_macro_crate(types::TomlTableRef::from(parsed.as_ref())).get()
            && !is_test_crate(types::TomlTableRef::from(parsed.as_ref())).get(),
    )
}
fn nearest_cargo_toml_path(path: types::PathRef<'_>) -> Option<types::OwnedPathBuf> {
    path.as_ref()
        .ancestors()
        .map(|ancestor| ancestor.join(constants_str::CARGO_TOML))
        .find(|cargo_toml_path| cargo_toml_path.exists())
        .map(types::OwnedPathBuf::from)
}
fn is_str_constants_source_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref() == std::path::Path::new(constants_str::STR_CONSTANTS_SRC_LIB_RS),
    )
}
fn is_test_crate(parsed: types::TomlTableRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        parsed
            .as_ref()
            .get(constants_str::PACKAGE)
            .and_then(toml::Value::as_table)
            .and_then(|package| package.get(constants_str::NAME))
            .and_then(toml::Value::as_str)
            .is_some_and(|name| constants_str::CODE_STYLE_TEST_CRATE_NAMES.contains(&name)),
    )
}
fn is_test_crate_source_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    if is_test_source_path(path).get() {
        return types::AnalyzerBool::from(true);
    }
    nearest_cargo_toml_path(path)
        .and_then(|cargo_toml_path| read_toml_table(types::PathRef::from(cargo_toml_path.as_ref())))
        .map_or_else(types::AnalyzerBool::default, |manifest| {
            is_test_crate(types::TomlTableRef::from(manifest.as_ref()))
        })
}
fn is_test_source_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .components()
            .any(|component| component.as_os_str() == constants_str::TESTS_ALT)
            || path
                .as_ref()
                .file_stem()
                .is_some_and(|file_stem| file_stem == constants_str::TESTS_ALT),
    )
}
fn is_non_policy_test_source_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        path.as_ref()
            .starts_with(constants_str::CODE_STYLE_TESTS_SRC_ROOT)
            && !path
                .as_ref()
                .starts_with(constants_str::TESTS_SRC_CODE_STYLE),
    )
}
fn is_direct_fs_owner_source_path(path: types::PathRef<'_>) -> types::AnalyzerBool {
    let path_text = path.as_ref().to_string_lossy();
    types::AnalyzerBool::from(
        constants_str::CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES
            .iter()
            .any(|suffix| path_text.ends_with(suffix)),
    )
}
#[allow(clippy::single_call_fn)] // proc-macro crates are allowed to panic by repository policy
fn is_proc_macro_crate(parsed: types::TomlTableRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(
        parsed
            .as_ref()
            .get(constants_str::LIB)
            .and_then(toml::Value::as_table)
            .and_then(|lib| lib.get(constants_str::PROC_MACRO))
            == Some(&toml::Value::Boolean(true)),
    )
}
fn has_test_only_cfg_attr(i: types::SynItemRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match i.as_ref() {
        syn::Item::Const(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Enum(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::ExternCrate(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Fn(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::ForeignMod(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Impl(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Macro(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Mod(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Static(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Struct(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Trait(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::TraitAlias(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Type(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Union(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Use(item) => item
            .attrs
            .iter()
            .any(|attr| attr_is_test_only_cfg(types::SynAttributeRef::from(attr)).get()),
        syn::Item::Verbatim(_) | _ => false,
    })
}
fn attr_is_test_only_cfg(attr: types::SynAttributeRef<'_>) -> types::AnalyzerBool {
    let attr_ref = attr.as_ref();
    if !attr_ref.path().is_ident(constants_str::CFG_ALT) {
        return types::AnalyzerBool::default();
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
    types::AnalyzerBool::from(is_test_only_cfg)
}
fn for_each_rs_file(mut on_file: impl FnMut(&snapshot::RsSourceFile)) {
    snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.rs_files().iter().for_each(&mut on_file);
    });
}
fn workspace_table_from_cargo_toml() -> types::TomlTable {
    let mut table = std::fs::read_to_string(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
        .expect("39a0d238 workspace_table_from_cargo_toml invariant must hold")
        .parse::<toml::Table>()
        .expect("beb11586 workspace_table_from_cargo_toml invariant must hold");
    toml_val_as_table(
        types::TomlValue::from(
            table
                .remove(constants_str::WORKSPACE)
                .expect("f728192d workspace_table_from_cargo_toml invariant must hold"),
        ),
        types::StaticStr::from(constants_str::VALUE_2BFB0B62),
    )
}
#[allow(clippy::single_call_fn)] // shared owned-value table extractor keeps table-shape validation reusable where ownership is required
fn toml_val_as_table(v: types::TomlValue, uuid: types::StaticStr) -> types::TomlTable {
    match v.into_inner() {
        toml::Value::Table(t) => types::TomlTable::from(t),
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => panic!("{}", uuid.get()),
    }
}
fn toml_val_as_table_ref(
    v: types::TomlValueRef<'_>,
    uuid: types::StaticStr,
) -> types::TomlTableRef<'_> {
    match v.get() {
        toml::Value::Table(t) => types::TomlTableRef::from(t),
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => panic!("{}", uuid.get()),
    }
}
fn collect_non_workspace_dep_ers(
    path: types::PathRef<'_>,
    parsed: types::TomlTableRef<'_>,
    mut ers: types::DiagnosticMsgsMutRef<'_>,
) {
    let root_dependency_tables = [
        constants_str::DEPENDENCIES,
        constants_str::DEV_DEPENDENCIES,
        constants_str::BUILD_DEPENDENCIES,
    ]
    .into_iter()
    .filter_map(|dep_section| {
        parsed
            .as_ref()
            .get(dep_section)
            .and_then(toml::Value::as_table)
            .map(|deps| (dep_section.to_owned(), deps))
    });
    let target_dependency_tables = parsed
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
    ers.extend(
        root_dependency_tables
            .chain(target_dependency_tables)
            .flat_map(|(dep_section, deps)| {
                deps.iter()
                    .filter(move |(_, dep_value)| {
                        !workspace_dep_entry_is_valid(types::TomlValueRef::from(*dep_value)).get()
                    })
                    .map(move |(dep_name, _)| {
                        String::from(workspace_dep_entry_error(
                            path,
                            types::SourceTextRef::from(dep_name.as_str()),
                            types::SourceTextRef::from(dep_section.as_str()),
                        ))
                    })
            }),
    );
}
#[allow(clippy::single_call_fn)] // keeps dependency-policy validation centralized for dependencies/dev-dependencies/build-dependencies checks
fn workspace_dep_entry_is_valid(dep_value: types::TomlValueRef<'_>) -> types::AnalyzerBool {
    types::AnalyzerBool::from(match dep_value.as_ref() {
        toml::Value::Table(dep_table) => {
            dep_table.get(constants_str::WORKSPACE) == Some(&toml::Value::Boolean(true))
        }
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => false,
    })
}
#[allow(clippy::single_call_fn)] // shared message builder keeps dependency-policy errors identical across call sites
fn workspace_dep_entry_error(
    path: types::PathRef<'_>,
    dep_name: types::SourceTextRef<'_>,
    dep_section: types::SourceTextRef<'_>,
) -> types::SourceText {
    types::SourceText::try_from(format!(
        "{}: dependency `{dep_name}` in [{dep_section}] must use `dep = {{ workspace = true }}`",
        path.as_ref().display(),
        dep_name = dep_name.as_ref(),
        dep_section = dep_section.as_ref(),
    ))
    .expect("c836ad25 workspace_dep_entry_error invariant must hold")
}
#[allow(clippy::single_call_fn)] // dedicated collector keeps workspace-members existence diagnostics reusable and deterministic with caller-managed sorting
fn collect_workspace_member_missing_cargo_toml_ers(
    members: types::SourceTextListRef<'_>,
) -> types::SourceTextList {
    types::SourceTextList::from(
        members
            .as_ref()
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
    )
}
fn workspace_members_as_strs(
    workspace: types::TomlTableRef<'_>,
    exp_id: types::StaticStr,
) -> types::SourceTextList {
    let Some(members) = workspace
        .as_ref()
        .get(constants_str::MEMBERS)
        .and_then(toml::Value::as_array)
    else {
        panic!("{}", exp_id.get());
    };
    members
        .iter()
        .map(|member| {
            member
                .as_str()
                .unwrap_or_else(|| panic!("{}", exp_id.get()))
                .to_owned()
        })
        .collect::<Vec<String>>()
        .into()
}
