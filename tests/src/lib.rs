#[cfg(test)]
use optml as _;

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        ffi::OsStr,
        fs::{self, read_to_string},
        io::ErrorKind,
        path::{Path, PathBuf},
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

    fn collect_files_recursively(directory_path: &Path, files: &mut Vec<PathBuf>) {
        let directory_entries = fs::read_dir(directory_path).expect("f1a27b8c");

        for directory_entry_result in directory_entries {
            let directory_entry = directory_entry_result.expect("d2e8c41a");
            let file_type = directory_entry.file_type().expect("8bf903de");
            let path = directory_entry.path();

            if file_type.is_dir() {
                if path.ends_with(".git") || path.ends_with("target") {
                    continue;
                }
                collect_files_recursively(&path, files);
                continue;
            }

            files.push(path);
        }
    }

    fn collect_workspace_files(workspace_root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        collect_files_recursively(workspace_root, &mut files);
        files
    }

    fn workspace_root_path() -> PathBuf {
        let crate_manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_manifest_directory
            .parent()
            .expect("a74d10bf")
            .to_path_buf()
    }

    fn non_test_source_segment(file_content: &str) -> &str {
        if let Some((before_test_segment, _after_test_segment)) =
            file_content.split_once("#[cfg(test)]")
        {
            return before_test_segment;
        }

        file_content
    }

    fn read_file(path: &Path) -> String {
        read_to_string(path).expect("9be35a17")
    }

    fn workflow_file_paths(workspace_root: &Path) -> Vec<PathBuf> {
        let workflows_directory_path = workspace_root.join(".github").join("workflows");
        fs::read_dir(&workflows_directory_path)
            .and_then(|directory_entries| {
                directory_entries
                    .map(|directory_entry_result| directory_entry_result.map(|entry| entry.path()))
                    .collect()
            })
            .or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    Ok(Vec::new())
                } else {
                    Err(error)
                }
            })
            .expect("7cf34bd1")
            .into_iter()
            .filter(|path| path.extension().is_some_and(|extension| extension == "yml"))
            .collect()
    }

    fn rust_source_files(workspace_files: &[PathBuf]) -> Vec<&PathBuf> {
        workspace_files
            .iter()
            .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("rs"))
            .collect()
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

    fn is_exception(path: &Path, exceptions: &[&str]) -> bool {
        exceptions.iter().any(|exception| path.ends_with(exception))
    }

    fn assert_root_workspace_cargo_policy(
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
                .filter(|el| !is_exception(el.path(), &ROOT_CARGO_TOML_EXCEPTIONS))
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

    fn str_set(items: &[String]) -> HashSet<&str> {
        items.iter().map(String::as_str).collect::<HashSet<&str>>()
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
            assert!(lints_not_in_cargo_toml.is_empty(), "1c5a9308 {lints_not_in_cargo_toml:?}");
            let outdated_lints_in_file =
                collect_missing_items(&lints_vec_from_cargo_toml, &lints_to_check_set);
            assert!(outdated_lints_in_file.is_empty(), "93787d2d");
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
            | &Value::Datetime(_) => panic!("27bcfb1c"),
        }
    }

    fn take_next_u64_part(iter: &mut Split<'_, char>) -> bool {
        iter.next()
            .and_then(|part| part.parse::<u64>().ok())
            .is_some()
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
        for_each_rs_file_content(|path, content| {
            let ast = parse_file(content).expect("5e7a83eb");
            let mut visitor = ExpectVisitor {
                method_name: expect_or_panic.method_name(),
                uuids: Vec::new(),
                ers: Vec::new(),
            };
            Visit::visit_file(&mut visitor, &ast);
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

    // --- Meta tests ---

    #[test]
    fn enforces_publish_false_in_all_crates() {
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
    fn enforces_workspace_lints_in_all_crates() {
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
    fn enforces_english_only_in_source_files() {
        let mut ers = Vec::new();
        let exceptions: [&str; 0] = [];
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
                continue;
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

    #[test]
    fn enforces_unique_uuid_in_expect_messages() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Expect);
    }

    #[test]
    fn enforces_unique_uuid_in_panic_messages() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Panic);
    }

    #[test]
    fn enforces_all_clippy_lints_in_workspace() {
        assert_workspace_lints_match(
            RustOrClippy::Clippy,
            "clippy-driver",
            true,
            "8895ca50",
            &CLIPPY_LINT_EXCEPTIONS,
        );
    }

    #[test]
    fn enforces_all_rust_lints_in_workspace() {
        assert_workspace_lints_match(RustOrClippy::Rust, "rustc", false, "3c20b457", &[
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
            "unqualified_local_imports",
            "unreachable_cfg_select_predicates",
            "default_overrides_default_fields",
            "linker_info",
            "duplicate_features",
            "deprecated_llvm_intrinsic",
            "tail_call_track_caller",
        ]);
    }

    #[test]
    fn enforces_unique_uuid_in_rs_files() {
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
    fn enforces_exact_version_in_workspace_dependencies() {
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

    #[test]
    fn enforces_workspace_members_exist_on_disk() {
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
        ers.sort();
        assert_joined_ers_empty(&ers, "a4e3b8d1");
    }

    #[test]
    fn enforces_workspace_members_sorted_alphabetically() {
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

    // --- Policy tests ---

    #[test]
    fn forbids_todo_and_unimplemented_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("todo!("),
                "found todo! in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("unimplemented!("),
                "found unimplemented! in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_panic_and_assert_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("panic!("),
                "found panic! in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("assert!("),
                "found assert! in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_source_dropping_map_err_pattern() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("map_err(|_|"),
                "found source-dropping map_err pattern in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_global_mutable_or_lazy_singleton_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("static mut"),
                "found static mut in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("lazy_static!"),
                "found lazy_static singleton in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("once_cell::sync::Lazy"),
                "found once_cell::sync::Lazy singleton in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("LazyLock::new("),
                "found LazyLock singleton in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_deterministic_test_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if !rust_file
                .components()
                .any(|component| component.as_os_str() == "tests")
            {
                continue;
            }
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }

            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("sleep("),
                "found sleep in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("SystemTime::now("),
                "found wall-clock time in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("Utc::now("),
                "found timezone-dependent call in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("Local::now("),
                "found local-time call in tests: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn requires_semver_check_and_hack_in_ci_and_changelog_file() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content.contains("cargo-semver-checks-action"),
            "missing semver check action in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content.contains("cargo hack check"),
            "missing cargo hack feature-matrix check in {}",
            ci_workflow_path.display()
        );

        let changelog_path = workspace_root.join("CHANGELOG.md");
        let changelog_content = read_file(&changelog_path);
        assert!(
            !changelog_content.trim().is_empty(),
            "CHANGELOG.md exists but is empty: {}",
            changelog_path.display()
        );
    }

    #[test]
    fn forbids_unwrap_and_error_masking_shortcuts_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("unwrap("),
                "found unwrap in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("unwrap_or_default("),
                "found unwrap_or_default in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("unwrap_or("),
                "found unwrap_or in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_expect_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("expect("),
                "found expect in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_direct_command_new_usage() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("Command::new("),
                "direct Command::new usage is forbidden: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_allow_lint_attributes_in_rust_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("#[allow("),
                "found #[allow(...)] attribute in {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("#![allow("),
                "found #![allow(...)] attribute in {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_dependency_declarations_for_member_crates() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);

        let member_manifest_paths = workspace_files
            .iter()
            .filter(|path| path.ends_with("Cargo.toml"))
            .filter(|path| {
                !path.ends_with("Cargo.toml") || *path != &workspace_root.join("Cargo.toml")
            })
            .filter(|path| path.starts_with(workspace_root.join("server")))
            .collect::<Vec<_>>();

        for manifest_path in member_manifest_paths {
            let manifest_content = read_file(manifest_path);
            let mut is_in_dependency_section = false;
            for line in manifest_content.lines() {
                let trimmed_line = line.trim();
                if trimmed_line.starts_with('[') {
                    is_in_dependency_section = trimmed_line == "[dependencies]"
                        || trimmed_line == "[dev-dependencies]"
                        || trimmed_line == "[build-dependencies]";
                    continue;
                }
                if !is_in_dependency_section
                    || trimmed_line.is_empty()
                    || trimmed_line.starts_with('#')
                {
                    continue;
                }
                assert!(
                    trimmed_line.contains(".workspace = true")
                        || trimmed_line.contains("workspace = true"),
                    "dependency entry is not workspace-based in {}: {}",
                    manifest_path.display(),
                    trimmed_line
                );
                assert!(
                    !trimmed_line.contains("version = "),
                    "member manifest must not pin crate versions directly in {}: {}",
                    manifest_path.display(),
                    trimmed_line
                );
            }
        }
    }

    #[test]
    fn requires_crates_io_dependencies_to_live_in_workspace_dependencies() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);
        let mut is_in_workspace_dependencies_section = false;

        for line in root_manifest_content.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with('[') {
                is_in_workspace_dependencies_section = trimmed_line == "[workspace.dependencies]";
                continue;
            }
            if !is_in_workspace_dependencies_section
                || trimmed_line.is_empty()
                || trimmed_line.starts_with('#')
            {
                continue;
            }
            if trimmed_line.contains("version = ") {
                assert!(
                    trimmed_line.contains("default-features = false"),
                    "workspace crates.io dependency must disable default features in {}: {}",
                    root_manifest_path.display(),
                    trimmed_line
                );
            }
        }
    }

    #[test]
    fn forbids_makefile_justfile_and_shell_scripts() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);

        assert!(
            !workspace_files
                .iter()
                .any(|path| path.file_name().is_some_and(|name| name == "Makefile")),
            "Makefile is forbidden by policy"
        );
        assert!(
            !workspace_files
                .iter()
                .any(|path| path.file_name().is_some_and(|name| name == "Justfile")),
            "Justfile is forbidden by policy"
        );
        assert!(
            !workspace_files
                .iter()
                .any(|path| path.extension().is_some_and(|extension| extension == "sh")),
            "shell script files (*.sh) are forbidden by policy"
        );
    }

    #[test]
    fn forbids_direct_index_zero_or_one_access_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            for line in file_content.lines() {
                let compact_line = line.replace(' ', "");
                assert!(
                    !compact_line.contains("[0]"),
                    "found forbidden [0] indexing pattern in {}: {}",
                    rust_file.display(),
                    line.trim()
                );
                assert!(
                    !compact_line.contains("[1]"),
                    "found forbidden [1] indexing pattern in {}: {}",
                    rust_file.display(),
                    line.trim()
                );
            }
        }
    }

    #[test]
    fn forbids_regular_loops_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);

            assert!(
                !source_segment.contains("\nfor "),
                "found regular for-loop in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("\nwhile "),
                "found regular while-loop in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_numeric_as_casts_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);
        let forbidden_as_patterns = [
            " as i8",
            " as i16",
            " as i32",
            " as i64",
            " as i128",
            " as isize",
            " as u8",
            " as u16",
            " as u32",
            " as u64",
            " as u128",
            " as usize",
            " as f32",
            " as f64",
        ];

        for rust_file in rust_files {
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            for forbidden_as_pattern in forbidden_as_patterns {
                assert!(
                    !source_segment.contains(forbidden_as_pattern),
                    "found numeric `as` cast in non-test code: {} pattern: {}",
                    rust_file.display(),
                    forbidden_as_pattern
                );
            }
        }
    }

    #[test]
    fn forbids_axum_from_fn_layer_usage() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains(".layer(from_fn("),
                "found forbidden axum from_fn layer in {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_hardened_release_profile_configuration() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);

        assert!(
            root_manifest_content.contains("[profile.release]"),
            "missing [profile.release] section in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("lto = \"fat\""),
            "release profile must enable fat LTO in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("codegen-units = 1"),
            "release profile must use codegen-units = 1 in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("panic = \"abort\""),
            "release profile must use panic = \"abort\" in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("strip = \"symbols\""),
            "release profile must strip symbols in {}",
            root_manifest_path.display()
        );
    }

    #[test]
    fn enforces_workspace_verify_alias_order() {
        let workspace_root = workspace_root_path();
        let cargo_config_path = workspace_root.join(".cargo").join("config.toml");
        let cargo_config_content = read_file(&cargo_config_path);
        let expected_verify_alias = "workspace-verify = \"!cargo fmt && cargo clippy \
                                     --all-targets --all-features -- -D warnings && cargo test\"";

        assert!(
            cargo_config_content.contains(expected_verify_alias),
            "workspace-verify alias must execute fmt -> clippy -> test in {}",
            cargo_config_path.display()
        );
    }

    #[test]
    fn enforces_nightly_toolchain_channel() {
        let workspace_root = workspace_root_path();
        let rust_toolchain_path = workspace_root.join("rust-toolchain.toml");
        let rust_toolchain_content = read_file(&rust_toolchain_path);

        assert!(
            rust_toolchain_content.contains("channel = \"nightly\""),
            "workspace template requires nightly toolchain in {}",
            rust_toolchain_path.display()
        );
    }

    #[test]
    fn forbids_debug_print_macros_in_non_test_code_except_entrypoint_output() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);
        let server_main_path = workspace_root.join("server").join("src").join("main.rs");

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            let path_is_server_main = rust_file == &server_main_path;
            if !path_is_server_main {
                assert!(
                    !source_segment.contains("println!("),
                    "found println! in non-test non-entrypoint code: {}",
                    rust_file.display()
                );
                assert!(
                    !source_segment.contains("eprintln!("),
                    "found eprintln! in non-test non-entrypoint code: {}",
                    rust_file.display()
                );
            }
            assert!(
                !source_segment.contains("dbg!("),
                "found dbg! in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_alias_set_for_daily_workflows() {
        let workspace_root = workspace_root_path();
        let cargo_config_path = workspace_root.join(".cargo").join("config.toml");
        let cargo_config_content = read_file(&cargo_config_path);

        let required_aliases = [
            "workspace-format = ",
            "workspace-lint = ",
            "workspace-test = ",
            "workspace-check-no-default-features = ",
            "workspace-doc = ",
            "workspace-nextest = ",
            "workspace-hack = ",
            "workspace-deny = ",
            "workspace-udeps = ",
            "workspace-verify = ",
        ];

        for required_alias in required_aliases {
            assert!(
                cargo_config_content.contains(required_alias),
                "missing required cargo alias `{}` in {}",
                required_alias,
                cargo_config_path.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_resolver_and_member_list_contract() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);

        assert!(
            root_manifest_content.contains("resolver = \"2\""),
            "workspace root must define resolver = \"2\" in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("\"server\""),
            "workspace members must include server crate in {}",
            root_manifest_path.display()
        );
    }

    #[test]
    fn forbids_server_from_direct_thiserror_dependency() {
        let workspace_root = workspace_root_path();
        let server_manifest_path = workspace_root.join("server").join("Cargo.toml");
        let server_manifest_content = read_file(&server_manifest_path);

        assert!(
            !server_manifest_content.contains("thiserror"),
            "server crate must not depend on thiserror directly in {}",
            server_manifest_path.display()
        );
    }

    #[test]
    fn enforces_fast_ci_test_commands_presence() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content
                .contains("cargo nextest run --all-targets --all-features --profile ci"),
            "fast CI must run nextest in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content.contains("cargo test --doc --all-features"),
            "fast CI must run doc tests in {}",
            ci_workflow_path.display()
        );
    }

    #[test]
    fn enforces_full_ci_mode_to_keep_baseline_quality_gates() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content.contains(
                "(needs.changed-files.outputs.fast == 'true' || needs.changed-files.outputs.full \
                 == 'true')"
            ),
            "CI must keep baseline jobs enabled for both fast and full modes in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content.contains("run: cargo fmt --check"),
            "CI must keep formatting gate in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content
                .contains("run: cargo clippy --all-targets --all-features -- -D warnings"),
            "CI must keep clippy gate in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content
                .contains("run: cargo nextest run --all-targets --all-features --profile ci"),
            "CI must keep test gate in {}",
            ci_workflow_path.display()
        );
    }

    #[test]
    fn enforces_documented_local_verification_order() {
        let workspace_root = workspace_root_path();
        let readme_path = workspace_root.join("README.md");
        let readme_content = read_file(&readme_path);
        let contributing_path = workspace_root.join("CONTRIBUTING.md");
        let contributing_content = read_file(&contributing_path);
        let required_verification_sequence =
            "cargo fmt\ncargo clippy --all-targets --all-features -- -D warnings\ncargo test";

        assert!(
            readme_content.contains(required_verification_sequence),
            "README must document fmt -> clippy -> test order in {}",
            readme_path.display()
        );
        assert!(
            contributing_content.contains(required_verification_sequence),
            "CONTRIBUTING must document fmt -> clippy -> test order in {}",
            contributing_path.display()
        );
    }

    #[test]
    fn enforces_workflow_level_minimal_permissions() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            assert!(
                workflow_content.contains("permissions:\n  contents: read"),
                "workflow must define top-level minimal permissions in {}",
                workflow_file_path.display()
            );
        }
    }

    #[test]
    fn enforces_workflow_concurrency_cancellation() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            assert!(
                workflow_content.contains("cancel-in-progress: true"),
                "workflow must enable cancel-in-progress in {}",
                workflow_file_path.display()
            );
        }
    }

    #[test]
    fn enforces_timeout_minutes_for_each_workflow_job() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            let runs_on_count = workflow_content.matches("runs-on:").count();
            let timeout_minutes_count = workflow_content.matches("timeout-minutes:").count();
            assert!(
                runs_on_count == timeout_minutes_count,
                "every workflow job must define timeout-minutes in {}: runs-on={}, \
                 timeout-minutes={}",
                workflow_file_path.display(),
                runs_on_count,
                timeout_minutes_count
            );
        }
    }

    #[test]
    fn enforces_marketplace_actions_to_be_pinned_by_full_commit_sha() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            for workflow_line in workflow_content.lines() {
                let Some((_, action_reference_value)) = workflow_line.split_once("uses:") else {
                    continue;
                };
                let action_reference = action_reference_value.trim();
                if action_reference.starts_with("./") || action_reference.starts_with("docker://") {
                    continue;
                }
                let Some((action_name, action_version_reference)) =
                    action_reference.split_once('@')
                else {
                    continue;
                };
                if !action_name.contains('/') {
                    continue;
                }
                assert!(
                    action_version_reference.len() == 40
                        && action_version_reference
                            .chars()
                            .all(|character| character.is_ascii_hexdigit()),
                    "GitHub Action must be pinned by full 40-char SHA in {}: {}",
                    workflow_file_path.display(),
                    workflow_line.trim()
                );
            }
        }
    }

    #[test]
    fn forbids_public_struct_fields_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if rust_file.ends_with("tests/src/lib.rs") {
                continue;
            }

            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            let mut is_inside_public_struct = false;
            let mut public_struct_brace_depth = 0usize;

            for source_line in source_segment.lines() {
                let trimmed_line = source_line.trim_start();
                if !is_inside_public_struct
                    && trimmed_line.starts_with("pub struct ")
                    && trimmed_line.contains('{')
                {
                    is_inside_public_struct = true;
                    public_struct_brace_depth = source_line.matches('{').count();
                    public_struct_brace_depth =
                        public_struct_brace_depth.saturating_sub(source_line.matches('}').count());
                    continue;
                }

                if is_inside_public_struct {
                    assert!(
                        !trimmed_line.starts_with("pub "),
                        "found public struct field in non-test code: {}: {}",
                        rust_file.display(),
                        source_line.trim()
                    );

                    public_struct_brace_depth += source_line.matches('{').count();
                    public_struct_brace_depth =
                        public_struct_brace_depth.saturating_sub(source_line.matches('}').count());
                    if public_struct_brace_depth == 0 {
                        is_inside_public_struct = false;
                    }
                }
            }
        }
    }

    #[test]
    fn forbids_placeholder_repository_metadata_in_workspace_crates() {
        let workspace_root = workspace_root_path();
        let manifest_paths = [workspace_root.join("server").join("Cargo.toml")];

        for manifest_path in manifest_paths {
            let manifest_content = read_file(&manifest_path);
            assert!(
                !manifest_content.contains("repository = \"https://github.com/user/repo\""),
                "placeholder repository metadata must be replaced in {}",
                manifest_path.display()
            );
        }
    }

    #[test]
    fn enforces_nightly_full_test_run_in_ci_for_harness_parity() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content
                .contains("cargo +nightly test --workspace --all-targets --all-features"),
            "CI must run nightly full cargo test for harness parity in {}",
            ci_workflow_path.display()
        );
    }
}
