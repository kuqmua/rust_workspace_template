#[test]
fn string_wrappers_do_not_use_from_string() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("e2a6b9c4"),
        super::types::SourceTextRef::from(
            "string wrappers must validate length; use TryFrom<String> with a length check instead of From<String>:",
        ),
        |path, ast, ers| {
            let string_wrapper_names =
                super::string_wrapper_names(super::types::SynFileRef::from(ast));
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::StringWrapperFromVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    string_wrapper_names: &string_wrapper_names,
                    try_from_string_names: super::types::StdSourceTextSet::default(),
                    try_from_string_len_checked_names: super::types::StdSourceTextSet::default(),
                },
            );
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.try_from_string_names.contains(name) {
                            None
                        } else {
                            Some(format!(
                                "{}: string wrapper `{name}` must implement `TryFrom<String>` with a length check",
                                path.display()
                            ))
                        }
                    }));
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.try_from_string_len_checked_names.contains(name) {
                            None
                        } else {
                            Some(format!(
                                "{}: string wrapper `{name}` implements `TryFrom<String>` without a `.len()` check",
                                path.display()
                            ))
                        }
                    }));
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
fn from_string_impl_visitor_rejects_non_string_wrappers_too() {
    let ast = syn::parse_file(
        "
struct SourceText(Box<str>);
impl From<String> for SourceText {
    fn from(value: String) -> Self {
        Self(value.into_boxed_str())
    }
}
",
    )
    .expect("f7c0e2a9");
    let string_wrapper_names = super::types::StdSourceTextSet::default();
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::StdSourceTextSet::default(),
            try_from_string_len_checked_names: super::types::StdSourceTextSet::default(),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 1, "a06d3c4f {ers:#?}");
    assert!(
        ers.iter()
            .any(|er| er.contains("`SourceText` implements `From<String>`")),
        "b19e40c8 {ers:#?}"
    );
}
#[test]
fn public_tuple_wrappers_do_not_expose_inner_field() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("b7c84e2a"),
        super::types::SourceTextRef::from(
            "public tuple wrappers must not expose inner fields; initialize them through From/TryFrom:",
        ),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::PublicTupleWrapperFieldVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
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
fn domain_boundaries_use_repository_declared_types() {
    let repo_crates = super::workspace_crate_names();
    let repo_types = super::declared_domain_type_names();
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("a7f9c3e1"),
        super::types::SourceTextRef::from(
            "raw external or primitive types found in domain boundaries; use repository domain wrapper types initialized with From/TryFrom:",
        ),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::DomainTypePolicyVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    generic_scopes: Vec::new(),
                    repo_crates: super::types::StdStdSourceTextSetRef::from(repo_crates.as_ref()),
                    repo_types: super::types::StdStdSourceTextSetRef::from(repo_types.as_ref()),
                },
            );
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
fn analyzer_state_struct_fields_use_repository_declared_wrappers() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("f2c7a91b"),
        super::types::SourceTextRef::from(
            "raw text containers found in helper struct fields; use repository wrapper types:",
        ),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::AnalyzerStateRawContainerFieldVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
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
fn analyzer_state_raw_container_field_visitor_reports_helper_fields() {
    let ast = syn::parse_file(
        "
struct HelperState {
    names: Vec<String>,
    seen: std::collections::BTreeSet<String>,
    refs: Option<std::collections::HashSet<&'static str>>,
    wrapped: types::SourceTextList,
}
struct SourceTextList(Vec<String>);
",
    )
    .expect("9f4d2a7c");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::AnalyzerStateRawContainerFieldVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 3, "2c0b7e91 {ers:#?}");
    assert!(
        ers.iter()
            .any(|er| er.contains("field `names` uses `Vec<String>`")),
        "74e18b2d {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|er| er.contains("field `seen` uses `BTreeSet<String>`")),
        "4a0df351 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|er| er.contains("field `refs` uses `HashSet<&str>`")),
        "81c6a2ef {ers:#?}"
    );
}
#[test]
fn helper_return_types_use_repository_declared_text_wrappers() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("6d41c8e2"),
        super::types::SourceTextRef::from(
            "raw text return types found in helper functions; use repository wrapper types:",
        ),
        |path, ast, ers| {
            if !super::is_code_style_meta_harness_source_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::HelperRawTextReturnVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
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
fn helper_raw_text_return_visitor_reports_free_and_inherent_helpers() {
    let ast = syn::parse_file(
        "
fn direct() -> String {
    String::new()
}
fn list() -> Vec<String> {
    Vec::new()
}
fn optional() -> Option<&'static str> {
    None
}
struct Helper;
impl Helper {
    fn nested() -> Result<types::SourceText, String> {
        Ok(types::SourceText::try_from(String::new()).expect(\"d3a1b7c9\"))
    }
    fn get(self) -> String {
        String::new()
    }
}
impl AsRef<str> for Helper {
    fn as_ref(&self) -> &str {
        \"\"
    }
}
",
    )
    .expect("3a9d7e2c");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::HelperRawTextReturnVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 4, "7b2e41a0 {ers:#?}");
    assert!(
        ers.iter()
            .any(|er| er.contains("function `direct` return type uses `String`")),
        "08d4b6ea {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|er| er.contains("function `list` return type uses `Vec<String>`")),
        "ae71c3f4 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|er| er.contains("function `optional` return type uses `Option<&str>`")),
        "59f0bca8 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|er| er.contains("method `nested` return type uses `String`")),
        "c46d8e10 {ers:#?}"
    );
}
#[test]
fn external_leaf_tuple_wrappers_include_crate_name() {
    let repo_crates = super::workspace_crate_names();
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("b93d2a8c"),
        super::types::SourceTextRef::from(
            "tuple wrappers over external types must include the external crate name:",
        ),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::ExternalLeafWrapperNameVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    repo_crates: super::types::StdStdSourceTextSetRef::from(repo_crates.as_ref()),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
