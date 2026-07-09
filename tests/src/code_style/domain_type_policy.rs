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
