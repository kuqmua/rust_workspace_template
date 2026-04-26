#[cfg(test)]
use optml as _;

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        ffi::OsStr,
        fs::read_to_string,
        path::Path,
        process::{Command, Stdio},
        str::Split,
    };

    use regex::Regex;
    use syn::{
        Expr, ExprLit, ExprMethodCall, Lit, parse_file,
        visit::{Visit, visit_expr_method_call},
    };
    use toml::{Table as TomlTable, Value, value::Table};
    use uuid::Uuid;
    use walkdir::WalkDir;
    const ROOT_CARGO_TOML_EXCEPTIONS: [&str; 1] = ["../Cargo.toml"];
    const CLIPPY_LINT_EXCEPTIONS: [&str; 22] = [
        "disallowed_fields",
        "unnecessary_trailing_comma",
        "manual_pop_if",
        "assign_ops",
        "extend_from_slice",
        "match_on_vec_items",
        "misaligned_transmute",
        "option_map_or_err_ok",
        "pub_enum_variant_names",
        "range_step_by_zero",
        "regex_macro",
        "replace_consts",
        "should_assert_eq",
        "string_to_string",
        "unsafe_vector_initialization",
        "unstable_as_mut_slice",
        "unstable_as_slice",
        "unused_collect",
        "wrong_pub_self_convention",
        "manual_noop_waker",
        "manual_option_zip",
        "useless_borrows_in_formatting",
    ];
    #[derive(Debug, Clone, Copy)]
    enum ExpectOrPanic {
        Expect,
        Panic,
    }
    impl ExpectOrPanic {
        const fn method_name(self) -> &'static str {
            match self {
                Self::Expect => "expect",
                Self::Panic => "panic",
            }
        }
    }
    #[derive(Debug, Clone, Copy)]
    enum RustOrClippy {
        Clippy,
        Rust,
    }
    impl RustOrClippy {
        fn name(&self) -> &str {
            match *self {
                Self::Rust => "rust",
                Self::Clippy => "clippy",
            }
        }
    }
    struct DbgVisitor {
        found: bool,
    }
    impl<'ast> Visit<'ast> for DbgVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if i.path
                .segments
                .last()
                .is_some_and(|v_4b8e1c7a| v_4b8e1c7a.ident == "dbg")
            {
                self.found = true;
            }
        }
    }
    struct TodoUnimplVisitor {
        todo_found: usize,
        unimplemented_found: usize,
    }
    impl<'ast> Visit<'ast> for TodoUnimplVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if let Some(last_segment) = i.path.segments.last() {
                match () {
                    () if last_segment.ident == "todo" => {
                        self.todo_found = self.todo_found.saturating_add(1);
                    }
                    () if last_segment.ident == "unimplemented" => {
                        self.unimplemented_found = self.unimplemented_found.saturating_add(1);
                    }
                    () => {}
                }
            }
        }
    }
    struct UnwrapVisitor {
        found_count: usize,
    }
    impl<'ast> Visit<'ast> for UnwrapVisitor {
        fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
            if i.method == "unwrap" && i.args.is_empty() {
                self.found_count = self.found_count.saturating_add(1);
            }
            visit_expr_method_call(self, i);
        }
    }
    #[test]
    fn all_crates_have_publish_false() {
        assert_root_workspace_cargo_policy("f2a8c5d3", |path, parsed, ers| {
            let publish = parsed
                .get("package")
                .and_then(|v_1c7b4e9d| v_1c7b4e9d.get("publish"));
            if publish != Some(&Value::Boolean(false)) {
                ers.push(format!("{}: missing `publish = false`", path.display()));
            }
        });
    }
    #[test]
    fn all_crates_have_workspace_lints() {
        assert_root_workspace_cargo_policy("d5f1a4e7", |path, parsed, ers| {
            match parsed
                .get("lints")
                .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
            {
                Some(lints_tbl) => {
                    if lints_tbl.get("workspace") != Some(&Value::Boolean(true)) {
                        ers.push(format!("{}: [lints] missing `workspace = true`", path.display()));
                    }
                }
                None => {
                    ers.push(format!("{}: missing [lints] section", path.display()));
                }
            }
        });
    }
    #[test]
    fn all_files_are_english_only() {
        let mut ers = Vec::new();
        let exceptions = [
            "../pg_crud/pg_crud_cmn/src/lib.rs", // contain utf-8 String test
            "../CODE_IMPROVEMENT_PLAN.md",
            "../DEVELOPMENT_PLAN.md",
        ];
        for el_d87f0495 in project_dir()
            .into_iter()
            .filter_entry(|el_6870bc3d| !is_ignored_dir_entry_name(el_6870bc3d.file_name()))
            .filter_map(Result::ok)
        {
            let path = el_d87f0495.path();
            if !(path.is_file()
                && matches!(
                    path.extension().and_then(OsStr::to_str),
                    Some("rs" | "toml" | "md" | "txt" | "yml" | "yaml" | "json")
                ))
            {
                continue;
            }
            if is_exception(path, &exceptions) {
                continue;
            }
            let Ok(content) = read_to_string(path) else {
                continue; //skip binary non-utf8 files
            };
            ers.extend({
                let mut collected = Vec::new();
                for (line_idx, line) in content.lines().enumerate() {
                    let line_number = line_idx.saturating_add(1);
                    for ch in line.chars() {
                        if !(matches!(ch, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}')
                            || ch.is_ascii())
                        {
                            collected.push(format!(
                                "{}:{} non-english symbol `{}` (U+{:04X})",
                                path.display(),
                                line_number,
                                ch,
                                u32::from(ch)
                            ));
                        }
                    }
                }
                collected
            });
        }
        assert_joined_ers_empty_with_ctx(&ers, "8db37a2f", "non-english symbols:");
    }
    fn check_expect_or_panic_contains_only_unq_uuid_v4(expect_or_panic: ExpectOrPanic) {
        struct ExpectVisitor {
            ers: Vec<String>,
            method_name: &'static str,
            uuids: Vec<String>,
        }
        impl<'ast> Visit<'ast> for ExpectVisitor {
            fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
                if i.method == self.method_name {
                    if i.args.len() == 1 {
                        if let Some(Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        })) = i.args.first()
                        {
                            let value = lit_str.value();
                            if value.len() == 8 {
                                self.uuids.push(value);
                            } else {
                                self.ers.push(format!("arg len is not 8: {value}"));
                            }
                        } else {
                            self.ers.push("arg is not string literal".to_owned());
                        }
                    } else {
                        self.ers.push("with != 1 arg".to_owned());
                    }
                }
                visit_expr_method_call(self, i);
            }
        }
        let mut all_uuids = Vec::new();
        let mut all_ers = Vec::new();
        for_each_rs_syn_file(|path, ast| {
            let visitor = visit_syn_file(ast, ExpectVisitor {
                method_name: expect_or_panic.method_name(),
                uuids: Vec::new(),
                ers: Vec::new(),
            });
            all_uuids.extend(visitor.uuids);
            all_ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|el_2b9891bd| format!("{path:?}: {el_2b9891bd}")),
            );
        });
        let duplicates = {
            let mut seen = HashSet::new();
            let mut duplicates = Vec::new();
            for el_45f4b8bc in &all_uuids {
                if !seen.insert(el_45f4b8bc.as_str()) {
                    duplicates.push(el_45f4b8bc.to_owned());
                }
            }
            duplicates
        };
        if !duplicates.is_empty() {
            all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}");
    }
    #[test]
    fn check_expect_contains_only_unq_uuid_v4() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Expect);
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
        assert_workspace_lints_match(
            RustOrClippy::Clippy,
            "clippy-driver",
            true,
            "8895ca50",
            &CLIPPY_LINT_EXCEPTIONS,
        );
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
        assert_workspace_lints_match(
            RustOrClippy::Rust,
            "rustc",
            false,
            "3c20b457",
            // todo on commit momment seems like this lints still not added to rustc, but in the
            // list of rustc -W help
            &[
                "fuzzy_provenance_casts",
                "lossy_provenance_casts",
                "multiple_supertrait_upcastable",
                "must_not_suspend",
                "non_exhaustive_omitted_patterns",
                "supertrait_item_shadowing_definition",
                "supertrait_item_shadowing_usage",
                "aarch_64_softfloat_neon",
                "dflt_overrides_dflt_fields",
                "test_unstable_lint",
                "resolving_to_items_shadowing_supertrait_items",
                "shadowing_supertrait_items",
                "unqualified_local_imports", /* need to use some kind of different test flag or
                                              * something for this */
                "unreachable_cfg_select_predicates",
                "default_overrides_default_fields",
                "linker_info",
                "duplicate_features",
                "deprecated_llvm_intrinsic",
                "tail_call_track_caller",
            ],
        );
    }
    fn assert_workspace_lints_match(
        rust_or_clippy: RustOrClippy,
        tool: &str,
        parse_only_clippy: bool,
        exp_id: &'static str,
        exceptions: &[&str],
    ) {
        let lints_vec_from_cargo_toml = {
            let workspace = workspace_tbl_from_cargo_toml();
            let lints = toml_val_as_tbl_ref(workspace.get("lints").expect("82eaea37"), "cae226cd");
            let toml_v_tbl = toml_val_as_tbl_ref(
                lints.get(rust_or_clippy.name()).expect("dbd02f72"),
                "6f4580ce",
            );
            toml_v_tbl.keys().cloned().collect::<Vec<String>>()
        };
        let lints_from_cmd = {
            let output = Command::new(tool)
                .args(["-W", "help"])
                .stdout(Stdio::piped())
                .output()
                .unwrap_or_else(|_| panic!("{exp_id}"));
            assert!(output.status.success(), "95d4595a");
            let stderr = String::from_utf8(output.stderr.clone()).expect("3c1d9f87");
            assert!(stderr.trim().is_empty(), "cc4670a2");
            let stdout = String::from_utf8(output.stdout).expect("5ef7b23a");
            let regex = if parse_only_clippy {
                Regex::new(r"(?m)^\s*clippy::([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                    .expect("fbf14346")
            } else {
                Regex::new(r"(?m)^\s*([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                    .expect("60d99c87")
            };
            regex
                .captures_iter(&stdout)
                .map(|el_70833f93| {
                    el_70833f93
                        .get(1)
                        .expect("4f9c2e87")
                        .as_str()
                        .replace('-', "_")
                })
                .collect::<Vec<String>>()
        };
        {
            let rust_or_clippy_name = rust_or_clippy.name();
            let lints_from_cargo_set = str_set(&lints_vec_from_cargo_toml);
            let lints_to_check_set = str_set(&lints_from_cmd);
            let lints_exceptions_set = exceptions.iter().copied().collect::<HashSet<&str>>();
            let (lints_not_in_cargo_toml, lints_missing_by_exception) = {
                let mut lints_not_in_cargo_toml = Vec::new();
                let mut lints_missing_by_exception = Vec::new();
                for lint in collect_missing_items(&lints_from_cmd, &lints_from_cargo_set) {
                    if lints_exceptions_set.contains(lint) {
                        lints_missing_by_exception.push(lint);
                    } else {
                        lints_not_in_cargo_toml.push(lint);
                    }
                }
                (lints_not_in_cargo_toml, lints_missing_by_exception)
            };
            for lint in lints_missing_by_exception {
                println!(
                    "todo!() {rust_or_clippy_name} {lint} 158b5c43-05fa-4b8f-b6fe-9cda49d26997"
                );
            }
            assert!(lints_not_in_cargo_toml.is_empty(), "d2b7ba9f {lints_not_in_cargo_toml:?}");
            let outdated_lints_in_file =
                collect_missing_items(&lints_vec_from_cargo_toml, &lints_to_check_set);
            assert!(outdated_lints_in_file.is_empty(), "93787d2d");
        }
    }
    #[test]
    fn check_panic_contains_only_unq_uuid_v4() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Panic);
    }
    #[test]
    fn check_rs_files_contains_only_unq_uuid_v4() {
        let rgx = Regex::new(
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b"
        ).expect("e098a1ff");
        let mut seen = HashSet::new();
        for_each_rs_file_content(|_, content| {
            for el_714b3d9c in rgx.find_iter(content) {
                let uuid = Uuid::parse_str(el_714b3d9c.as_str()).expect("c9711efd");
                assert!(uuid.get_version_num() == 4, "49b49b21");
                assert!(seen.insert(uuid), "4cf9d239");
            }
        });
    }
    #[test]
    fn check_workspace_dependencies_having_exact_version() {
        let workspace = workspace_tbl_from_cargo_toml();
        for (_, v_5c36cb98) in
            toml_val_as_tbl_ref(workspace.get("dependencies").expect("2376f58e"), "e117fa5a")
        {
            {
                let v_tbl = toml_val_as_tbl_ref(v_5c36cb98, "cb693a3f");
                if let Some(path_v) = v_tbl.get("path") {
                    match path_v {
                        Value::String(_) => {}
                        Value::Table(_)
                        | Value::Integer(_)
                        | Value::Float(_)
                        | Value::Boolean(_)
                        | Value::Datetime(_)
                        | Value::Array(_) => panic!("6ca03a1f"),
                    }
                } else {
                    match v_tbl.get("version").expect("d5b2b269") {
                        Value::String(version_string) => {
                            assert!(
                                version_string.strip_prefix('=').is_some_and(|rest| {
                                    let mut iter = rest.split('.');
                                    take_next_u64_part(&mut iter)
                                        && take_next_u64_part(&mut iter)
                                        && take_next_u64_part(&mut iter)
                                        && iter.next().is_none()
                                }),
                                "6640b9bf"
                            );
                        }
                        Value::Table(_)
                        | Value::Integer(_)
                        | Value::Float(_)
                        | Value::Boolean(_)
                        | Value::Datetime(_)
                        | Value::Array(_) => panic!("a3410a37"),
                    }
                    match v_tbl.len() {
                        1 => {}
                        2 => {
                            if v_tbl.contains_key("features") {
                                validate_workspace_dep_features(v_tbl);
                            }
                        }
                        3 => {
                            validate_workspace_dep_features(v_tbl);
                            match v_tbl.get("default-features").expect("847a138f") {
                                &Value::Boolean(_) => (),
                                &Value::String(_)
                                | &Value::Table(_)
                                | &Value::Integer(_)
                                | &Value::Float(_)
                                | &Value::Datetime(_)
                                | &Value::Array(_) => panic!("b320164b"),
                            }
                        }
                        _ => panic!("f1139378 {v_tbl:#?}"),
                    }
                }
            }
        }
    }
    fn validate_workspace_dep_features(v_tbl: &Table) {
        match v_tbl.get("features").expect("473577d5") {
            &Value::Array(_) => (),
            &Value::String(_)
            | &Value::Table(_)
            | &Value::Integer(_)
            | &Value::Float(_)
            | &Value::Boolean(_)
            | &Value::Datetime(_) => panic!("38ba32e9"),
        }
    }
    fn take_next_u64_part(iter: &mut Split<'_, char>) -> bool {
        iter.next()
            .and_then(|part| part.parse::<u64>().ok())
            .is_some()
    }
    fn env_keys_from_file(path: &str) -> Vec<String> {
        read_to_string(path)
            .expect("b3a7c1e4")
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    return None;
                }
                trimmed.split_once('=').map(|(key, _)| key)
            })
            .map(str::to_owned)
            .collect()
    }
    #[test]
    fn env_and_envexample_have_same_keys() {
        let env_keys = env_keys_from_file("../server/.env");
        let example_keys = env_keys_from_file("../server/.envexample");
        let env_keys_set = str_set(&env_keys);
        let example_keys_set = str_set(&example_keys);
        let mut ers = collect_missing_key_ers(&env_keys, &example_keys_set, ".env", ".envexample");
        ers.extend(collect_missing_key_ers(&example_keys, &env_keys_set, ".envexample", ".env"));
        assert_joined_ers_empty_sorted(&mut ers, "c8d2f1a3");
    }
    fn collect_missing_items<'items>(
        items: &'items [String],
        present_set: &HashSet<&str>,
    ) -> Vec<&'items str> {
        items
            .iter()
            .map(String::as_str)
            .filter(|item| !present_set.contains(item))
            .collect::<Vec<&str>>()
    }
    fn collect_missing_key_ers(
        source_keys: &[String],
        target_set: &HashSet<&str>,
        source_file: &str,
        target_file: &str,
    ) -> Vec<String> {
        collect_missing_items(source_keys, target_set)
            .into_iter()
            .map(|key| format!("key `{key}` in {source_file} but missing from {target_file}"))
            .collect::<Vec<String>>()
    }
    fn is_exception(path: &Path, exceptions: &[&str]) -> bool {
        exceptions.iter().any(|exception| path.ends_with(exception))
    }
    fn assert_cargo_toml_ers_empty(
        exceptions: &[&str],
        exp_id: &'static str,
        mut mk_ers: impl FnMut(&Path, &TomlTable, &mut Vec<String>),
    ) {
        let ers = {
            let mut collected_ers = Vec::new();
            for entry in project_dir()
                .into_iter()
                .filter_entry(|el| !is_ignored_dir_entry_name(el.file_name()))
                .filter_map(Result::ok)
                .filter(|el| el.file_name() == "Cargo.toml")
                .filter(|el| !is_exception(el.path(), exceptions))
            {
                let path = entry.path();
                let parsed = match read_to_string(path) {
                    Ok(content) => match content.parse::<TomlTable>() {
                        Ok(table) => table,
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                };
                mk_ers(path, &parsed, &mut collected_ers);
            }
            collected_ers
        };
        assert_joined_ers_empty(&ers, exp_id);
    }
    fn assert_root_workspace_cargo_policy(
        exp_id: &'static str,
        mut mk_ers: impl FnMut(&Path, &TomlTable, &mut Vec<String>),
    ) {
        assert_cargo_toml_ers_empty(&ROOT_CARGO_TOML_EXCEPTIONS, exp_id, |path, parsed, ers| {
            mk_ers(path, parsed, ers);
        });
    }
    fn assert_joined_ers_empty(ers: &[String], exp_id: &'static str) {
        assert_joined_ers_empty_with_ctx(ers, exp_id, "");
    }
    fn assert_joined_ers_empty_with_ctx(ers: &[String], exp_id: &'static str, ctx: &str) {
        if ctx.is_empty() {
            assert!(ers.is_empty(), "{exp_id}\n{}", ers.join("\n"));
        } else {
            assert!(ers.is_empty(), "{exp_id} {ctx}\n{}", ers.join("\n"));
        }
    }
    fn assert_joined_ers_empty_sorted(ers: &mut [String], exp_id: &'static str) {
        ers.sort();
        assert_joined_ers_empty(ers, exp_id);
    }
    fn str_set(items: &[String]) -> HashSet<&str> {
        items.iter().map(String::as_str).collect::<HashSet<&str>>()
    }
    fn visit_syn_file<V>(ast: &syn::File, mut visitor: V) -> V
    where
        V: for<'ast> Visit<'ast>,
    {
        Visit::visit_file(&mut visitor, ast);
        visitor
    }
    fn assert_rs_ast_ers_empty_with_ctx(
        exp_id: &'static str,
        ctx: &str,
        mut mk_ers: impl FnMut(&Path, &syn::File, &mut Vec<String>),
    ) {
        let ers = {
            let mut collected_ers = Vec::new();
            for_each_rs_syn_file(|path, ast| {
                mk_ers(path, ast, &mut collected_ers);
            });
            collected_ers
        };
        assert_joined_ers_empty_with_ctx(&ers, exp_id, ctx);
    }
    #[test]
    fn no_dbg_macro_in_source_code() {
        assert_rs_ast_ers_empty_with_ctx("f1c7a4e3", "dbg!() found:", |path, ast, ers| {
            let visitor = visit_syn_file(ast, DbgVisitor { found: false });
            if visitor.found {
                ers.push(format!("{}: contains dbg!()", path.display()));
            }
        });
    }
    #[test]
    fn no_todo_or_unimplemented_macro_in_source_code() {
        assert_rs_ast_ers_empty_with_ctx(
            "c4e9a2d7",
            "todo!/unimplemented! found:",
            |path, ast, ers| {
                let visitor = visit_syn_file(ast, TodoUnimplVisitor {
                    todo_found: 0,
                    unimplemented_found: 0,
                });
                push_repeated_file_er(ers, path, "contains todo!()", visitor.todo_found);
                push_repeated_file_er(
                    ers,
                    path,
                    "contains unimplemented!()",
                    visitor.unimplemented_found,
                );
            },
        );
    }
    #[test]
    fn no_unwrap_in_source_code() {
        assert_rs_ast_ers_empty_with_ctx("e8b3a6d2", "unwrap() found:", |path, ast, ers| {
            let visitor = visit_syn_file(ast, UnwrapVisitor { found_count: 0 });
            push_repeated_file_er(ers, path, "unwrap() call", visitor.found_count);
        });
    }
    fn push_repeated_file_er(ers: &mut Vec<String>, path: &Path, msg: &str, times: usize) {
        for _ in 0..times {
            ers.push(format!("{}: {msg}", path.display()));
        }
    }
    fn project_dir() -> WalkDir {
        WalkDir::new("../")
    }
    fn is_ignored_dir_entry_name(name: &OsStr) -> bool {
        name == "target" || name == ".git"
    }
    fn for_each_rs_file_content(mut on_file: impl FnMut(&Path, &str)) {
        for entry in project_dir()
            .into_iter()
            .filter_entry(|el| {
                !is_ignored_dir_entry_name(el.file_name())
                    && (el.file_type().is_dir()
                        || el.path().extension().and_then(OsStr::to_str) == Some("rs"))
            })
            .filter_map(Result::ok)
            .filter(|el| el.path().extension().and_then(OsStr::to_str) == Some("rs"))
        {
            let path = entry.path();
            let Ok(content) = read_to_string(path) else {
                continue;
            };
            on_file(path, &content);
        }
    }
    fn for_each_rs_syn_file(mut on_file: impl FnMut(&Path, &syn::File)) {
        for_each_rs_file_content(|path, content| {
            let ast = parse_file(content).expect("5e7a83eb");
            on_file(path, &ast);
        });
    }
    fn workspace_tbl_from_cargo_toml() -> Table {
        let mut tbl = read_to_string("../Cargo.toml")
            .expect("39a0d238")
            .parse::<TomlTable>()
            .expect("beb11586");
        match tbl.remove("workspace").expect("f728192d") {
            Value::Table(table) => table,
            Value::String(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Array(_) => panic!("2bfb0b62"),
        }
    }
    fn toml_val_as_tbl_ref<'value_lt>(value: &'value_lt Value, uuid: &str) -> &'value_lt Table {
        match value {
            Value::Table(table) => table,
            Value::String(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Array(_) => panic!("{uuid}"),
        }
    }
    #[test]
    fn workspace_crates_must_use_workspace_dependencies() {
        assert_cargo_toml_ers_empty(
            &[
                "../Cargo.toml", // workspace
            ],
            "5f8a6d17",
            |path: &Path, parsed: &TomlTable, ers: &mut Vec<String>| {
                for dep_section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                    if let Some(deps) = parsed.get(dep_section).and_then(Value::as_table) {
                        for (dep_name, dep_value) in deps {
                            if !match dep_value {
                                Value::Table(dep_tbl) => {
                                    dep_tbl.contains_key("path")
                                        || dep_tbl.get("workspace") == Some(&Value::Boolean(true))
                                }
                                Value::String(_)
                                | Value::Integer(_)
                                | Value::Float(_)
                                | Value::Boolean(_)
                                | Value::Datetime(_)
                                | Value::Array(_) => false,
                            } {
                                ers.push(format!(
                                    "{}: dependency `{dep_name}` in [{dep_section}] must use \
                                     `.workspace = true` (only `path = ...` is allowed as \
                                     exception)",
                                    path.display(),
                                ));
                            }
                        }
                    }
                }
            },
        );
    }
    #[test]
    fn workspace_members_exist_on_disk() {
        let workspace = workspace_tbl_from_cargo_toml();
        let members = workspace_members_as_strs(&workspace, "7f3a1c4e");
        let mut ers = {
            let mut collected = Vec::new();
            for member_str in members {
                let member_path = Path::new("..").join(member_str).join("Cargo.toml");
                if !member_path.exists() {
                    collected.push(format!(
                        "member `{member_str}` Cargo.toml not found at {}",
                        member_path.display()
                    ));
                }
            }
            collected
        };
        assert_joined_ers_empty_sorted(&mut ers, "a4e3b8d1");
    }
    #[test]
    fn workspace_members_sorted_alphabetically() {
        let workspace = workspace_tbl_from_cargo_toml();
        let members_vec = workspace_members_as_strs(&workspace, "c1d4f7a2");
        let mut sorted = members_vec.clone();
        sorted.sort_unstable();
        let mut ers = Vec::new();
        for (k_4b1e6a8c, (got, expected)) in members_vec.iter().zip(sorted.iter()).enumerate() {
            if got != expected {
                ers.push(format!("index {k_4b1e6a8c}: got `{got}`, expected `{expected}`"));
            }
        }
        assert_joined_ers_empty_with_ctx(&ers, "b7c2e5f8", "members not sorted:");
    }
    fn workspace_members_as_strs<'members_lt>(
        workspace: &'members_lt Table,
        exp_id: &'static str,
    ) -> Vec<&'members_lt str> {
        let Some(members) = workspace.get("members").and_then(Value::as_array) else {
            panic!("{exp_id}");
        };
        let mut output = Vec::with_capacity(members.len());
        for member in members {
            match member.as_str() {
                Some(member_str) => output.push(member_str),
                None => panic!("{exp_id}"),
            }
        }
        output
    }
}
